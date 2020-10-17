use cli_test_dir::*;

const BIN: &'static str = "./main";

#[test]
fn sample1() {
    let testdir = TestDir::new(BIN, "");
    let output = testdir
        .cmd()
        .output_with_stdin(r#"3
2 2 4
"#)
        .tee_output()
        .expect_success();
    assert_eq!(output.stdout_str(), "4 0 4\n");
    assert!(output.stderr_str().is_empty());
}

#[test]
fn sample2() {
    let testdir = TestDir::new(BIN, "");
    let output = testdir
        .cmd()
        .output_with_stdin(r#"5
3 8 7 5 5
"#)
        .tee_output()
        .expect_success();
    assert_eq!(output.stdout_str(), "2 4 12 2 8\n");
    assert!(output.stderr_str().is_empty());
}

#[test]
fn sample3() {
    let testdir = TestDir::new(BIN, "");
    let output = testdir
        .cmd()
        .output_with_stdin(r#"3
1000000000 1000000000 0
"#)
        .tee_output()
        .expect_success();
    assert_eq!(output.stdout_str(), "0 2000000000 0\n");
    assert!(output.stderr_str().is_empty());
}

