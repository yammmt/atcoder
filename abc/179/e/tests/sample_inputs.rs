use cli_test_dir::*;

const BIN: &'static str = "./main";

#[test]
fn sample1() {
    let testdir = TestDir::new(BIN, "");
    let output = testdir
        .cmd()
        .output_with_stdin(
            r#"6 2 1001
"#,
        )
        .tee_output()
        .expect_success();
    assert_eq!(output.stdout_str(), "1369\n");
    assert!(output.stderr_str().is_empty());
}

#[test]
fn sample2() {
    let testdir = TestDir::new(BIN, "");
    let output = testdir
        .cmd()
        .output_with_stdin(
            r#"1000 2 16
"#,
        )
        .tee_output()
        .expect_success();
    assert_eq!(output.stdout_str(), "6\n");
    assert!(output.stderr_str().is_empty());
}

#[test]
fn sample3() {
    let testdir = TestDir::new(BIN, "");
    let output = testdir
        .cmd()
        .output_with_stdin(
            r#"10000000000 10 99959
"#,
        )
        .tee_output()
        .expect_success();
    assert_eq!(output.stdout_str(), "492443256176507\n");
    assert!(output.stderr_str().is_empty());
}
