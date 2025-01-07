fn parse_numbers(items: Vec<&str>) -> Result<Vec<i32>, std::num::ParseIntError> {
    items
        .into_iter()
        .map(|item| item.parse::<i32>())
        .collect::<Result<Vec<_>, _>>()
}

#[allow(clippy::useless_vec)]
fn main() {
    let items = vec!["42", "93", "apple", "17"];
    let parsed = parse_numbers(items);

    match parsed {
        Ok(numbers) => println!("Parsed numbers: {:?}", numbers),
        Err(e) => eprintln!("Failed to parse: {}", e),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_numbers_success() {
        let items = vec!["42", "93", "17"];
        let result = parse_numbers(items);
        assert!(result.is_ok());
        assert_eq!(result.unwrap(), vec![42, 93, 17]);
    }

    #[test]
    fn test_parse_numbers_failure() {
        let items = vec!["42", "apple", "17"];
        let result = parse_numbers(items);
        assert!(result.is_err());
    }

    #[test]
    fn test_parse_empty_list() {
        let items: Vec<&str> = vec![];
        let result = parse_numbers(items);
        assert!(result.is_ok());
        assert_eq!(result.unwrap(), vec![]);
    }
}
