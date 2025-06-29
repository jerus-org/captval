#[doc(hidden)]
mod client_response;
mod code;
#[cfg(not(feature = "ext"))]
#[doc(hidden)]
mod secret;
#[cfg(feature = "ext")]
#[doc(hidden)]
mod secret_ext;
#[doc(hidden)]
mod sitekey;

// pub(crate) use client_response::ClientResponse;
pub use code::Code;

pub use client_response::Token;
#[cfg(not(feature = "ext"))]
pub(crate) use secret::Secret;
#[allow(unused_imports)]
#[cfg(feature = "ext")]
pub(crate) use secret_ext::Secret;
#[allow(unused_imports)]
pub(crate) use sitekey::Sitekey;

/// Endpoint url for the Hcaptcha siteverify API.
pub const VERIFY_URL: &str = "https://api.hcaptcha.com/siteverify";
