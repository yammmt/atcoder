use cli_test_dir::*;

const BIN: &'static str = "./main";

#[test]
fn sample1() {
    let testdir = TestDir::new(BIN, "");
    let output = testdir
        .cmd()
        .output_with_stdin(r#"5 3
"#)
        .tee_output()
        .expect_success();
    assert_eq!(output.stdout_str(), r#"3
6
1
"#);
    assert!(output.stderr_str().is_empty());
}

#[test]
fn sample2() {
    let testdir = TestDir::new(BIN, "");
    let output = testdir
        .cmd()
        .output_with_stdin(r#"2000 3
"#)
        .tee_output()
        .expect_success();
    assert_eq!(output.stdout_str(), r#"1998
3990006
327341989
"#);
    assert!(output.stderr_str().is_empty());
}
