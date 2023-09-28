use cli_test_dir::*;

const BIN: &'static str = "./main";

#[test]
fn sample1() {
    let testdir = TestDir::new(BIN, "");
    let output = testdir
        .cmd()
        .output_with_stdin(r#"20 4
3 7 8 4
"#)
        .tee_output()
        .expect_success();
    assert_eq!(output.stdout_str(), "777773\n");
    assert!(output.stderr_str().is_empty());
}

#[test]
fn sample2() {
    let testdir = TestDir::new(BIN, "");
    let output = testdir
        .cmd()
        .output_with_stdin(r#"101 9
9 8 7 6 5 4 3 2 1
"#)
        .tee_output()
        .expect_success();
    assert_eq!(output.stdout_str(), "71111111111111111111111111111111111111111111111111\n");
    assert!(output.stderr_str().is_empty());
}

#[test]
fn sample3() {
    let testdir = TestDir::new(BIN, "");
    let output = testdir
        .cmd()
        .output_with_stdin(r#"15 3
5 4 6
"#)
        .tee_output()
        .expect_success();
    assert_eq!(output.stdout_str(), "654\n");
    assert!(output.stderr_str().is_empty());
}

