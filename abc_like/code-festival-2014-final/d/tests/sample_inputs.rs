use cli_test_dir::*;

const BIN: &'static str = "./main";

#[test]
fn sample1() {
    let testdir = TestDir::new(BIN, "");
    let output = testdir
        .cmd()
        .output_with_stdin(r#"10
"#)
        .tee_output()
        .expect_success();
    assert_eq!(output.stdout_str(), "6 3\n");
    assert!(output.stderr_str().is_empty());
}

#[test]
fn sample2() {
    let testdir = TestDir::new(BIN, "");
    let output = testdir
        .cmd()
        .output_with_stdin(r#"3921225
"#)
        .tee_output()
        .expect_success();
    assert_eq!(output.stdout_str(), "101 5\n");
    assert!(output.stderr_str().is_empty());
}

#[test]
fn sample3() {
    let testdir = TestDir::new(BIN, "");
    let output = testdir
        .cmd()
        .output_with_stdin(r#"1
"#)
        .tee_output()
        .expect_success();
    assert_eq!(output.stdout_str(), "1 1\n");
    assert!(output.stderr_str().is_empty());
}
