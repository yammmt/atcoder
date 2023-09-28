use cli_test_dir::*;

const BIN: &'static str = "./main";

#[test]
fn sample1() {
    let testdir = TestDir::new(BIN, "");
    let output = testdir
        .cmd()
        .output_with_stdin(r#"5
6 7 9 10 31
"#)
        .tee_output()
        .expect_success();
    assert_eq!(output.stdout_str(), "APPROVED\n");
    assert!(output.stderr_str().is_empty());
}

#[test]
fn sample2() {
    let testdir = TestDir::new(BIN, "");
    let output = testdir
        .cmd()
        .output_with_stdin(r#"3
28 27 24
"#)
        .tee_output()
        .expect_success();
    assert_eq!(output.stdout_str(), "DENIED\n");
    assert!(output.stderr_str().is_empty());
}
