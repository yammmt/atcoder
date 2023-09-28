use cli_test_dir::*;

const BIN: &'static str = "./main";

#[test]
fn sample1() {
    let testdir = TestDir::new(BIN, "");
    let output = testdir
        .cmd()
        .output_with_stdin(
            r#"1 2
10 10
12 4
"#,
        )
        .tee_output()
        .expect_success();
    assert_eq!(output.stdout_str(), "1\n");
    assert!(output.stderr_str().is_empty());
}

#[test]
fn sample2() {
    let testdir = TestDir::new(BIN, "");
    let output = testdir
        .cmd()
        .output_with_stdin(
            r#"100 1
101 101
102 1
"#,
        )
        .tee_output()
        .expect_success();
    assert_eq!(output.stdout_str(), "infinity\n");
    assert!(output.stderr_str().is_empty());
}

#[test]
fn sample3() {
    let testdir = TestDir::new(BIN, "");
    let output = testdir
        .cmd()
        .output_with_stdin(
            r#"12000 15700
3390000000 3810000000
5550000000 2130000000
"#,
        )
        .tee_output()
        .expect_success();
    assert_eq!(output.stdout_str(), "113\n");
    assert!(output.stderr_str().is_empty());
}
