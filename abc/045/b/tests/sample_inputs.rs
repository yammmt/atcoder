use cli_test_dir::*;

const BIN: &'static str = "./main";

#[test]
fn sample1() {
    let testdir = TestDir::new(BIN, "");
    let output = testdir
        .cmd()
        .output_with_stdin(r#"aca
accc
ca
"#)
        .tee_output()
        .expect_success();
    assert_eq!(output.stdout_str(), "A\n");
    assert!(output.stderr_str().is_empty());
}

#[test]
fn sample2() {
    let testdir = TestDir::new(BIN, "");
    let output = testdir
        .cmd()
        .output_with_stdin(r#"abcb
aacb
bccc
"#)
        .tee_output()
        .expect_success();
    assert_eq!(output.stdout_str(), "C\n");
    assert!(output.stderr_str().is_empty());
}
