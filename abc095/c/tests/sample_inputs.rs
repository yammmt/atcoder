use cli_test_dir::*;

const BIN: &'static str = "./main";

#[test]
fn sample1() {
    let testdir = TestDir::new(BIN, "");
    let output = testdir
        .cmd()
        .output_with_stdin(r#"1500 2000 1600 3 2
"#)
        .tee_output()
        .expect_success();
    assert_eq!(output.stdout_str(), "7900\n");
    assert!(output.stderr_str().is_empty());
}

#[test]
fn sample2() {
    let testdir = TestDir::new(BIN, "");
    let output = testdir
        .cmd()
        .output_with_stdin(r#"1500 2000 1900 3 2
"#)
        .tee_output()
        .expect_success();
    assert_eq!(output.stdout_str(), "8500\n");
    assert!(output.stderr_str().is_empty());
}

#[test]
fn sample3() {
    let testdir = TestDir::new(BIN, "");
    let output = testdir
        .cmd()
        .output_with_stdin(r#"1500 2000 500 90000 100000
"#)
        .tee_output()
        .expect_success();
    assert_eq!(output.stdout_str(), "100000000\n");
    assert!(output.stderr_str().is_empty());
}

