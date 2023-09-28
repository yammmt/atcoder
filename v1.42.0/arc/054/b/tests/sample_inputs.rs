use cli_test_dir::*;

const BIN: &'static str = "./main";

#[test]
fn sample1() {
    let testdir = TestDir::new(BIN, "");
    let output = testdir
        .cmd()
        .output_with_stdin(r#"3.0000
"#)
        .tee_output()
        .expect_success();
    assert_eq!(output.stdout_str(), "2.8708930019\n");
    assert!(output.stderr_str().is_empty());
}

#[test]
fn sample2() {
    let testdir = TestDir::new(BIN, "");
    let output = testdir
        .cmd()
        .output_with_stdin(r#"0.0400
"#)
        .tee_output()
        .expect_success();
    assert_eq!(output.stdout_str(), "0.04\n");
    assert!(output.stderr_str().is_empty());
}

#[test]
fn sample3() {
    let testdir = TestDir::new(BIN, "");
    let output = testdir
        .cmd()
        .output_with_stdin(r#"1000000000000000000.0000
"#)
        .tee_output()
        .expect_success();
    assert_eq!(output.stdout_str(), "90.1855078128\n");
    assert!(output.stderr_str().is_empty());
}

