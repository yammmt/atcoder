use cli_test_dir::*;

const BIN: &'static str = "./main";

#[test]
fn sample1() {
    let testdir = TestDir::new(BIN, "");
    let output = testdir
        .cmd()
        .output_with_stdin(r#"1222
"#)
        .tee_output()
        .expect_success();
    assert_eq!(output.stdout_str(), "1+2+2+2=7\n");
    assert!(output.stderr_str().is_empty());
}

#[test]
fn sample2() {
    let testdir = TestDir::new(BIN, "");
    let output = testdir
        .cmd()
        .output_with_stdin(r#"0290
"#)
        .tee_output()
        .expect_success();
    assert_eq!(output.stdout_str(), "0-2+9+0=7\n");
    assert!(output.stderr_str().is_empty());
}

#[test]
fn sample3() {
    let testdir = TestDir::new(BIN, "");
    let output = testdir
        .cmd()
        .output_with_stdin(r#"3242
"#)
        .tee_output()
        .expect_success();
    assert_eq!(output.stdout_str(), "3+2+4-2=7\n");
    assert!(output.stderr_str().is_empty());
}

