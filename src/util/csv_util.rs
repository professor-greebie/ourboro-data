
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

    fn test_head() {
        let lines: Vec<(String, String)> = vec![("A".to_string(), "A".to_string()), 
          ("B".to_string(), "B".to_string()), ("C".to_string(), "C".to_string())];
        let result: Vec<(String, String)> = head(lines, 2);
        assert_eq!(result.len(), 2);
        assert_eq!(result[0], ("A".to_string(), "A".to_string()));
        assert_eq!(result[1], ("B".to_string(), "B".to_string()));
    }
}