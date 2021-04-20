use cli_test_dir::*;

const BIN: &'static str = "./main";

#[test]
fn sample1() {
    let testdir = TestDir::new(BIN, "");
    let output = testdir
        .cmd()
        .output_with_stdin(r#"3 7
1 6 3
"#)
        .tee_output()
        .expect_success();
    assert_eq!(output.stdout_str(), "14\n");
    assert!(output.stderr_str().is_empty());
}

#[test]
fn sample2() {
    let testdir = TestDir::new(BIN, "");
    let output = testdir
        .cmd()
        .output_with_stdin(r#"4 9
7 4 0 3
"#)
        .tee_output()
        .expect_success();
    assert_eq!(output.stdout_str(), "46\n");
    assert!(output.stderr_str().is_empty());
}

#[test]
fn sample3() {
    let testdir = TestDir::new(BIN, "");
    let output = testdir
        .cmd()
        .output_with_stdin(r#"1 0
1000000000000
"#)
        .tee_output()
        .expect_success();
    assert_eq!(output.stdout_str(), "1000000000000\n");
    assert!(output.stderr_str().is_empty());
}

