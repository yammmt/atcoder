use cli_test_dir::*;

const BIN: &'static str = "./main";

#[test]
fn sample1() {
    let testdir = TestDir::new(BIN, "");
    let output = testdir
        .cmd()
        .output_with_stdin(r#"3 8 2
"#)
        .tee_output()
        .expect_success();
    assert_eq!(output.stdout_str(), r#"3
4
7
8
"#);
    assert!(output.stderr_str().is_empty());
}

#[test]
fn sample2() {
    let testdir = TestDir::new(BIN, "");
    let output = testdir
        .cmd()
        .output_with_stdin(r#"4 8 3
"#)
        .tee_output()
        .expect_success();
    assert_eq!(output.stdout_str(), r#"4
5
6
7
8
"#);
    assert!(output.stderr_str().is_empty());
}

#[test]
fn sample3() {
    let testdir = TestDir::new(BIN, "");
    let output = testdir
        .cmd()
        .output_with_stdin(r#"2 9 100
"#)
        .tee_output()
        .expect_success();
    assert_eq!(output.stdout_str(), r#"2
3
4
5
6
7
8
9
"#);
    assert!(output.stderr_str().is_empty());
}

