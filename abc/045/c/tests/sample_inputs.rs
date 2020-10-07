use cli_test_dir::*;

const BIN: &'static str = "./main";

#[test]
fn sample1() {
    let testdir = TestDir::new(BIN, "");
    let output = testdir
        .cmd()
        .output_with_stdin(r#"125
"#)
        .tee_output()
        .expect_success();
    assert_eq!(output.stdout_str(), "176\n");
    assert!(output.stderr_str().is_empty());
}

#[test]
fn sample2() {
    let testdir = TestDir::new(BIN, "");
    let output = testdir
        .cmd()
        .output_with_stdin(r#"9999999999
"#)
        .tee_output()
        .expect_success();
    assert_eq!(output.stdout_str(), "12656242944\n");
    assert!(output.stderr_str().is_empty());
}
