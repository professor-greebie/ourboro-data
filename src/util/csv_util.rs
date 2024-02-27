use calamine::{open_workbook, DataType, Reader, Xlsx};


pub fn xlsx_to_csv(path: &str) {
    let lines = read_xlsx(path);
    //println!("{:?}", lines);
    let csv_path = path.to_string();
    let _ = csv_path.replace(".xlsx", ".csv");
    write_csv(&csv_path, lines);
}

fn read_xlsx(path: &str) -> Vec<Vec<String>> {
    let mut result = Vec::new();
    println!("Reading file: {}", path);
    let mut workbook: Xlsx<_> = open_workbook(path).unwrap();
    for sheet in workbook.sheet_names().to_owned() {
        println!("Reading sheet: {}", sheet);
        if let Ok(r) = workbook.worksheet_range(&sheet) {
        println!("Reading sheet 1");
        for row in r.rows() {
            let mut row_result = Vec::new();
            for (_i, c) in row.iter().enumerate() {
                println!("{:?}", c);
                match c {
                    DataType::String(s) => row_result.push(s.to_string()),
                    DataType::Float(f) => row_result.push(f.to_string()),
                    DataType::Int(i) => row_result.push(i.to_string()),
                    _ => row_result.push("".to_string()),
                }
            }
            result.push(row_result);
        }
    }}
    result
}

pub fn write_csv(path: &str, lines: Vec<Vec<String>>) {
    let mut wtr = csv::Writer::from_path(path).unwrap();
    for line in lines {
        wtr.write_record(line).unwrap();
    }
    wtr.flush().unwrap();
}

pub fn head(lines: Vec<(String, String)>, take: usize) -> Vec<(String, String)> {
    let mut result = Vec::new();
    for i in 0..take {
        result.push(lines[i].clone());
    }
    result
}

pub fn split_csv_line(line: &str) -> Vec<String> {
    let mut result = Vec::new();
    let mut in_quotes = false;
    let mut current_string = String::new();
    for c in line.chars() {
        if c == '"' {
            in_quotes = !in_quotes;
        } else if c == ',' && !in_quotes {
            result.push(current_string.clone());
            current_string.clear();
        } else {
            current_string.push(c);
        }
    }
    result.push(current_string);
    result
}

#[cfg(test)]
mod test {
    use super::*;
    #[test]
    fn test_split_csv_line() {
        let line = r#""A","B, B","C""#;
        let result = split_csv_line(line);
        assert_eq!(result.len(), 3);
        assert_eq!(result[0], "A");
        assert_eq!(result[1], "B, B");
        assert_eq!(result[2], "C");
    }

    #[test]
    fn test_head() {
        let lines: Vec<(String, String)> = vec![("A".to_string(), "A".to_string()), 
          ("B".to_string(), "B".to_string()), ("C".to_string(), "C".to_string())];
        let result: Vec<(String, String)> = head(lines, 2);
        assert_eq!(result.len(), 2);
        assert_eq!(result[0], ("A".to_string(), "A".to_string()));
        assert_eq!(result[1], ("B".to_string(), "B".to_string()));
    }
}