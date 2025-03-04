use std::error::Error;
use std::fs::File;
use std::path::Path;
use std::collections::HashMap;
use dbase::{FieldValue, Record};
use csv::WriterBuilder;

/// Convert a FieldValue to a String representation
fn field_value_to_string(value: &FieldValue) -> String {
    match value {
        FieldValue::Character(Some(string)) => string.clone(),
        FieldValue::Numeric(Some(num)) => num.to_string(),
        FieldValue::Numeric(None) => String::new(),
        FieldValue::Character(None) => String::new(),
        FieldValue::Date(None) => String::new(),
        FieldValue::Date(Some(date)) => date.to_string(),
        _ => String::new(), // Handle other types as needed
    }
}

/// Extract headers from the first record
fn extract_headers(first_record: Record) -> Vec<String> {
    let mut headers: Vec<String> = Vec::new();
    
    for (name, _) in first_record {
        headers.push(name.to_string());
    }
    
    headers
}

/// Convert a single record to a CSV row based on headers
fn record_to_csv_row(record: &Record, headers: &[String]) -> Vec<String> {
    // Convert record to a HashMap for easy lookup
    let record_map: HashMap<String, FieldValue> = record.clone().into_iter().collect();
    
    // Write values in the order of headers
    headers.iter()
        .map(|header| {
            record_map.get(header)
                .map(field_value_to_string)
                .unwrap_or_default()
        })
        .collect()
}

/// Convert DBF file to CSV
fn convert_dbf_to_csv(input_path: &Path, output_path: &Path) -> Result<(), Box<dyn Error>> {
    // Read the DBF file
    let records = dbase::read(input_path)?;
    
    // Validate records exist
    if records.is_empty() {
        return Err("No records found in the DBF file".into());
    }
    
    // Create a CSV writer
    let file = File::create(output_path)?;
    let mut csv_writer = WriterBuilder::new().from_writer(file);
    
    // Extract headers from first record
    let headers = extract_headers(records[0].clone());
    
    // Write headers
    csv_writer.write_record(&headers)?;
    
    // Write records to CSV
    for record in &records {
        let csv_row = record_to_csv_row(record, &headers);
        csv_writer.write_record(&csv_row)?;
    }
    
    // Ensure all data is written and flushed
    csv_writer.flush()?;
    
    Ok(())
}

fn main() -> Result<(), Box<dyn Error>> {
    for entry in std::fs::read_dir(Path::new("."))? {
        let entry = entry?;
        let path = entry.path();
        
        if path.is_file() && 
           path.extension().map_or(false, |ext| ext.eq_ignore_ascii_case("dbf")) {
            
            let filename = path.file_name()
                .and_then(|n| n.to_str())
                .map(|n| format!("{}.csv", n))
                .unwrap_or_else(|| "output.csv".to_string());
            
            // Convert file
            match convert_dbf_to_csv(&path, Path::new(&filename)) {
                Ok(_) => println!("Converted {:?} to {:?}", path, filename),
                Err(e) => eprintln!("Error converting {:?}: {}", path, e),
            }
        }
    }

    Ok(())
}