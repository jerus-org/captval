use captval::Captval;
use claims::assert_ok;
use wasm_bindgen::prelude::*;

#[derive(Captval)]
struct Test {
    #[captcha]
    token: String,
}

#[wasm_bindgen]
pub async fn validate_standard() {
    let response = "10000000-aaaa-bbbb-cccc-000000000001";
    let secret = "0x0000000000000000000000000000000000000000";

    let form = Test {
        token: response.to_string(),
    };

    let response = form.valid_response(secret, None).await;

    assert_ok!(&response);
    let response = response.unwrap();
    assert!(&response.success());
}
