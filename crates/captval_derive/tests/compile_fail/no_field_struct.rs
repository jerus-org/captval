use captval_derive::Captval;

#[derive(Captval)]
pub struct ContactForm {
    name: String,
    #[allow(dead_code)]
    phone: String,
    email: String,
    #[allow(dead_code)]
    message: String,
    token: String,
}

fn main() {
    println!("hello");
}
