use cli_test_dir::*;

const BIN: &'static str = "./main";

#[test]
fn sample1() {
    let testdir = TestDir::new(BIN, "");
    let output = testdir
        .cmd()
        .output_with_stdin(r#"4
2 4 4 3
"#)
        .tee_output()
        .expect_success();
    assert_eq!(output.stdout_str(), r#"4
3
3
4
"#);
    assert!(output.stderr_str().is_empty());
}

#[test]
fn sample2() {
    let testdir = TestDir::new(BIN, "");
    let output = testdir
        .cmd()
        .output_with_stdin(r#"2
1 2
"#)
        .tee_output()
        .expect_success();
    assert_eq!(output.stdout_str(), r#"2
1
"#);
    assert!(output.stderr_str().is_empty());
}

#[test]
fn sample3() {
    let testdir = TestDir::new(BIN, "");
    let output = testdir
        .cmd()
        .output_with_stdin(r#"6
5 5 4 4 3 3
"#)
        .tee_output()
        .expect_success();
    assert_eq!(output.stdout_str(), r#"4
4
4
4
4
4
"#);
    assert!(output.stderr_str().is_empty());
}

