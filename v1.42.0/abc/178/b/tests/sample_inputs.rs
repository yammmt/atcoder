use cli_test_dir::*;

const BIN: &'static str = "./main";

#[test]
fn sample1() {
    let testdir = TestDir::new(BIN, "");
    let output = testdir
        .cmd()
        .output_with_stdin(r#"1 2 1 1
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
        .output_with_stdin(r#"3 5 -4 -2
"#)
        .tee_output()
        .expect_success();
    assert_eq!(output.stdout_str(), "-6\n");
    assert!(output.stderr_str().is_empty());
}

#[test]
fn sample3() {
    let testdir = TestDir::new(BIN, "");
    let output = testdir
        .cmd()
        .output_with_stdin(r#"-1000000000 0 -1000000000 0
"#)
        .tee_output()
        .expect_success();
    assert_eq!(output.stdout_str(), "1000000000000000000\n");
    assert!(output.stderr_str().is_empty());
}

