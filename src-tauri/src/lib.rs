use serde::{Deserialize, Serialize};
use std::fs;
use std::path::{Path, PathBuf};
use tauri::{AppHandle, Manager, path::BaseDirectory};
use umya_spreadsheet::{ShowColumn, ShowStripes, TableStyleInfo};

const SHEET_NAME: &str = "Sign in Sheet";
const TEMPLATE_RESOURCE: &str = "resources/roster_template.xlsx";
const DATABASE_FOLDER: &str = "Personnel Management System";
const DATABASE_FILENAME: &str = "database.xlsx";
const FIRST_DATA_ROW: u32 = 7;
const LAST_DATA_ROW: u32 = 68;

const ALLOWED_RANKS: &[&str] = &[
    "Major",
    "Captain",
    "First Lieutenant",
    "Second Lieutenant",
    "Warrant Officer First Class",
    "Warrant Officer",
    "Staff Sergeant",
    "Sergeant",
    "Corporal First Class",
    "Corporal",
    "Lance Corporal",
    "Private First Class",
    "Private",
];
const ALLOWED_UNITS: &[&str] = &[
    "Land Forces",
    "National Guard",
    "Presidential Guard",
    "Navy",
    "Joint Aviation Command",
];
const ALLOWED_COURSES: &[&str] = &[
    "Master Gunners Course",
    "Master Armors Course",
    "Instructor",
];

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
struct PersonnelInput {
    full_name: String,
    rank: String,
    military_id: String,
    unit: String,
    current_date: String,
    course: String,
}

#[derive(Debug)]
struct PersonnelRecord {
    full_name: String,
    rank: String,
    military_id: String,
    unit: String,
    current_date: String,
    course: String,
}

#[derive(Debug, Serialize)]
#[serde(rename_all = "camelCase")]
struct SaveResult {
    roster_number: u32,
    excel_row: u32,
}

impl PersonnelInput {
    fn validate(self) -> Result<PersonnelRecord, String> {
        let full_name = validate_text(&self.full_name, "Full name", 100)?;
        let rank = validate_choice(&self.rank, "Rank", ALLOWED_RANKS)?;
        let military_id = validate_text(&self.military_id, "Military ID", 40)?;
        let unit = validate_choice(&self.unit, "Unit", ALLOWED_UNITS)?;
        let current_date = validate_text(&self.current_date, "Date", 10)?;
        let course = validate_choice(&self.course, "Course", ALLOWED_COURSES)?;

        if !is_valid_iso_date(&current_date) {
            return Err("Date must use the YYYY-MM-DD format.".to_string());
        }

        Ok(PersonnelRecord {
            full_name,
            rank,
            military_id,
            unit,
            current_date,
            course,
        })
    }
}

fn validate_text(value: &str, field_name: &str, maximum_length: usize) -> Result<String, String> {
    let value = value.trim();

    if value.is_empty() {
        return Err(format!("{field_name} is required."));
    }

    if value.chars().count() > maximum_length {
        return Err(format!(
            "{field_name} cannot exceed {maximum_length} characters."
        ));
    }

    if value.chars().any(char::is_control) {
        return Err(format!("{field_name} contains unsupported characters."));
    }

    if matches!(value.chars().next(), Some('=' | '+' | '-' | '@')) {
        return Err(format!(
            "{field_name} cannot begin with a spreadsheet formula character."
        ));
    }

    Ok(value.to_string())
}

fn validate_choice(value: &str, field_name: &str, allowed: &[&str]) -> Result<String, String> {
    let value = validate_text(value, field_name, 64)?;

    if allowed.contains(&value.as_str()) {
        Ok(value)
    } else {
        Err(format!("Select a valid {field_name}."))
    }
}

#[allow(clippy::manual_is_multiple_of)] // Keep compatibility with the declared Rust 1.85 MSRV.
fn is_valid_iso_date(value: &str) -> bool {
    if value.len() != 10 {
        return false;
    }

    let mut parts = value.split('-');
    let (Some(year), Some(month), Some(day), None) =
        (parts.next(), parts.next(), parts.next(), parts.next())
    else {
        return false;
    };

    if year.len() != 4 || month.len() != 2 || day.len() != 2 {
        return false;
    }

    let (Ok(year), Ok(month), Ok(day)) = (
        year.parse::<u32>(),
        month.parse::<u32>(),
        day.parse::<u32>(),
    ) else {
        return false;
    };

    let days_in_month = match month {
        1 | 3 | 5 | 7 | 8 | 10 | 12 => 31,
        4 | 6 | 9 | 11 => 30,
        2 if year % 400 == 0 || (year % 4 == 0 && year % 100 != 0) => 29,
        2 => 28,
        _ => return false,
    };

    (1..=days_in_month).contains(&day)
}

fn get_database_path(app: &AppHandle) -> Result<PathBuf, String> {
    let documents_dir = app
        .path()
        .document_dir()
        .map_err(|error| format!("Could not locate the Documents folder: {error}"))?;
    let app_folder = documents_dir.join(DATABASE_FOLDER);

    fs::create_dir_all(&app_folder)
        .map_err(|error| format!("Could not create the application data folder: {error}"))?;

    Ok(app_folder.join(DATABASE_FILENAME))
}

fn create_database_from_template(app: &AppHandle, database_path: &Path) -> Result<(), String> {
    let template_path = app
        .path()
        .resolve(TEMPLATE_RESOURCE, BaseDirectory::Resource)
        .map_err(|error| format!("Could not resolve the roster template: {error}"))?;

    if !template_path.exists() {
        return Err("The bundled roster template could not be found.".to_string());
    }

    fs::copy(template_path, database_path)
        .map_err(|error| format!("Could not create the roster workbook: {error}"))?;

    Ok(())
}

fn find_first_empty_row(worksheet: &umya_spreadsheet::Worksheet) -> Result<u32, String> {
    for row in FIRST_DATA_ROW..=LAST_DATA_ROW {
        let name_cell = format!("B{row}");

        if worksheet.value(name_cell.as_str()).trim().is_empty() {
            return Ok(row);
        }
    }

    Err(format!(
        "The roster is full. All {} available entries are in use.",
        LAST_DATA_ROW - FIRST_DATA_ROW + 1
    ))
}

fn set_text_preserving_style(
    worksheet: &mut umya_spreadsheet::Worksheet,
    coordinate: String,
    value: &str,
) {
    let style = worksheet.style(coordinate.as_str()).clone();
    worksheet.cell_mut(coordinate.as_str()).set_value(value);
    worksheet.set_style(coordinate.as_str(), style);
}

fn set_number_preserving_style(
    worksheet: &mut umya_spreadsheet::Worksheet,
    coordinate: String,
    value: u32,
) {
    let style = worksheet.style(coordinate.as_str()).clone();
    worksheet
        .cell_mut(coordinate.as_str())
        .set_value_number(value);
    worksheet.set_style(coordinate.as_str(), style);
}

fn restore_table_row_stripes(worksheet: &mut umya_spreadsheet::Worksheet) {
    for table in worksheet.tables_mut() {
        table.set_style_info(Some(TableStyleInfo::new(
            "TableStyleMedium2",
            ShowColumn::Hide,
            ShowColumn::Hide,
            ShowStripes::Show,
            ShowStripes::Hide,
        )));
    }
}

fn write_personnel_row(
    worksheet: &mut umya_spreadsheet::Worksheet,
    row: u32,
    data: &PersonnelRecord,
) {
    let roster_number = row - FIRST_DATA_ROW + 1;

    set_number_preserving_style(worksheet, format!("A{row}"), roster_number);
    set_text_preserving_style(worksheet, format!("B{row}"), &data.full_name);
    set_text_preserving_style(worksheet, format!("C{row}"), &data.rank);
    set_text_preserving_style(worksheet, format!("D{row}"), &data.military_id);
    set_text_preserving_style(worksheet, format!("E{row}"), &data.unit);
    set_text_preserving_style(worksheet, format!("F{row}"), &data.current_date);
    set_text_preserving_style(worksheet, format!("G{row}"), &data.course);
}

fn save_to_excel(app: &AppHandle, data: &PersonnelRecord) -> Result<u32, String> {
    let database_path = get_database_path(app)?;

    if !database_path.exists() {
        create_database_from_template(app, &database_path)?;
    }

    let mut workbook = umya_spreadsheet::reader::xlsx::read(&database_path)
        .map_err(|error| format!("Could not open the roster workbook: {error}"))?;

    let worksheet = workbook.sheet_by_name_mut(SHEET_NAME).map_err(|error| {
        format!("The worksheet '{SHEET_NAME}' is missing from the roster template: {error}")
    })?;

    let next_row = find_first_empty_row(worksheet)?;
    write_personnel_row(worksheet, next_row, data);
    restore_table_row_stripes(worksheet);

    umya_spreadsheet::writer::xlsx::write(&workbook, &database_path).map_err(|error| {
        format!("Could not save the roster workbook. Close it in Excel and try again: {error}")
    })?;

    Ok(next_row)
}

#[tauri::command]
fn save_personnel(app: AppHandle, data: PersonnelInput) -> Result<SaveResult, String> {
    let record = data.validate()?;
    let excel_row = save_to_excel(&app, &record)?;

    Ok(SaveResult {
        roster_number: excel_row - FIRST_DATA_ROW + 1,
        excel_row,
    })
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .invoke_handler(tauri::generate_handler![save_personnel])
        .run(tauri::generate_context!())
        .expect("failed to run the Class Roster application");
}

#[cfg(test)]
mod tests {
    use super::*;

    fn valid_input() -> PersonnelInput {
        PersonnelInput {
            full_name: "Alex Smith".to_string(),
            rank: "Captain".to_string(),
            military_id: "001234".to_string(),
            unit: "Land Forces".to_string(),
            current_date: "2026-07-22".to_string(),
            course: "Instructor".to_string(),
        }
    }

    #[test]
    fn accepts_a_valid_record() {
        assert!(valid_input().validate().is_ok());
    }

    #[test]
    fn rejects_an_impossible_date() {
        let mut input = valid_input();
        input.current_date = "2026-02-30".to_string();

        assert!(input.validate().is_err());
    }

    #[test]
    fn rejects_an_unknown_choice() {
        let mut input = valid_input();
        input.rank = "Unknown".to_string();

        assert!(input.validate().is_err());
    }

    #[test]
    fn rejects_spreadsheet_formula_prefixes() {
        let mut input = valid_input();
        input.full_name = "=HYPERLINK(\"https://example.com\")".to_string();

        assert!(input.validate().is_err());
    }

    #[test]
    fn bundled_template_contains_the_roster_worksheet() {
        let template_path = Path::new(env!("CARGO_MANIFEST_DIR")).join(TEMPLATE_RESOURCE);
        let workbook = umya_spreadsheet::reader::xlsx::read(&template_path)
            .expect("the bundled roster template should be readable");

        assert!(
            workbook.sheet_by_name(SHEET_NAME).is_ok(),
            "the bundled roster template should contain '{SHEET_NAME}'"
        );
    }
}
