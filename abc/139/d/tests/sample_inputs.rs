use cli_test_dir::*;

const BIN: &'static str = "./main";

#[test]
fn sample1() {
    let testdir = TestDir::new(BIN, "");
    let output = testdir
        .cmd()
        .output_with_stdin(r#"2
"#)
        .tee_output()
        .expect_success();
    assert_eq!(output.stdout_str(), "1\n");
    assert!(output.stderr_str().is_empty());
}

#[test]
fn sample2() {
    let testdir = TestDir::new(BIN, "13");
    let output = testdir
        .cmd()
        .output_with_stdin(r#"
"#)
        .tee_output()
        .expect_success();
    assert_eq!(output.stdout_str(), "78\n");
    assert!(output.stderr_str().is_empty());
}

#[test]
fn sample3() {
    let testdir = TestDir::new(BIN, "1");
    let output = testdir
        .cmd()
        .output_with_stdin(r#"0
"#)
        .tee_output()
        .expect_success();
    assert_eq!(output.stdout_str(), "\n");
    assert!(output.stderr_str().is_empty());
}

