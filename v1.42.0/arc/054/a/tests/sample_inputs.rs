use cli_test_dir::*;

const BIN: &'static str = "./main";

#[test]
fn sample1() {
    let testdir = TestDir::new(BIN, "");
    let output = testdir
        .cmd()
        .output_with_stdin(r#"6 2 3 1 5
"#)
        .tee_output()
        .expect_success();
    assert_eq!(output.stdout_str(), "0.8000000000\n");
    assert!(output.stderr_str().is_empty());
}

#[test]
fn sample2() {
    let testdir = TestDir::new(BIN, "");
    let output = testdir
        .cmd()
        .output_with_stdin(r#"6 2 10 1 5
"#)
        .tee_output()
        .expect_success();
    assert_eq!(output.stdout_str(), "0.2500000000\n");
    assert!(output.stderr_str().is_empty());
}

#[test]
fn sample3() {
    let testdir = TestDir::new(BIN, "");
    let output = testdir
        .cmd()
        .output_with_stdin(r#"6 3 1 5 3
"#)
        .tee_output()
        .expect_success();
    assert_eq!(output.stdout_str(), "1.0000000000\n");
    assert!(output.stderr_str().is_empty());
}

#[test]
fn sample4() {
    let testdir = TestDir::new(BIN, "");
    let output = testdir
        .cmd()
        .output_with_stdin(r#"10 7 7 6 0
"#)
        .tee_output()
        .expect_success();
    assert_eq!(output.stdout_str(), "0.2857142857\n");
    assert!(output.stderr_str().is_empty());
}
