use cli_test_dir::*;

const BIN: &'static str = "./main";

#[test]
fn sample1() {
    let testdir = TestDir::new(BIN, "");
    let output = testdir
        .cmd()
        .output_with_stdin(r#"2 5
0 1 0 1 0
1 0 0 0 1
"#)
        .tee_output()
        .expect_success();
    assert_eq!(output.stdout_str(), "9\n");
    assert!(output.stderr_str().is_empty());
}

#[test]
fn sample2() {
    let testdir = TestDir::new(BIN, "");
    let output = testdir
        .cmd()
        .output_with_stdin(r#"3 6
1 0 0 0 1 0
1 1 1 0 1 0
1 0 1 1 0 1
"#)
        .tee_output()
        .expect_success();
    assert_eq!(output.stdout_str(), "15\n");
    assert!(output.stderr_str().is_empty());
}
