use captval::Captval;
// use captval_derive::Captval;

#[derive(Captval)]
enum Test {
    #[captcha]
    Captval(String),
}

fn main() {
    println!("Super!");
}
