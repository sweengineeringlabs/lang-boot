//! CSV serialization and parsing helpers

use crate::error::SerializationResult;
use serde::{de::DeserializeOwned, Serialize};

/// Parse CSV content into typed records
///
/// # Example
/// ```no_run
/// use dev_engineeringlabs_rustboot_serialization::csv::from_csv;
/// use serde::Deserialize;
///
/// #[derive(Deserialize)]
/// struct Record {
///     name: String,
///     age: u32,
/// }
///
/// let csv_data = "name,age\nAlice,30\nBob,25";
/// let records: Vec<Record> = from_csv(csv_data).unwrap();
/// ```
pub fn from_csv<T: DeserializeOwned>(content: &str) -> SerializationResult<Vec<T>> {
    let mut reader = csv::Reader::from_reader(content.as_bytes());
    let mut records = Vec::new();

    for result in reader.deserialize() {
        records.push(result?);
    }

    Ok(records)
}

/// Serialize records to CSV string
///
/// # Example
/// ```no_run
/// use dev_engineeringlabs_rustboot_serialization::csv::to_csv;
/// use serde::Serialize;
///
/// #[derive(Serialize)]
/// struct Record {
///     name: String,
///     age: u32,
/// }
///
/// let records = vec![
///     Record { name: "Alice".to_string(), age: 30 },
///     Record { name: "Bob".to_string(), age: 25 },
/// ];
///
/// let csv = to_csv(&records).unwrap();
/// ```
pub fn to_csv<T: Serialize>(records: &[T]) -> SerializationResult<String> {
    let mut writer = csv::Writer::from_writer(Vec::new());

    for record in records {
        writer.serialize(record)?;
    }

    let bytes = writer.into_inner().map_err(|e| {
        csv::Error::from(std::io::Error::new(std::io::ErrorKind::Other, e.to_string()))
    })?;
    Ok(String::from_utf8(bytes)?)
}

/// Parse CSV content with custom delimiter
pub fn from_csv_with_delimiter<T: DeserializeOwned>(
    content: &str,
    delimiter: u8,
) -> SerializationResult<Vec<T>> {
    let mut reader = csv::ReaderBuilder::new()
        .delimiter(delimiter)
        .from_reader(content.as_bytes());

    let mut records = Vec::new();
    for result in reader.deserialize() {
        records.push(result?);
    }

    Ok(records)
}

#[cfg(test)]
mod tests {
    use super::*;
    use serde::{Deserialize, Serialize};

    #[derive(Debug, Serialize, Deserialize, PartialEq)]
    struct Person {
        name: String,
        age: u32,
        active: bool,
    }

    #[test]
    fn test_csv_roundtrip() {
        let data = vec![
            Person {
                name: "Alice".to_string(),
                age: 30,
                active: true,
            },
            Person {
                name: "Bob".to_string(),
                age: 25,
                active: false,
            },
        ];

        let csv = to_csv(&data).unwrap();
        let decoded: Vec<Person> = from_csv(&csv).unwrap();

        assert_eq!(data, decoded);
    }

    #[test]
    fn test_csv_custom_delimiter() {
        let data = "name;age;active\nAlice;30;true\nBob;25;false";
        let records: Vec<Person> = from_csv_with_delimiter(data, b';').unwrap();

        assert_eq!(records.len(), 2);
        assert_eq!(records[0].name, "Alice");
        assert_eq!(records[1].age, 25);
    }
}
