use captval::Captval;
use captval::Error;
use claims::assert_ok;

#[derive(Debug, Captval)]
pub struct Test {
    #[captcha]
    captval: String,
    #[sitekey]
    sitekey: String,
}

pub async fn recaptcha_integration_test(
    response: &str,
    site_key: &str,
    secret_key: &str,
) -> Result<(), Error> {
    let form = Test {
        captval: response.to_string(),
        sitekey: site_key.to_string(),
    };

    let response = form.valid_response(secret_key, None).await;

    assert_ok!(&response);
    let response = response.unwrap();
    assert!(&response.success());

    Ok(())
}
