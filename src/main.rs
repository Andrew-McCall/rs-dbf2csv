use std::error::Error;
use std::fs::File;
use std::path::Path;
use std::collections::HashMap;
use dbase::{FieldValue, Record};
use csv::WriterBuilder;

fn field_value_to_string(value: &FieldValue) -> String {
    match value {
        dbase::FieldValue::Character(opt) => opt.clone().unwrap_or_default(),
        dbase::FieldValue::Numeric(opt) => opt.map_or_else(String::new, |n| n.to_string()),
        dbase::FieldValue::Date(opt) => opt.map_or_else(String::new, |d| d.to_unix_days().to_string()),
        dbase::FieldValue::Logical(opt) => opt.map_or_else(String::new, |b: bool| match b {true=>"1".to_string(),false=>"0".to_string()}),
        dbase::FieldValue::Memo(m) =>  m.to_string(),
        dbase::FieldValue::Float(opt) => opt.map_or_else(String::new, |f| f.to_string()),
        dbase::FieldValue::DateTime(opt) => opt.to_unix_timestamp().to_string(),
        _ => "ERR".to_string(),
    }
}    

fn extract_headers(first_record: Record) -> Vec<String> {
    let mut headers: Vec<String> = Vec::new();
    
    for (name, _) in first_record {
        headers.push(name.to_string());
    }
    
    headers
}

fn record_to_csv_row(record: &Record, headers: &[String]) -> Vec<String> {
    let record_map: HashMap<String, FieldValue> = record.clone().into_iter().collect();
    
    headers.iter()
        .map(|header| {
            record_map.get(header)
                .map(field_value_to_string)
                .unwrap_or_default()
        })
        .collect()
}

fn convert_dbf_to_csv(input_path: &Path, output_path: &Path) -> Result<(), Box<dyn Error>> {
    let mut memo_path = input_path.to_path_buf();
    memo_path.set_extension("FPT"); 

    let dbf_file = File::open(input_path).unwrap();

    let options = dbase::ReadingOptions::default()
    .character_trim(dbase::TrimOption::BeginEnd);
    
    let mut reader;

    if memo_path.exists() {
        reader = dbase::ReaderBuilder::new(dbf_file)
        .with_options(options)
        .with_encoding(dbase::encoding::UnicodeLossy)
        .with_memo( File::open(memo_path).unwrap())
        .build()
        .unwrap();
    }else{
        reader = dbase::ReaderBuilder::new(dbf_file)
        .with_options(options)
        .with_encoding(dbase::encoding::UnicodeLossy)
        .build()
        .unwrap();
    }

    let records = reader.read()?;
    
    if records.is_empty() {
        return Err("No records found in the DBF file".into());
    }
    
    let file = File::create(output_path)?;
    let mut csv_writer = WriterBuilder::new().from_writer(file);
    
    let headers = extract_headers(records[0].clone());

    csv_writer.write_record(&headers)?;
    
    for record in &records {
        let csv_row = record_to_csv_row(record, &headers);
        csv_writer.write_record(&csv_row)?;
    }
    
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
                .map(|n| format!("{}.csv", &n[..n.len()-4]))
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