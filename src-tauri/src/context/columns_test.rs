use super::columns::{
    apply_column_overrides, column_overrides_from_args, project_column_types, sql_type_to_arrow,
    ColumnTypeOverride,
};
use crate::context::error::AppError;
use crate::context::schema::AppResult;
use crate::sql::parse::parse_statements;
use datafusion::arrow::array::{Array, Int64Array, StringArray};
use datafusion::arrow::datatypes::{DataType, Field, Schema, TimeUnit};
use datafusion::arrow::record_batch::RecordBatch;
use datafusion::prelude::SessionContext;
use sqlparser::ast::SetExpr::Select;
use sqlparser::ast::{Statement, TableFactor, TableFunctionArgs};
use std::sync::Arc;

fn table_args(sql: &str) -> AppResult<Option<TableFunctionArgs>> {
    let mut ast = parse_statements(sql)?;
    if ast.is_empty() {
        return Ok(None);
    }
    if let Statement::Query(query) = ast.remove(0) {
        if let Select(select) = *query.body {
            if let Some(table) = select.from.into_iter().next() {
                if let TableFactor::Table { args, .. } = table.relation {
                    return Ok(args);
                }
            }
        }
    }
    Ok(None)
}

fn override_named(name: &str, data_type: DataType) -> ColumnTypeOverride {
    ColumnTypeOverride {
        name: name.to_string(),
        data_type,
    }
}

#[test]
fn test_column_overrides_from_args_parses_type_map() -> AppResult<()> {
    let args = table_args(
        "SELECT * FROM read_ndjson('data.ndjson', columns => {control_phase: 'VARCHAR', id: BIGINT})",
    )?;
    let overrides = column_overrides_from_args(&args)?;

    assert_eq!(overrides.len(), 2);
    assert_eq!(overrides[0].name, "control_phase");
    assert_eq!(overrides[0].data_type, DataType::Utf8);
    assert_eq!(overrides[1].name, "id");
    assert_eq!(overrides[1].data_type, DataType::Int64);
    Ok(())
}

#[test]
fn test_column_overrides_from_args_parses_quoted_column_name() -> AppResult<()> {
    let args = table_args(
        "SELECT * FROM read_csv('data.csv', columns => {\"control-phase\": 'VARCHAR(32)'})",
    )?;
    let overrides = column_overrides_from_args(&args)?;

    assert_eq!(overrides.len(), 1);
    assert_eq!(overrides[0].name, "control-phase");
    assert_eq!(overrides[0].data_type, DataType::Utf8);
    Ok(())
}

#[test]
fn test_column_overrides_from_args_absent_is_empty() -> AppResult<()> {
    let args = table_args("SELECT * FROM read_csv('data.csv', infer_schema => false)")?;
    let overrides = column_overrides_from_args(&args)?;
    assert!(overrides.is_empty());
    Ok(())
}

#[test]
fn test_column_overrides_from_args_rejects_unknown_type() {
    let args = table_args("SELECT * FROM read_csv('data.csv', columns => {id: 'XML'})")
        .expect("sql parsed");
    let error = column_overrides_from_args(&args).expect_err("unknown type");
    match error {
        AppError::BadRequest { message } => assert!(message.contains("XML"), "{message}"),
        other => panic!("unexpected error: {other:?}"),
    }
}

#[test]
fn test_column_overrides_from_args_rejects_duplicate_names() {
    let args =
        table_args("SELECT * FROM read_csv('data.csv', columns => {id: 'BIGINT', ID: 'VARCHAR'})")
            .expect("sql parsed");
    let error = column_overrides_from_args(&args).expect_err("duplicate");
    match error {
        AppError::BadRequest { message } => assert!(message.contains("duplicate"), "{message}"),
        other => panic!("unexpected error: {other:?}"),
    }
}

#[test]
fn test_sql_type_to_arrow_known_names() -> AppResult<()> {
    assert_eq!(sql_type_to_arrow("text")?, DataType::Utf8);
    assert_eq!(sql_type_to_arrow("DOUBLE")?, DataType::Float64);
    assert_eq!(sql_type_to_arrow("boolean")?, DataType::Boolean);
    assert_eq!(sql_type_to_arrow("date")?, DataType::Date32);
    assert_eq!(
        sql_type_to_arrow("timestamp")?,
        DataType::Timestamp(TimeUnit::Nanosecond, None)
    );
    Ok(())
}

#[test]
fn test_apply_column_overrides_replaces_named_field_only() -> AppResult<()> {
    let schema = Schema::new(vec![
        Field::new("id", DataType::Int64, true),
        Field::new("name", DataType::Utf8, true),
    ]);
    let updated = apply_column_overrides(&schema, &[override_named("ID", DataType::Utf8)])?;

    assert_eq!(updated.field(0).name(), "id");
    assert_eq!(updated.field(0).data_type(), &DataType::Utf8);
    assert_eq!(updated.field(1).data_type(), &DataType::Utf8);
    Ok(())
}

#[test]
fn test_apply_column_overrides_unknown_column_lists_available() {
    let schema = Schema::new(vec![Field::new("id", DataType::Int64, true)]);
    let error = apply_column_overrides(&schema, &[override_named("missing", DataType::Utf8)])
        .expect_err("missing column");
    match error {
        AppError::BadRequest { message } => {
            assert!(message.contains("missing"), "{message}");
            assert!(message.contains("id"), "{message}");
        }
        other => panic!("unexpected error: {other:?}"),
    }
}

#[tokio::test]
async fn test_project_column_types_casts_registered_table() -> AppResult<()> {
    let ctx = SessionContext::new();
    let schema = Arc::new(Schema::new(vec![
        Field::new("id", DataType::Int64, true),
        Field::new("name", DataType::Utf8, true),
    ]));
    let batch = RecordBatch::try_new(
        schema,
        vec![
            Arc::new(Int64Array::from(vec![7])),
            Arc::new(StringArray::from(vec!["alice"])),
        ],
    )?;
    ctx.register_batch("src", batch)?;

    project_column_types(
        &ctx,
        "src",
        "typed",
        &[override_named("id", DataType::Utf8)],
    )
    .await?;

    let frame = ctx.table("typed").await?;
    let fields = frame.schema().as_arrow().clone();
    assert_eq!(fields.field(0).data_type(), &DataType::Utf8);
    assert_eq!(fields.field(1).data_type(), &DataType::Utf8);

    let batches = frame.collect().await?;
    let id = batches[0]
        .column(0)
        .as_any()
        .downcast_ref::<StringArray>()
        .expect("id cast to string");
    assert_eq!(id.value(0), "7");
    Ok(())
}
