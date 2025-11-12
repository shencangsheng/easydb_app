use crate::context::error::AppError;
use crate::context::schema::AppResult;
use crate::utils::file_utils::find_files;
use arrow_array::{Float64Array, Int64Array, RecordBatch, StringArray, TimestampNanosecondArray};
use arrow_schema::{DataType, Field, Schema, TimeUnit};
use calamine::{open_workbook, Data, HeaderRow, Range, Reader, Xlsx};
use chrono::{DateTime, NaiveDateTime, Utc};
use std::collections::HashSet;
use std::sync::Arc;

pub struct ExcelReadOptions {}

pub struct ExcelParseOptions {}

pub struct ExcelReader {
    path: String,
    sheet_name: String,
    infer_schema_length: usize,
    try_parse_dates: bool,
}

impl ExcelReader {
    pub fn new(path: String) -> Self {
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

    pub fn finish(self) -> AppResult<RecordBatch> {
        let mut schema: Option<Schema> = None;
        let mut string_data: Vec<Vec<Option<String>>> = Vec::new();
        let mut int64_data: Vec<Vec<Option<i64>>> = Vec::new();
        let mut float64_data: Vec<Vec<Option<f64>>> = Vec::new();
        let mut timestamp_data: Vec<Vec<Option<i64>>> = Vec::new();

        let files = find_files(&self.path)?;
        for file in files {
            let mut xlsx: Xlsx<_> = open_workbook(file)?;
            let range = xlsx
                .with_header_row(HeaderRow::Row(0))
                .worksheet_range(&self.sheet_name)?;

            if schema.is_none() {
                let inferred = infer_field_schema(&range, self.infer_schema_length)?;
                let num_columns = inferred.fields().len();
                string_data = vec![Vec::new(); num_columns];
                int64_data = vec![Vec::new(); num_columns];
                float64_data = vec![Vec::new(); num_columns];
                timestamp_data = vec![Vec::new(); num_columns];
                schema = Some(inferred);
            }

            if let Some(schema) = &schema {
                for row in range.rows().skip(1) {
                    for (i, cell) in row.iter().enumerate() {
                        if let Some(field) = schema.fields().get(i) {
                            match field.data_type() {
                                DataType::Int64 => {
                                    let value = match cell {
                                        Data::Int(v) => Some(*v as i64),
                                        Data::Float(v) => Some(*v as i64),
                                        _ => None,
                                    };
                                    int64_data[i].push(value);
                                }
                                DataType::Float64 => {
                                    let value = match cell {
                                        Data::Float(v) => Some(*v),
                                        Data::Int(v) => Some(*v as f64),
                                        _ => None,
                                    };
                                    float64_data[i].push(value);
                                }
                                DataType::Timestamp(TimeUnit::Nanosecond, _) => {
                                    let value = match cell {
                                        Data::DateTime(dt) => {
                                            // ExcelDateTime with "dates" feature: convert via chrono
                                            // ExcelDateTime in calamine 0.30.1 with dates feature is a newtype
                                            // We can convert it by using the underlying chrono::NaiveDateTime
                                            // Try accessing via Deref or using a conversion method
                                            let dt_str = dt.to_string();
                                            NaiveDateTime::parse_from_str(
                                                &dt_str,
                                                "%Y-%m-%d %H:%M:%S",
                                            )
                                            .ok()
                                            .or_else(|| {
                                                NaiveDateTime::parse_from_str(&dt_str, "%Y-%m-%d")
                                                    .ok()
                                            })
                                            .map(|naive_dt| {
                                                let dt_utc =
                                                    DateTime::<Utc>::from_naive_utc_and_offset(
                                                        naive_dt, Utc,
                                                    );
                                                dt_utc.timestamp_nanos_opt()
                                            })
                                            .flatten()
                                        }
                                        _ => None,
                                    };
                                    timestamp_data[i].push(value);
                                }
                                _ => {
                                    let value = match cell {
                                        Data::Empty => None,
                                        Data::String(s) => Some(s.clone()),
                                        _ => Some(cell.to_string()),
                                    };
                                    string_data[i].push(value);
                                }
                            }
                        }
                    }
                }
            }
        }

        match schema {
            Some(schema) => {
                let num_rows = string_data
                    .first()
                    .map(|v| v.len())
                    .or_else(|| int64_data.first().map(|v| v.len()))
                    .or_else(|| float64_data.first().map(|v| v.len()))
                    .or_else(|| timestamp_data.first().map(|v| v.len()))
                    .unwrap_or(0);

                let mut arrays = Vec::new();
                for (i, field) in schema.fields().iter().enumerate() {
                    let array: Arc<dyn arrow_array::Array> = match field.data_type() {
                        DataType::Int64 => Arc::new(Int64Array::from(int64_data[i].clone())),
                        DataType::Float64 => Arc::new(Float64Array::from(float64_data[i].clone())),
                        DataType::Timestamp(TimeUnit::Nanosecond, _) => {
                            Arc::new(TimestampNanosecondArray::from(timestamp_data[i].clone()))
                        }
                        _ => Arc::new(StringArray::from(string_data[i].clone())),
                    };
                    arrays.push(array);
                }

                let record_batch = RecordBatch::try_new(Arc::new(schema), arrays)?;
                Ok(record_batch)
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

pub fn infer_field_schema(range: &Range<Data>, infer_schema_length: usize) -> AppResult<Schema> {
    let headers: Vec<String> = if range.headers().is_none() {
        if let Some(rows) = range.rows().next() {
            rows.iter()
                .enumerate()
                .map(|(i, _)| format!("t{}", i + 1))
                .collect()
        } else {
            return Err(AppError::BadRequest {
                message: "Header not found".to_string(),
            });
        }
    } else {
        range
            .headers()
            .unwrap()
            .iter()
            .map(|h| h.to_string())
            .collect()
    };

    let num_columns = headers.len();
    let mut data_types: Vec<HashSet<DataType>> = vec![HashSet::new(); num_columns];

    for row in range.rows().take(infer_schema_length) {
        for (i, cell) in row.iter().enumerate() {
            if i < num_columns && !matches!(cell, Data::Empty) {
                let inferred_type = infer_cell_data_type(cell);
                data_types[i].insert(inferred_type);
            }
        }
    }

    let fields: Vec<Field> = data_types
        .iter()
        .enumerate()
        .map(|(i, types)| {
            let data_type = if types.is_empty() {
                DataType::Utf8
            } else if types.contains(&DataType::Int64) {
                DataType::Int64
            } else if types.contains(&DataType::Float64) {
                DataType::Float64
            } else if types.contains(&DataType::Timestamp(TimeUnit::Nanosecond, None)) {
                DataType::Timestamp(TimeUnit::Nanosecond, None)
            } else {
                DataType::Utf8
            };
            Field::new(headers[i].clone(), data_type, true)
        })
        .collect();

    Ok(Schema::new(fields))
}

fn infer_cell_data_type(cell: &Data) -> DataType {
    match cell {
        Data::Int(_) => DataType::Int64,
        Data::Float(_) => DataType::Float64,
        Data::DateTime(_) => DataType::Timestamp(TimeUnit::Nanosecond, None),
        _ => DataType::Utf8,
    }
}

fn convert_arrow_data_type(cell: &Data) -> DataType {
    match cell {
        Data::String(_) => DataType::Utf8,
        Data::Float(_) => DataType::Float64,
        Data::Int(_) => DataType::Int64,
        Data::DateTime(_) => DataType::Timestamp(TimeUnit::Nanosecond, None),
        _ => DataType::Utf8,
    }
}
