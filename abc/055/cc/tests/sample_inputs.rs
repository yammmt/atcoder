use cli_test_dir::*;

const BIN: &'static str = "./main";

#[test]
fn sample1() {
    let testdir = TestDir::new(BIN, "");
    let output = testdir
        .cmd()
        .output_with_stdin(r#"1 6
"#)
        .tee_output()
        .expect_success();
    assert_eq!(output.stdout_str(), "2\n");
    assert!(output.stderr_str().is_empty());
}

#[test]
fn sample2() {
    let testdir = TestDir::new(BIN, "");
    let output = testdir
        .cmd()
        .output_with_stdin(r#"12345 678901
"#)
        .tee_output()
        .expect_success();
    assert_eq!(output.stdout_str(), "175897\n");
    assert!(output.stderr_str().is_empty());
}
