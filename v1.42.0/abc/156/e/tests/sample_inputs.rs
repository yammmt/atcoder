use cli_test_dir::*;

const BIN: &'static str = "./main";

#[test]
fn sample1() {
    let testdir = TestDir::new(BIN, "");
    let output = testdir
        .cmd()
        .output_with_stdin(
            r#"3 2
"#,
        )
        .tee_output()
        .expect_success();
    assert_eq!(output.stdout_str(), "10\n");
    assert!(output.stderr_str().is_empty());
}

#[test]
fn sample2() {
    let testdir = TestDir::new(BIN, "");
    let output = testdir
        .cmd()
        .output_with_stdin(
            r#"200000 1000000000
"#,
        )
        .tee_output()
        .expect_success();
    assert_eq!(output.stdout_str(), "607923868\n");
    assert!(output.stderr_str().is_empty());
}

#[test]
fn sample3() {
    let testdir = TestDir::new(BIN, "");
    let output = testdir
        .cmd()
        .output_with_stdin(
            r#"15 6
"#,
        )
        .tee_output()
        .expect_success();
    assert_eq!(output.stdout_str(), "22583772\n");
    assert!(output.stderr_str().is_empty());
}
