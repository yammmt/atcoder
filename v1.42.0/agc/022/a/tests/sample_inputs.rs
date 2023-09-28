use cli_test_dir::*;

const BIN: &'static str = "./main";

#[test]
fn sample1() {
    let testdir = TestDir::new(BIN, "");
    let output = testdir
        .cmd()
        .output_with_stdin(r#"atcoder
"#)
        .tee_output()
        .expect_success();
    assert_eq!(output.stdout_str(), "atcoderb\n");
    assert!(output.stderr_str().is_empty());
}

#[test]
fn sample2() {
    let testdir = TestDir::new(BIN, "");
    let output = testdir
        .cmd()
        .output_with_stdin(r#"abc
"#)
        .tee_output()
        .expect_success();
    assert_eq!(output.stdout_str(), "abcd\n");
    assert!(output.stderr_str().is_empty());
}

#[test]
fn sample3() {
    let testdir = TestDir::new(BIN, "");
    let output = testdir
        .cmd()
        .output_with_stdin(r#"zyxwvutsrqponmlkjihgfedcba
"#)
        .tee_output()
        .expect_success();
    assert_eq!(output.stdout_str(), "-1\n");
    assert!(output.stderr_str().is_empty());
}

#[test]
fn sample4() {
    let testdir = TestDir::new(BIN, "");
    let output = testdir
        .cmd()
        .output_with_stdin(r#"abcdefghijklmnopqrstuvwzyx
"#)
        .tee_output()
        .expect_success();
    assert_eq!(output.stdout_str(), "abcdefghijklmnopqrstuvx\n");
    assert!(output.stderr_str().is_empty());
}

#[test]
fn sample5() {
    let testdir = TestDir::new(BIN, "");
    let output = testdir
        .cmd()
        .output_with_stdin(r#"igcjkzlhomsupdaqwrxtvefnby
"#)
        .tee_output()
        .expect_success();
    assert_eq!(output.stdout_str(), "igcjkzlhomsupdaqwrxtvefny\n");
    assert!(output.stderr_str().is_empty());
}

#[test]
fn sample6() {
    let testdir = TestDir::new(BIN, "");
    let output = testdir
        .cmd()
        .output_with_stdin(r#"smtkcpwrnzyxvuqoljihgfedba
"#)
        .tee_output()
        .expect_success();
    assert_eq!(output.stdout_str(), "smtkcpwro\n");
    assert!(output.stderr_str().is_empty());
}
