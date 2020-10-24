use cli_test_dir::*;

const BIN: &'static str = "./main";

#[test]
fn sample1() {
    let testdir = TestDir::new(BIN, "");
    let output = testdir
        .cmd()
        .output_with_stdin(r#"100
1
"#)
        .tee_output()
        .expect_success();
    assert_eq!(output.stdout_str(), "19\n");
    assert!(output.stderr_str().is_empty());
}

#[test]
fn sample2() {
    let testdir = TestDir::new(BIN, "");
    let output = testdir
        .cmd()
        .output_with_stdin(r#"25
2
"#)
        .tee_output()
        .expect_success();
    assert_eq!(output.stdout_str(), "14\n");
    assert!(output.stderr_str().is_empty());
}

#[test]
fn sample3() {
    let testdir = TestDir::new(BIN, "");
    let output = testdir
        .cmd()
        .output_with_stdin(r#"314159
2
"#)
        .tee_output()
        .expect_success();
    assert_eq!(output.stdout_str(), "937\n");
    assert!(output.stderr_str().is_empty());
}

#[test]
fn sample4() {
    let testdir = TestDir::new(BIN, "");
    let output = testdir
        .cmd()
        .output_with_stdin(r#"9999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999
3
"#)
        .tee_output()
        .expect_success();
    assert_eq!(output.stdout_str(), "117879300\n");
    assert!(output.stderr_str().is_empty());
}
