use crate::{Error};
use crate::Code;
use std::collections::HashSet;
use std::fmt;

#[derive(Debug, Default, Clone, serde::Deserialize, serde::Serialize)]
pub struct Sitekey(String);

impl fmt::Display for Sitekey {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.0)
    }
}

impl Sitekey {
    #[cfg_attr(
        feature = "trace",
        tracing::instrument(name = "Validate Site Key.", skip(s), level = "debug")
    )]
    pub fn parse(s: String) -> Result<Self, Error> {
        empty_sitekey(&s)?;
        invalid_sitekey(&s)?;

        Ok(Sitekey(s))
    }
}

#[cfg_attr(
    feature = "trace",
    tracing::instrument(name = "Return error on empty string.", skip(s), level = "debug")
)]
fn empty_sitekey(s: &str) -> Result<(), Error> {
    if s.trim().is_empty() {
        let mut codes = HashSet::new();
        codes.insert(Code::MissingSiteKey);

        #[cfg(feature = "trace")]
        tracing::debug!("{}", Code::MissingSiteKey);
        Err(Error::Codes(codes))
    } else {
        Ok(())
    }
}

#[cfg_attr(
    feature = "trace",
    tracing::instrument(name = "Return error if not a valid sitekey string.", skip(s), level = "debug")
)]
fn invalid_sitekey(s: &str) -> Result<(), Error> {
    // Google reCAPTCHA site keys are 40 characters long and base64url encoded
    let is_valid_length = s.len() == 40;
    let is_base64url_like = s.chars().all(|c| c.is_alphanumeric() || c == '-' || c == '_');
    
    if !is_valid_length || !is_base64url_like {
        let mut codes = HashSet::new();
        codes.insert(Code::InvalidSiteKey);

        #[cfg(feature = "trace")]
        tracing::debug!("{}", Code::InvalidSiteKey);
        Err(Error::Codes(codes))
    } else {
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::{Sitekey, Code};
    use crate::Error;
    use claims::{assert_err, assert_ok};

    #[test]
    fn whitespace_only_sitekeys_are_rejected() {
        let sitekey = " ".to_string();
        assert_err!(Sitekey::parse(sitekey));
    }

    #[test]
    fn empty_string_is_rejected() {
        let sitekey = "".to_string();
        assert_err!(Sitekey::parse(sitekey));
    }

    #[test]
    fn error_set_contains_missing_sitekey_error() {
        let sitekey = "".to_string();
        if let Err(Error::Codes(hs)) = Sitekey::parse(sitekey) {
            assert!(hs.contains(&Code::MissingSiteKey));
        }
    }

    #[test]
    fn error_set_contains_invalid_sitekey_error() {
        let sitekey = "invalid-key".to_string();
        let res = Sitekey::parse(sitekey);
        assert_err!(&res);

        if let Err(Error::Codes(hs)) = res {
            assert!(hs.contains(&Code::InvalidSiteKey));
        }
    }

    #[test]
    fn sitekey_with_invalid_characters_is_rejected() {
        let sitekey = "6LeIxAcTAAAAAJcZVRqyHh71UMIEGNQ@MXjiZK!I".to_string();
        assert_err!(Sitekey::parse(sitekey));
    }

    #[test]
    fn sitekey_with_wrong_length_is_rejected() {
        let sitekey = "6LeIxAcTAAAAAJcZVRqyHh71UMIEGNQ_MXjiZKhI123".to_string();
        assert_err!(Sitekey::parse(sitekey));
    }

    #[test]
    fn valid_sitekey_is_valid() {
        let sitekey = "6LeIxAcTAAAAAJcZVRqyHh71UMIEGNQ_MXjiZKhI".to_string();
        assert_ok!(Sitekey::parse(sitekey));
    }

    #[test]
    fn another_valid_sitekey_is_valid() {
        let sitekey = "6LeIxAcTAAAAAGG-vFI1TnRWxMZNFuojJ4WifJWe".to_string();
        assert_ok!(Sitekey::parse(sitekey));
    }
}
