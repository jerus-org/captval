use captval_derive::Captval;

#[derive(Debug, Captval)]
pub struct ContactForm {
    name: String,
    #[allow(dead_code)]
    phone: String,
    email: String,
    #[allow(dead_code)]
    message: String,
    #[captcha]
    token: String,
}
