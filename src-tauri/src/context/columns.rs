use crate::context::error::AppError;
use crate::context::schema::AppResult;
use datafusion::arrow::datatypes::{DataType, Field, Schema, TimeUnit};
use datafusion::logical_expr::{cast, col};
use datafusion::prelude::SessionContext;
use sqlparser::ast::{Expr, FunctionArg, FunctionArgExpr, Ident, TableFunctionArgs, Value};
use std::collections::{HashMap, HashSet};
use std::sync::Arc;

/// A user-requested type for one column of a `read_*` table function.
#[derive(Clone, Debug, PartialEq)]
pub(crate) struct ColumnTypeOverride {
    pub name: String,
    pub data_type: DataType,
}

/// Read `columns => {name: 'VARCHAR', id: 'BIGINT'}` from a table function.
///
/// Absent `columns` yields an empty list. The same syntax is accepted by every
/// `read_*` function.
pub(crate) fn column_overrides_from_args(
    args: &Option<TableFunctionArgs>,
) -> AppResult<Vec<ColumnTypeOverride>> {
    let Some(table_args) = args else {
        return Ok(Vec::new());
    };

    let mut found = false;
    let mut overrides = Vec::new();
    for arg in &table_args.args {
        let FunctionArg::Named { name, arg, .. } = arg else {
            continue;
        };
        if name.value != "columns" {
            continue;
        }
        if found {
            return Err(AppError::BadRequest {
                message: "columns can only be specified once".to_string(),
            });
        }
        found = true;
        overrides = parse_columns_expr(arg)?;
    }
    Ok(overrides)
}

fn parse_columns_expr(arg: &FunctionArgExpr) -> AppResult<Vec<ColumnTypeOverride>> {
    let FunctionArgExpr::Expr(Expr::Dictionary(fields)) = arg else {
        return Err(AppError::BadRequest {
            message:
                "columns must be a type map, for example columns => {name: 'VARCHAR', id: 'BIGINT'}"
                    .to_string(),
        });
    };
    if fields.is_empty() {
        return Err(AppError::BadRequest {
            message: "columns must list at least one column".to_string(),
        });
    }

    let mut seen = HashSet::new();
    let mut overrides = Vec::with_capacity(fields.len());
    for field in fields {
        let column_name = field.key.value.clone();
        if column_name.is_empty() {
            return Err(AppError::BadRequest {
                message: "columns contains an empty column name".to_string(),
            });
        }
        let key = column_name.to_ascii_lowercase();
        if !seen.insert(key) {
            return Err(AppError::BadRequest {
                message: format!("duplicate column '{column_name}' in columns"),
            });
        }
        let type_name = expr_type_name(field.value.as_ref())?;
        let data_type = sql_type_to_arrow(&type_name)?;
        overrides.push(ColumnTypeOverride {
            name: column_name,
            data_type,
        });
    }
    Ok(overrides)
}

fn expr_type_name(expr: &Expr) -> AppResult<String> {
    match expr {
        Expr::Value(Value::SingleQuotedString(value) | Value::DoubleQuotedString(value)) => {
            Ok(value.clone())
        }
        Expr::Identifier(Ident { value, .. }) => Ok(value.clone()),
        _ => Err(AppError::BadRequest {
            message: "column type must be a type name, for example 'VARCHAR' or BIGINT".to_string(),
        }),
    }
}

/// Map a SQL type name onto the Arrow type readers and casts understand.
pub(crate) fn sql_type_to_arrow(sql_type: &str) -> AppResult<DataType> {
    let trimmed = sql_type.trim();
    let base = match trimmed.split_once('(') {
        Some((name, _)) => name.trim(),
        None => trimmed,
    };
    let normalized = base.to_ascii_uppercase();
    match normalized.as_str() {
        "VARCHAR" | "CHAR" | "TEXT" | "STRING" | "UTF8" => Ok(DataType::Utf8),
        "INT" | "INTEGER" | "BIGINT" | "SMALLINT" | "TINYINT" | "INT64" => Ok(DataType::Int64),
        "FLOAT" | "DOUBLE" | "REAL" | "FLOAT64" | "DECIMAL" | "NUMERIC" => Ok(DataType::Float64),
        "BOOL" | "BOOLEAN" => Ok(DataType::Boolean),
        "DATE" => Ok(DataType::Date32),
        other if other.starts_with("TIMESTAMP") || other.starts_with("DATETIME") => {
            Ok(DataType::Timestamp(TimeUnit::Nanosecond, None))
        }
        _ => Err(AppError::BadRequest {
            message: format!(
                "unknown column type '{sql_type}'. Use VARCHAR, BIGINT, DOUBLE, BOOLEAN, TIMESTAMP, or DATE"
            ),
        }),
    }
}

/// Replace inferred types for the named columns. Other fields stay as they are.
pub(crate) fn apply_column_overrides(
    schema: &Schema,
    overrides: &[ColumnTypeOverride],
) -> AppResult<Schema> {
    if overrides.is_empty() {
        return Ok(schema.clone());
    }
    let resolved = resolve_column_overrides(schema, overrides)?;
    let fields = schema
        .fields()
        .iter()
        .map(|field| match resolved.get(field.name()) {
            Some(data_type) => Arc::new(field.as_ref().clone().with_data_type(data_type.clone())),
            None => Arc::clone(field),
        })
        .collect::<Vec<_>>();
    Ok(Schema::new_with_metadata(fields, schema.metadata().clone()))
}

/// Register a view named `table_name` that casts columns of an already
/// registered `source_name`. Used by readers whose stored types cannot be
/// chosen up front (Parquet, MySQL, PostgreSQL).
pub(crate) async fn project_column_types(
    ctx: &SessionContext,
    source_name: &str,
    table_name: &str,
    overrides: &[ColumnTypeOverride],
) -> AppResult<()> {
    let frame = ctx.table(source_name).await?;
    let arrow_schema = frame.schema().as_arrow().clone();
    let resolved = resolve_column_overrides(&arrow_schema, overrides)?;
    let exprs = arrow_schema
        .fields()
        .iter()
        .map(|field| match resolved.get(field.name()) {
            Some(data_type) => cast(col(field.name()), data_type.clone()).alias(field.name()),
            None => col(field.name()),
        })
        .collect::<Vec<_>>();
    let view = frame.select(exprs)?.into_view();
    ctx.register_table(table_name, view)?;
    Ok(())
}

fn resolve_column_overrides(
    schema: &Schema,
    overrides: &[ColumnTypeOverride],
) -> AppResult<HashMap<String, DataType>> {
    let fields: Vec<&Field> = schema.fields().iter().map(|field| field.as_ref()).collect();
    let mut resolved = HashMap::with_capacity(overrides.len());
    for column in overrides {
        let index = find_field_index(&fields, &column.name)?;
        let actual_name = fields[index].name().clone();
        resolved.insert(actual_name, column.data_type.clone());
    }
    Ok(resolved)
}

fn find_field_index(fields: &[&Field], name: &str) -> AppResult<usize> {
    if let Some(index) = fields.iter().position(|field| field.name() == name) {
        return Ok(index);
    }

    let matches = fields
        .iter()
        .enumerate()
        .filter(|(_, field)| field.name().eq_ignore_ascii_case(name))
        .map(|(index, _)| index)
        .collect::<Vec<_>>();
    match matches.as_slice() {
        [index] => Ok(*index),
        [] => {
            let available = fields
                .iter()
                .map(|field| field.name().as_str())
                .collect::<Vec<_>>()
                .join(", ");
            Err(AppError::BadRequest {
                message: format!(
                    "unknown column '{name}' in columns. Available columns: {available}"
                ),
            })
        }
        _ => Err(AppError::BadRequest {
            message: format!("column '{name}' in columns matches more than one field"),
        }),
    }
}

/// `read_excel` materializes only these physical types.
pub(crate) fn ensure_excel_column_type(name: &str, data_type: &DataType) -> AppResult<()> {
    match data_type {
        DataType::Utf8
        | DataType::LargeUtf8
        | DataType::Int64
        | DataType::Float64
        | DataType::Boolean
        | DataType::Null
        | DataType::Timestamp(_, _) => Ok(()),
        _ => Err(AppError::BadRequest {
            message: format!(
                "read_excel cannot read column '{name}' as {data_type}. Use VARCHAR, BIGINT, DOUBLE, BOOLEAN, or TIMESTAMP"
            ),
        }),
    }
}
