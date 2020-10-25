use cli_test_dir::*;

const BIN: &'static str = "./main";

#[test]
fn sample1() {
    let testdir = TestDir::new(BIN, "");
    let output = testdir
        .cmd()
        .output_with_stdin(r#"4
"#)
        .tee_output()
        .expect_success();
    assert_eq!(output.stdout_str(), "23\n");
    assert!(output.stderr_str().is_empty());
}

#[test]
fn sample2() {
    let testdir = TestDir::new(BIN, "");
    let output = testdir
        .cmd()
        .output_with_stdin(r#"100
"#)
        .tee_output()
        .expect_success();
    assert_eq!(output.stdout_str(), "26879\n");
    assert!(output.stderr_str().is_empty());
}

#[test]
fn sample3() {
    let testdir = TestDir::new(BIN, "");
    let output = testdir
        .cmd()
        .output_with_stdin(r#"10000000
"#)
        .tee_output()
        .expect_success();
    assert_eq!(output.stdout_str(), "838627288460105\n");
    assert!(output.stderr_str().is_empty());
}

