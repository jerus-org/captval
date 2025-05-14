use captval_derive::Captval;

#[derive(Captval)]
pub enum ContactEnum {
    Name,
    Token,
}

fn main() {
    println!("hello");
}
