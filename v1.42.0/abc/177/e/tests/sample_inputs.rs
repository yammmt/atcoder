use cli_test_dir::*;

const BIN: &'static str = "./main";

#[test]
fn sample1() {
    let testdir = TestDir::new(BIN, "");
    let output = testdir
        .cmd()
        .output_with_stdin(r#"3
3 4 5
"#)
        .tee_output()
        .expect_success();
    assert_eq!(output.stdout_str(), "pairwise coprime\n");
    assert!(output.stderr_str().is_empty());
}

#[test]
fn sample2() {
    let testdir = TestDir::new(BIN, "");
    let output = testdir
        .cmd()
        .output_with_stdin(r#"3
6 10 15
"#)
        .tee_output()
        .expect_success();
    assert_eq!(output.stdout_str(), "setwise coprime\n");
    assert!(output.stderr_str().is_empty());
}

#[test]
fn sample3() {
    let testdir = TestDir::new(BIN, "");
    let output = testdir
        .cmd()
        .output_with_stdin(r#"3
6 10 16
"#)
        .tee_output()
        .expect_success();
    assert_eq!(output.stdout_str(), "not coprime\n");
    assert!(output.stderr_str().is_empty());
}

