use captval_derive::Captval;

#[derive(Captval)]
pub enum ContactEnum {
    Name,
    #[captcha]
    Token,
}

fn main() {
    println!("hello");
}
