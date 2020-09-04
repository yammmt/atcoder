use cli_test_dir::*;

const BIN: &'static str = "./main";

#[test]
fn sample1() {
    let testdir = TestDir::new(BIN, "");
    let output = testdir
        .cmd()
        .output_with_stdin(r#"3
1 2 3
"#)
        .tee_output()
        .expect_success();
    assert_eq!(output.stdout_str(), "11\n");
    assert!(output.stderr_str().is_empty());
}

#[test]
fn sample2() {
    let testdir = TestDir::new(BIN, "");
    let output = testdir
        .cmd()
        .output_with_stdin(r#"4
141421356 17320508 22360679 244949
"#)
        .tee_output()
        .expect_success();
    assert_eq!(output.stdout_str(), "437235829\n");
    assert!(output.stderr_str().is_empty());
}
