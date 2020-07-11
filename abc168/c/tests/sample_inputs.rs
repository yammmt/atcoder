use cli_test_dir::*;

const BIN: &'static str = "./main";

#[test]
fn sample1() {
    let testdir = TestDir::new(BIN, "");
    let output = testdir
        .cmd()
        .output_with_stdin(r#"3 4 9 0
"#)
        .tee_output()
        .expect_success();
    assert_eq!(output.stdout_str(), "5.00000000000000000000\n");
    assert!(output.stderr_str().is_empty());
}

#[test]
fn sample2() {
    let testdir = TestDir::new(BIN, "");
    let output = testdir
        .cmd()
        .output_with_stdin(r#"3 4 10 40
"#)
        .tee_output()
        .expect_success();
    assert_eq!(output.stdout_str(), "4.56425719433005567605\n");
    assert!(output.stderr_str().is_empty());
}
