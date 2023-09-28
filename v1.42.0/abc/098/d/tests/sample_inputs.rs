use cli_test_dir::*;

const BIN: &'static str = "./main";

#[test]
fn sample1() {
    let testdir = TestDir::new(BIN, "");
    let output = testdir
        .cmd()
        .output_with_stdin(
            r#"4
2 5 4 6
"#,
        )
        .tee_output()
        .expect_success();
    assert_eq!(output.stdout_str(), "5\n");
    assert!(output.stderr_str().is_empty());
}

#[test]
fn sample2() {
    let testdir = TestDir::new(BIN, "");
    let output = testdir
        .cmd()
        .output_with_stdin(
            r#"9
0 0 0 0 0 0 0 0 0
"#,
        )
        .tee_output()
        .expect_success();
    assert_eq!(output.stdout_str(), "45\n");
    assert!(output.stderr_str().is_empty());
}

#[test]
fn sample3() {
    let testdir = TestDir::new(BIN, "");
    let output = testdir
        .cmd()
        .output_with_stdin(
            r#"19
885 8 1 128 83 32 256 206 639 16 4 128 689 32 8 64 885 969 1
"#,
        )
        .tee_output()
        .expect_success();
    assert_eq!(output.stdout_str(), "37\n");
    assert!(output.stderr_str().is_empty());
}
