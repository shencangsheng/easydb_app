use super::excel::{excel_cell_to_timestamp_nanos, infer_cell_data_type};
use calamine::{Data, ExcelDateTime, ExcelDateTimeType};
use chrono::{DateTime, NaiveDate, NaiveDateTime, Utc};
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
fn test_non_datetime_cells_return_none() {
    assert_eq!(excel_cell_to_timestamp_nanos(&Data::Empty), None);
    assert_eq!(
        excel_cell_to_timestamp_nanos(&Data::String("hello".to_string())),
        None
    );
    assert_eq!(excel_cell_to_timestamp_nanos(&Data::Int(42)), None);
    assert_eq!(excel_cell_to_timestamp_nanos(&Data::Float(3.14)), None);
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
    assert_eq!(infer_cell_data_type(&Data::Float(3.14)), DataType::Float64);
    // Whole-number floats are treated as integers.
    assert_eq!(infer_cell_data_type(&Data::Float(7.0)), DataType::Int64);
    assert_eq!(
        infer_cell_data_type(&Data::String("hi".to_string())),
        DataType::Utf8
    );
}
