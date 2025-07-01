#[doc(hidden)]
mod secret;
#[doc(hidden)]
mod sitekey;
#[doc(hidden)]
mod token;

#[allow(unused_imports)]
pub(crate) use secret::Secret;
#[allow(unused_imports)]
pub(crate) use sitekey::Sitekey;
#[allow(unused_imports)]
pub(crate) use token::Token;

/// Endpoint url for the Google reCAPTCHA siteverify API.
pub const VERIFY_URL: &str = "https://www.google.com/recaptcha/api/siteverify";
