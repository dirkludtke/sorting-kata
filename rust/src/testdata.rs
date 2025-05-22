pub struct TestdataParser;

impl TestdataParser {
    /// Parses a string containing a two-dimensional array in JSON-like format.
    pub fn parse(text: &str) -> Result<Vec<Vec<u16>>, String> {
        let mut chars = text.chars().peekable();
        let mut test_sets = Vec::new();

        Self::find_open(&mut chars)?;
        if chars.peek() != Some(&']') {
            loop {
                test_sets.push(Self::parse_testset(&mut chars)?);
                match chars.peek() {
                    Some(']') => break,
                    Some(',') => {
                        chars.next();
                        Self::skip_whitespace(&mut chars);
                    }
                    _ => return Err(format!("Expected ',' or ']' at position")),
                }
            }
        }
        chars.next();
        Self::skip_whitespace(&mut chars);
        if chars.peek().is_some() {
            return Err(format!("Unexpected character at the end of input"));
        }
        Ok(test_sets)
    }

    fn parse_testset(chars: &mut std::iter::Peekable<std::str::Chars>) -> Result<Vec<u16>, String> {
        let mut numbers = Vec::new();
        Self::find_open(chars)?;
        if chars.peek() != Some(&']') {
            loop {
                numbers.push(Self::parse_number(chars)?);
                match chars.peek() {
                    Some(']') => break,
                    Some(',') => {
                        chars.next();
                        Self::skip_whitespace(chars);
                    }
                    _ => return Err(format!("Expected ',' or ']' in test set")),
                }
            }
        }
        chars.next();
        Self::skip_whitespace(chars);
        Ok(numbers)
    }

    fn parse_number(chars: &mut std::iter::Peekable<std::str::Chars>) -> Result<u16, String> {
        Self::skip_whitespace(chars);
        let mut result = 0;
        let mut found_digit = false;

        while let Some(c) = chars.peek() {
            if c.is_ascii_digit() {
                found_digit = true;
                result = result * 10 + c.to_digit(10).unwrap() as u16;
                chars.next();
            } else {
                break;
            }
        }

        if !found_digit {
            return Err(format!("Expected a number"));
        }

        Self::skip_whitespace(chars);
        Ok(result)
    }

    fn find_open(chars: &mut std::iter::Peekable<std::str::Chars>) -> Result<(), String> {
        Self::skip_whitespace(chars);
        match chars.next() {
            Some('[') => {
                Self::skip_whitespace(chars);
                Ok(())
            }
            _ => Err(format!("Expected '['")),
        }
    }

    fn skip_whitespace(chars: &mut std::iter::Peekable<std::str::Chars>) {
        while let Some(c) = chars.peek() {
            if c.is_whitespace() {
                chars.next();
            } else {
                break;
            }
        }
    }
}
