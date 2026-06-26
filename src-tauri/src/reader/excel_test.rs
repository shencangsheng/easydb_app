use super::excel::{
    excel_cell_to_timestamp_nanos, infer_cell_data_type, infer_field_schema,
    resolve_column_data_type, ExcelReader,
};
use calamine::{Data, ExcelDateTime, ExcelDateTimeType, Range};
use chrono::{DateTime, NaiveDate, NaiveDateTime, Utc};
use datafusion::arrow::array::{Array, Float64Array, Int64Array, TimestampNanosecondArray};
use datafusion::arrow::datatypes::{DataType, TimeUnit};

// ═══════════════════════════════════════════════════════════════════════
// Stability Tests
// ═══════════════════════════════════════════════════════════════════════

/// Excel stores dates as a serial number counting days since 1899-12-30.
/// This builds the serial for an arbitrary `NaiveDateTime` so tests can
/// avoid hard-coding magic float values.
fn to_excel_serial(dt: NaiveDateTime) -> f64 {
    let epoch = NaiveDate::from_ymd_opt(1899, 12, 30)
        .unwrap()
        .and_hms_opt(0, 0, 0)
        .unwrap();
    (dt - epoch).num_milliseconds() as f64 / 86_400_000.0
}

fn expected_nanos(dt: NaiveDateTime) -> Option<i64> {
    DateTime::<Utc>::from_naive_utc_and_offset(dt, Utc).timestamp_nanos_opt()
}

// ─── excel_cell_to_timestamp_nanos ────────────────────────────────────

#[test]
fn test_converts_excel_datetime_cell() {
    let dt = NaiveDate::from_ymd_opt(2026, 5, 26)
        .unwrap()
        .and_hms_opt(19, 41, 0)
        .unwrap();
    let cell = Data::DateTime(ExcelDateTime::new(
        to_excel_serial(dt),
        ExcelDateTimeType::DateTime,
        false,
    ));

    assert_eq!(excel_cell_to_timestamp_nanos(&cell), expected_nanos(dt));
}

#[test]
fn test_converts_excel_date_only_cell() {
    let dt = NaiveDate::from_ymd_opt(2026, 5, 26)
        .unwrap()
        .and_hms_opt(0, 0, 0)
        .unwrap();
    let cell = Data::DateTime(ExcelDateTime::new(
        to_excel_serial(dt),
        ExcelDateTimeType::DateTime,
        false,
    ));

    assert_eq!(excel_cell_to_timestamp_nanos(&cell), expected_nanos(dt));
}

#[test]
fn test_converts_iso_datetime_string_cell() {
    let dt = NaiveDate::from_ymd_opt(2026, 5, 26)
        .unwrap()
        .and_hms_opt(19, 41, 0)
        .unwrap();

    let cell = Data::DateTimeIso("2026-05-26T19:41:00".to_string());
    assert_eq!(excel_cell_to_timestamp_nanos(&cell), expected_nanos(dt));
}

#[test]
fn test_converts_iso_date_only_string_cell() {
    let dt = NaiveDate::from_ymd_opt(2026, 5, 26)
        .unwrap()
        .and_hms_opt(0, 0, 0)
        .unwrap();

    let cell = Data::DateTimeIso("2026-05-26".to_string());
    assert_eq!(excel_cell_to_timestamp_nanos(&cell), expected_nanos(dt));
}

#[test]
fn test_converts_space_separated_datetime_string_cell() {
    // Some producers emit "YYYY-MM-DD HH:MM:SS" (space) instead of the "T" form.
    let dt = NaiveDate::from_ymd_opt(2026, 5, 26)
        .unwrap()
        .and_hms_opt(19, 41, 0)
        .unwrap();

    let cell = Data::DateTimeIso("2026-05-26 19:41:00".to_string());
    assert_eq!(excel_cell_to_timestamp_nanos(&cell), expected_nanos(dt));
}

#[test]
fn test_non_datetime_cells_return_none() {
    assert_eq!(excel_cell_to_timestamp_nanos(&Data::Empty), None);
    assert_eq!(
        excel_cell_to_timestamp_nanos(&Data::String("hello".to_string())),
        None
    );
    assert_eq!(excel_cell_to_timestamp_nanos(&Data::Int(42)), None);
    assert_eq!(excel_cell_to_timestamp_nanos(&Data::Float(9.87)), None);
}

// ─── infer_cell_data_type ─────────────────────────────────────────────

#[test]
fn test_infers_datetime_columns_as_timestamp() {
    let serial = to_excel_serial(
        NaiveDate::from_ymd_opt(2026, 5, 26)
            .unwrap()
            .and_hms_opt(19, 41, 0)
            .unwrap(),
    );
    let dt_cell = Data::DateTime(ExcelDateTime::new(serial, ExcelDateTimeType::DateTime, false));

    assert_eq!(
        infer_cell_data_type(&dt_cell),
        DataType::Timestamp(TimeUnit::Nanosecond, None)
    );
    assert_eq!(
        infer_cell_data_type(&Data::DateTimeIso("2026-05-26T19:41:00".to_string())),
        DataType::Timestamp(TimeUnit::Nanosecond, None)
    );
}

#[test]
fn test_infers_scalar_columns() {
    assert_eq!(infer_cell_data_type(&Data::Int(7)), DataType::Int64);
    assert_eq!(infer_cell_data_type(&Data::Float(9.87)), DataType::Float64);
    // Whole-number floats are treated as integers.
    assert_eq!(infer_cell_data_type(&Data::Float(7.0)), DataType::Int64);
    assert_eq!(
        infer_cell_data_type(&Data::String("hi".to_string())),
        DataType::Utf8
    );
}

// ─── infer_field_schema ───────────────────────────────────────────────

#[test]
fn test_infer_field_schema_resolves_column_types() {
    // 3 rows x 3 cols. Row 0 is the header (skipped during inference).
    //   col0 "mix": Int + non-whole Float  -> Float64 (any float promotes column)
    //   col1 "dt" : DateTime cells          -> Timestamp(ns)
    //   col2      : entirely empty          -> Utf8 (empty-set default)
    let mut range: Range<Data> = Range::new((0, 0), (2, 2));
    range.set_value((0, 0), Data::String("mix".to_string()));
    range.set_value((1, 0), Data::Int(1));
    range.set_value((2, 0), Data::Float(2.5));

    let serial = to_excel_serial(
        NaiveDate::from_ymd_opt(2026, 5, 26)
            .unwrap()
            .and_hms_opt(12, 0, 0)
            .unwrap(),
    );
    range.set_value((0, 1), Data::String("dt".to_string()));
    range.set_value(
        (1, 1),
        Data::DateTime(ExcelDateTime::new(serial, ExcelDateTimeType::DateTime, false)),
    );
    range.set_value(
        (2, 1),
        Data::DateTime(ExcelDateTime::new(serial, ExcelDateTimeType::DateTime, false)),
    );
    // col2 intentionally left as Data::Empty for every row.

    let schema = infer_field_schema(&range, 100).expect("schema inferred");
    let fields = schema.fields();
    assert_eq!(fields.len(), 3);

    assert_eq!(fields[0].name(), "mix");
    assert_eq!(fields[0].data_type(), &DataType::Float64);

    assert_eq!(fields[1].name(), "dt");
    assert_eq!(
        fields[1].data_type(),
        &DataType::Timestamp(TimeUnit::Nanosecond, None)
    );

    // Empty column resolves to the Utf8 default.
    assert_eq!(fields[2].data_type(), &DataType::Utf8);
}

#[test]
fn test_infer_field_schema_promotes_to_float_when_any_fractional_value() {
    // Regression: a column with mostly integers but one fractional value must
    // be Float64, not Int64 (which would truncate via `as i64` on read).
    let mut range: Range<Data> = Range::new((0, 0), (4, 0));
    range.set_value((0, 0), Data::String("amount".to_string()));
    range.set_value((1, 0), Data::Float(1.0));
    range.set_value((2, 0), Data::Float(2.0));
    range.set_value((3, 0), Data::Float(3.0));
    range.set_value((4, 0), Data::Float(2.5));

    let schema = infer_field_schema(&range, 100).expect("schema inferred");
    assert_eq!(
        schema.fields()[0].data_type(),
        &DataType::Float64,
        "one fractional cell must promote the whole column to Float64"
    );
}

#[test]
fn test_infer_field_schema_mixed_number_and_string_becomes_utf8() {
    let mut range: Range<Data> = Range::new((0, 0), (4, 0));
    range.set_value((0, 0), Data::String("score".to_string()));
    range.set_value((1, 0), Data::Int(90));
    range.set_value((2, 0), Data::Int(85));
    range.set_value((3, 0), Data::String("N/A".to_string()));
    range.set_value((4, 0), Data::Int(88));

    let schema = infer_field_schema(&range, 100).expect("schema inferred");
    assert_eq!(
        schema.fields()[0].data_type(),
        &DataType::Utf8,
        "mixed numeric and text cells must fall back to Utf8"
    );
}

#[test]
fn test_resolve_column_data_type_rules() {
    use std::collections::HashSet;

    assert_eq!(resolve_column_data_type(&HashSet::new()), DataType::Utf8);

    let mut ints = HashSet::new();
    ints.insert(DataType::Int64);
    assert_eq!(resolve_column_data_type(&ints), DataType::Int64);

    let mut mixed_numeric = HashSet::new();
    mixed_numeric.insert(DataType::Int64);
    mixed_numeric.insert(DataType::Float64);
    assert_eq!(
        resolve_column_data_type(&mixed_numeric),
        DataType::Float64,
        "any fractional value promotes pure numeric columns"
    );

    let mut mixed_text_number = HashSet::new();
    mixed_text_number.insert(DataType::Utf8);
    mixed_text_number.insert(DataType::Int64);
    assert_eq!(
        resolve_column_data_type(&mixed_text_number),
        DataType::Utf8
    );
}

#[test]
fn test_infer_field_schema_empty_range_errors() {
    // An empty range has no rows, so headers() is None and there is no first row
    // to synthesize column names from -> "Header not found".
    let range: Range<Data> = Range::empty();
    assert!(infer_field_schema(&range, 100).is_err());
}

// ─── ExcelReader::finish (full xlsx pipeline under arrow 58) ───────────

/// Write a tiny .xlsx fixture to a unique temp path and return it. Uses
/// `write_datetime_with_format` so calamine recognizes the column as a date,
/// and an exact-binary-fraction time (12:00:00 = 0.5 of a day) so the
/// nanosecond value round-trips precisely.
fn write_xlsx_fixture() -> std::path::PathBuf {
    use rust_xlsxwriter::{ExcelDateTime as XlsxDateTime, Format, Workbook};

    let mut path = std::env::temp_dir();
    let unique = format!(
        "easydb_excel_fixture_{}_{}.xlsx",
        std::process::id(),
        sample_finish_nanos()
    );
    path.push(unique);

    let mut workbook = Workbook::new();
    let sheet = workbook.add_worksheet();
    let date_fmt = Format::new().set_num_format("yyyy-mm-dd hh:mm:ss");

    for (col, name) in ["id", "price", "name", "ts"].iter().enumerate() {
        sheet
            .write_string(0, col as u16, *name)
            .expect("write header");
    }

    // Row 1
    sheet.write_number(1, 0, 1.0).expect("id");
    sheet.write_number(1, 1, 9.87).expect("price");
    sheet.write_string(1, 2, "alice").expect("name");
    let dt1 = XlsxDateTime::from_ymd(2026, 5, 26)
        .expect("date")
        .and_hms(12, 0, 0)
        .expect("time");
    sheet
        .write_datetime_with_format(1, 3, &dt1, &date_fmt)
        .expect("ts");

    // Row 2
    sheet.write_number(2, 0, 2.0).expect("id");
    sheet.write_number(2, 1, 2.5).expect("price");
    sheet.write_string(2, 2, "bob").expect("name");
    let dt2 = XlsxDateTime::from_ymd(2026, 5, 27)
        .expect("date")
        .and_hms(6, 0, 0)
        .expect("time");
    sheet
        .write_datetime_with_format(2, 3, &dt2, &date_fmt)
        .expect("ts");

    workbook.save(&path).expect("save xlsx");
    path
}

/// Expected nanos for the row-1 timestamp (2026-05-26 12:00:00 UTC).
fn sample_finish_nanos() -> i64 {
    let dt = NaiveDate::from_ymd_opt(2026, 5, 26)
        .unwrap()
        .and_hms_opt(12, 0, 0)
        .unwrap();
    DateTime::<Utc>::from_naive_utc_and_offset(dt, Utc)
        .timestamp_nanos_opt()
        .unwrap()
}

#[test]
fn test_excel_reader_finish_pipeline() {
    let path = write_xlsx_fixture();
    let path_str = path.to_str().expect("utf8 path").to_string();

    let result = ExcelReader::new(path_str).finish();

    // Best-effort cleanup regardless of assertion outcome.
    let batch = match result {
        Ok(batch) => batch,
        Err(e) => {
            let _ = std::fs::remove_file(&path);
            panic!("finish failed: {:?}", e);
        }
    };

    assert_eq!(batch.num_columns(), 4);
    assert_eq!(batch.num_rows(), 2);

    let schema = batch.schema();
    let fields = schema.fields();
    assert_eq!(fields[0].data_type(), &DataType::Int64);
    assert_eq!(fields[1].data_type(), &DataType::Float64);
    assert_eq!(fields[2].data_type(), &DataType::Utf8);
    assert_eq!(
        fields[3].data_type(),
        &DataType::Timestamp(TimeUnit::Nanosecond, None)
    );

    let ids = batch
        .column(0)
        .as_any()
        .downcast_ref::<Int64Array>()
        .expect("Int64Array");
    assert_eq!(ids.value(0), 1);
    assert_eq!(ids.value(1), 2);

    let prices = batch
        .column(1)
        .as_any()
        .downcast_ref::<Float64Array>()
        .expect("Float64Array");
    assert!((prices.value(0) - 9.87).abs() < 1e-9);

    let ts = batch
        .column(3)
        .as_any()
        .downcast_ref::<TimestampNanosecondArray>()
        .expect("TimestampNanosecondArray");
    assert_eq!(ts.value(0), sample_finish_nanos());

    let _ = std::fs::remove_file(&path);
}
