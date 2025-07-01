use crate::{Code, Error};
use std::collections::HashSet;
use std::fmt;

#[derive(Debug, Default, Clone, serde::Serialize)]
pub struct Secret(String);

impl fmt::Display for Secret {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.0)
    }
}

impl Secret {
    #[cfg_attr(
        feature = "trace",
        tracing::instrument(name = "Simple check of secret.", skip(s), level = "debug")
    )]
    pub fn parse(s: String) -> Result<Self, Error> {
        if s.trim().is_empty() {
            let mut codes = HashSet::new();
            codes.insert(Code::MissingSecret);

            #[cfg(feature = "trace")]
            tracing::debug!("Secret string is missing");
            Err(Error::Codes(codes))
        } else {
            Ok(Secret(s))
        }
    }
}

#[cfg(test)]
mod tests {
    use super::{Code, Secret};
    use crate::Error;
    use claims::{assert_err, assert_ok};

    #[test]
    fn whitespace_only_secrets_are_rejected() {
        let secret = " ".to_string();
        assert_err!(Secret::parse(secret));
    }

    #[test]
    fn empty_string_is_rejected() {
        let secret = "".to_string();
        assert_err!(Secret::parse(secret));
    }

    #[test]
    fn error_set_contains_missing_secret_error() {
        let secret = "".to_string();
        if let Err(Error::Codes(hs)) = Secret::parse(secret) {
            assert!(hs.contains(&Code::MissingSecret));
        }
    }

    #[test]
    fn test_valid_secret_key() {
        let secret = "6LeIxAcTAAAAAJcZVRqyHh71UMIEGNQ_MXjiZKhI".to_string();
        assert_ok!(Secret::parse(secret));
    }

    #[test]
    fn test_another_valid_secret_key() {
        let secret = "6LeIxAcTAAAAAGG-vFI1TnRWxMZNFuojJ4WifJWe".to_string();
        assert_ok!(Secret::parse(secret));
    }
}
