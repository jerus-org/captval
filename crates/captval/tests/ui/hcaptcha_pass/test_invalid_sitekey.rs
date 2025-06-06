mod helper;

use captval::{Captval, Code};
use chrono::{TimeDelta, Utc};
use serde_json::json;
use wiremock::matchers::{body_string, method, path};
use wiremock::{Mock, MockServer, ResponseTemplate};

#[derive(Debug, Captval)]
struct Test {
    #[captcha]
    captval: String,
    #[sitekey]
    sitekey: String,
    #[remoteip]
    ip: String,
}

#[tokio::main]
async fn main() {
    // Setup
    let token = helper::random_string(100);
    let remoteip = mockd::internet::ipv4_address();
    // let sitekey = mockd::unique::uuid_v4();
    let sitekey = "mockd::unique::uuid_v4()".to_string();
    let secret = format!("0x{}", hex::encode(helper::random_string(20)));

    let expected_body = format!(
        "response={}&remoteip={}&sitekey={}&secret={}",
        &token, &remoteip, &sitekey, &secret
    );

    let timestamp = Utc::now()
        .checked_sub_signed(TimeDelta::try_minutes(10).unwrap())
        .unwrap()
        .to_rfc3339();

    let response_template = ResponseTemplate::new(200).set_body_json(json!({
        "success": true,
        "challenge_ts": timestamp,
        "hostname": "test-host",
    }));

    let mock_server = MockServer::start().await;
    Mock::given(method("POST"))
        .and(path("/siteverify"))
        .and(body_string(&expected_body))
        .respond_with(response_template)
        .mount(&mock_server)
        .await;

    let uri = format!("{}{}", mock_server.uri(), "/siteverify");

    let form = Test {
        captval: token,
        sitekey,
        ip: remoteip,
    };
    let response = form.valid_response(&secret, Some(uri)).await;

    claims::assert_err!(&response);

    if let Err(codes) = response {
        match codes {
            captval::Error::Codes(hash_set) => {
                assert_eq!(hash_set.len(), 1);
                assert!(hash_set.contains(&Code::InvalidSiteKey));
            }
            _ => unreachable!(),
        }
    };
}
