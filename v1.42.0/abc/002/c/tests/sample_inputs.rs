use cli_test_dir::*;

const BIN: &'static str = "./main";

#[test]
fn sample1() {
    let testdir = TestDir::new(BIN, "");
    let output = testdir
        .cmd()
        .output_with_stdin(r#"1 0 3 0 2 5
"#)
        .tee_output()
        .expect_success();
    assert_eq!(output.stdout_str(), "5.0\n");
    assert!(output.stderr_str().is_empty());
}

#[test]
fn sample2() {
    let testdir = TestDir::new(BIN, "");
    let output = testdir
        .cmd()
        .output_with_stdin(r#"-1 -2 3 4 5 6
"#)
        .tee_output()
        .expect_success();
    assert_eq!(output.stdout_str(), "2.0\n");
    assert!(output.stderr_str().is_empty());
}

#[test]
fn sample3() {
    let testdir = TestDir::new(BIN, "");
    let output = testdir
        .cmd()
        .output_with_stdin(r#"298 520 903 520 4 663
"#)
        .tee_output()
        .expect_success();
    assert_eq!(output.stdout_str(), "43257.5\n");
    assert!(output.stderr_str().is_empty());
}

