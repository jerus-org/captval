use crate::Code;
use crate::Error;
use std::collections::HashSet;
use std::fmt;

#[derive(Debug, Default, Clone, PartialEq, serde::Deserialize, serde::Serialize)]
pub struct Token(String);

impl Token {
    pub fn parse(s: String) -> Result<Token, Error> {
        if s.trim().is_empty() {
            let mut codes = HashSet::new();
            codes.insert(Code::MissingResponse);

            Err(Error::Codes(codes))
        } else {
            Ok(Token(s))
        }
    }

    pub fn as_str(&self) -> &str {
        &self.0
    }
}

impl fmt::Display for Token {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.0)
    }
}

#[cfg(test)]
mod tests {
    use super::{Code, Token};
    use crate::Error;
    use claims::assert_err;

    #[test]
    fn whitespace_only_names_are_rejected() {
        let response = " ".to_string();
        assert_err!(Token::parse(response));
    }

    #[test]
    fn empty_string_is_rejected() {
        let response = "".to_string();
        assert_err!(Token::parse(response));
    }

    #[test]
    fn error_set_contains_missing_response_error() {
        let response = "".to_string();

        if let Err(Error::Codes(hs)) = Token::parse(response) {
            assert!(hs.contains(&Code::MissingResponse));
        }
    }

    #[test]
    fn test_as_str() {
        let response = Token("test_response".to_string());
        assert_eq!(response.as_str(), "test_response");
    }

    #[test]
    fn valid_token_is_accepted() {
        let token = "03AGdBq25SiUFR_j4TdKKe-P8CRjRz5uVwPB8Vwh123ABC".to_string();
        let result = Token::parse(token.clone());
        assert!(result.is_ok());
        assert_eq!(result.unwrap().as_str(), token);
    }
}
