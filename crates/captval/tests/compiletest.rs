#[test]
fn ui() {
    let t = trybuild::TestCases::new();
    t.compile_fail("tests/ui/compilefail/*.rs");
    t.pass("tests/ui/hcaptcha_pass/test_*.rs");
}
