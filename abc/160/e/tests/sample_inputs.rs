use cli_test_dir::*;

const BIN: &'static str = "./main";

#[test]
fn sample1() {
    let testdir = TestDir::new(BIN, "");
    let output = testdir
        .cmd()
        .output_with_stdin(r#"1 2 2 2 1
2 4
5 1
3
"#)
        .tee_output()
        .expect_success();
    assert_eq!(output.stdout_str(), "12\n");
    assert!(output.stderr_str().is_empty());
}

#[test]
fn sample2() {
    let testdir = TestDir::new(BIN, "");
    let output = testdir
        .cmd()
        .output_with_stdin(r#"2 2 2 2 2
8 6
9 1
2 1
"#)
        .tee_output()
        .expect_success();
    assert_eq!(output.stdout_str(), "25\n");
    assert!(output.stderr_str().is_empty());
}

#[test]
fn sample3() {
    let testdir = TestDir::new(BIN, "");
    let output = testdir
        .cmd()
        .output_with_stdin(r#"2 2 4 4 4
11 12 13 14
21 22 23 24
1 2 3 4
"#)
        .tee_output()
        .expect_success();
    assert_eq!(output.stdout_str(), "74\n");
    assert!(output.stderr_str().is_empty());
}

