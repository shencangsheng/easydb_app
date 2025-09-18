use crate::context::error::AppError;
use crate::context::schema::AppResult;
use crate::utils::file_utils::find_files;
use calamine::{open_workbook, Data, DataType, HeaderRow, Range, Reader, Xlsx};
use polars::datatypes::{DataType as PLDataType, Field, InitHashMaps, PlHashSet, PlSmallStr};
use polars::frame::DataFrame;
use polars::prelude::schema_inference::finish_infer_field_schema;
use polars::prelude::{AnyValue, Column, PlPath};

pub struct ExcelReadOptions {}

pub struct ExcelParseOptions {}

pub struct ExcelReader {
    path: PlPath,
    sheet_name: String,
    infer_schema_length: usize,
    try_parse_dates: bool,
}

impl ExcelReader {
    pub fn new(path: PlPath) -> Self {
        Self {
            path,
            sheet_name: "Sheet1".to_string(),
            infer_schema_length: 100,
            try_parse_dates: false,
        }
    }

    pub fn with_sheet_name(mut self, sheet_name: String) -> Self {
        self.sheet_name = sheet_name;
        self
    }

    pub fn with_infer_schema_length(mut self, infer_schema_length: usize) -> Self {
        self.infer_schema_length = infer_schema_length;
        self
    }

    pub fn finish(self) -> AppResult<DataFrame> {
        let mut schemas: Option<Vec<Field>> = None;
        let mut records: Vec<Vec<AnyValue>> = Vec::new();

        let files = find_files(self.path.to_str())?;
        for file in files {
            let mut xlsx: Xlsx<_> = open_workbook(file)?;
            let range = xlsx
                .with_header_row(HeaderRow::Row(0))
                .worksheet_range(&self.sheet_name)?;

            if schemas.is_none() {
                let inferred = infer_field_schema(&range, self.infer_schema_length)?;
                records = vec![Vec::new(); inferred.len()];
                schemas = Some(inferred);
            }

            if let Some(schemas) = &schemas {
                for row in range.rows().skip(1) {
                    for (i, cell) in row.iter().enumerate() {
                        if let Some(field) = schemas.get(i) {
                            let value = match cell {
                                Data::Empty => AnyValue::Null,
                                _ => match field.dtype {
                                    PLDataType::String => {
                                        AnyValue::StringOwned(PlSmallStr::from(cell.to_string()))
                                    }
                                    PLDataType::Int64 => {
                                        cell.as_i64().map(AnyValue::Int64).unwrap_or(AnyValue::Null)
                                    }
                                    PLDataType::Float64 => cell
                                        .as_f64()
                                        .map(AnyValue::Float64)
                                        .unwrap_or(AnyValue::Null),
                                    PLDataType::Time => {
                                        cell.as_i64().map(AnyValue::Time).unwrap_or(AnyValue::Null)
                                    }
                                    _ => AnyValue::Null,
                                },
                            };
                            records[i].push(value);
                        }
                    }
                }
            }
        }

        match schemas {
            Some(schemas) => {
                let columns: Vec<Column> = schemas
                    .iter()
                    .enumerate()
                    .map(|(i, field)| Column::new(field.name.clone(), &records[i]))
                    .collect();
                Ok(DataFrame::new(columns)?)
            }
            None => Err(AppError::BadRequest {
                message: "Header not found".to_string(),
            }),
        }
    }
}

impl Default for ExcelParseOptions {
    fn default() -> Self {
        Self {}
    }
}

pub fn infer_field_schema(
    range: &Range<Data>,
    infer_schema_length: usize,
) -> AppResult<Vec<Field>> {
    if range.headers().is_none() {
        return if let Some(rows) = range.rows().next() {
            Ok(rows
                .iter()
                .enumerate()
                .map(|(i, cell)| {
                    Field::new(format!("t{}", i + 1).into(), convert_arrow_data_type(cell))
                })
                .collect())
        } else {
            return Err(AppError::BadRequest {
                message: "Header not found".to_string(),
            });
        };
    }

    let headers = range.headers().unwrap();

    let mut data_types: Vec<PlHashSet<PLDataType>> =
        vec![PlHashSet::with_capacity(5); headers.len()];

    for row in range.rows().take(infer_schema_length) {
        for (i, cell) in row.iter().enumerate() {
            if !matches!(cell, Data::Empty) {
                if let Some(types) = data_types.get_mut(i) {
                    let s = cell.to_string();
                    types.insert(polars::prelude::schema_inference::infer_field_schema(
                        &s, false, false,
                    ));
                }
            }
        }
    }

    if data_types.is_empty() {
        return Ok(headers
            .iter()
            .map(|header| Field::new(header.into(), PLDataType::String))
            .collect());
    }

    let fields: Vec<Field> = data_types
        .iter()
        .enumerate()
        .map(|(i, types)| {
            Field::new(
                headers.get(i).unwrap().into(),
                finish_infer_field_schema(types),
            )
        })
        .collect();

    Ok(fields)
}

fn convert_arrow_data_type(cell: &Data) -> PLDataType {
    match cell {
        Data::String(_) => PLDataType::String,
        Data::Float(_) => PLDataType::Float64,
        Data::Int(_) => PLDataType::Int64,
        Data::DateTime(_) => PLDataType::Time,
        _ => PLDataType::String,
    }
}
