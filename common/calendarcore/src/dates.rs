use crate::error::AppError;
use base64::{Engine as _, engine::general_purpose};
use jiff::{Span, civil::Date as JiffDate};

#[cfg(target_family = "wasm")]
use wasm_bindgen::prelude::*;

fn generate_ad_dates(start: Date, end: Date) -> Result<(), AppError> {
    Ok(())
}

fn generate_bs_dates(start: Date, end: Date) -> Result<(), AppError> {
    Ok(())
}

fn ad_to_bs(date: Date) -> Result<(), AppError> {
    Ok(())
}

fn bs_to_ad(date: Date) -> Result<(), AppError> {
    Ok(())
}

#[cfg_attr(not(target_family = "wasm"), derive(uniffi::Enum))]
#[cfg_attr(target_family = "wasm", wasm_bindgen)]
pub enum CalendarFormat {
    BS,
    AD,
}

#[cfg_attr(target_family = "wasm", wasm_bindgen)]
impl Date {
    #[cfg_attr(any(target_os = "ios", target_os = "android"), uniffi::constructor)]
    pub fn new(year: i16, month: i8, day: i8) -> Self {
        Date { year, month, day }
    }
}

#[cfg_attr(any(target_os = "ios", target_os = "android"), uniffi::export)]
#[cfg_attr(target_family = "wasm", wasm_bindgen)]
pub fn convert_date_format(
    date: Date,
    from_format: CalendarFormat,
    to_format: CalendarFormat,
) -> Result<(), AppError> {
    match (from_format, to_format) {
        (CalendarFormat::BS, CalendarFormat::AD) => bs_to_ad(date),
        (CalendarFormat::AD, CalendarFormat::BS) => ad_to_bs(date),
        _ => Ok(()),
    }
}

#[cfg_attr(any(target_os = "ios", target_os = "android"), uniffi::export)]
#[cfg_attr(target_family = "wasm", wasm_bindgen)]
pub fn generate_dates(start: Date, end: Date, format: CalendarFormat) -> Result<(), AppError> {
    match format {
        CalendarFormat::BS => generate_bs_dates(start, end),
        CalendarFormat::AD => generate_ad_dates(start, end),
    }
}

#[cfg_attr(any(target_os = "ios", target_os = "android"), derive(uniffi::Record))]
#[cfg_attr(target_family = "wasm", wasm_bindgen)]
#[derive(Clone, Debug)]
pub struct Date {
    pub year: i16,
    pub month: i8,
    pub day: i8,
}

mod test {

    use crate::dates::*;

    #[test]
    fn test_generate_dates() {
        let start = Date::new(2082, 1, 1);
        let end = Date::new(2082, 12, 31);

        let dates = generate_bs_dates(start, end).unwrap();
        // assert!(dates.len() == 366);

        let start = Date::new(2080, 1, 1);
        let end = Date::new(2080, 12, 30);

        let dates = generate_bs_dates(start, end).unwrap();
        // assert!(dates.len() == 365);

        let bs_date = Date::new(2082, 3, 32);
        let ad_date = bs_to_ad(bs_date).unwrap();
        // assert!(ad_date.year == 2025 && ad_date.month == 7 && ad_date.day == 16);
    }
}
