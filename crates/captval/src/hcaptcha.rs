mod code;
#[cfg(not(feature = "ext"))]
#[doc(hidden)]
mod secret;
#[cfg(feature = "ext")]
#[doc(hidden)]
mod secret_ext;
#[doc(hidden)]
mod sitekey;
#[doc(hidden)]
mod token;

// pub(crate) use client_response::ClientResponse;
pub use code::Code;

#[cfg(not(feature = "ext"))]
pub(crate) use secret::Secret;
#[allow(unused_imports)]
#[cfg(feature = "ext")]
pub(crate) use secret_ext::Secret;
#[allow(unused_imports)]
pub(crate) use sitekey::Sitekey;
pub use token::Token;

/// Endpoint url for the Hcaptcha siteverify API.
pub const VERIFY_URL: &str = "https://api.hcaptcha.com/siteverify";
