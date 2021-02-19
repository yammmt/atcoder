use cli_test_dir::*;

const BIN: &'static str = "./main";

#[test]
fn sample1() {
    let testdir = TestDir::new(BIN, "");
    let output = testdir
        .cmd()
        .output_with_stdin(r#"3
3 2 1
2 2 1
1 1 1
3
1
4
9
"#)
        .tee_output()
        .expect_success();
    assert_eq!(output.stdout_str(), r#"3
9
14
"#);
    assert!(output.stderr_str().is_empty());
}

#[test]
fn sample2() {
    let testdir = TestDir::new(BIN, "");
    let output = testdir
        .cmd()
        .output_with_stdin(r#"3
1 1 1
1 1 1
9 9 9
1
4
"#)
        .tee_output()
        .expect_success();
    assert_eq!(output.stdout_str(), "27\n");
    assert!(output.stderr_str().is_empty());
}
