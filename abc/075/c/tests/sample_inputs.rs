use cli_test_dir::*;

const BIN: &'static str = "./main";

#[test]
fn sample1() {
    let testdir = TestDir::new(BIN, "");
    let output = testdir
        .cmd()
        .output_with_stdin(r#"7 7
1 3
2 7
3 4
4 5
4 6
5 6
6 7
"#)
        .tee_output()
        .expect_success();
    assert_eq!(output.stdout_str(), "4\n");
    assert!(output.stderr_str().is_empty());
}

#[test]
fn sample2() {
    let testdir = TestDir::new(BIN, "");
    let output = testdir
        .cmd()
        .output_with_stdin(r#"3 3
1 2
1 3
2 3
"#)
        .tee_output()
        .expect_success();
    assert_eq!(output.stdout_str(), "0\n");
    assert!(output.stderr_str().is_empty());
}

#[test]
fn sample3() {
    let testdir = TestDir::new(BIN, "");
    let output = testdir
        .cmd()
        .output_with_stdin(r#"6 5
1 2
2 3
3 4
4 5
5 6
"#)
        .tee_output()
        .expect_success();
    assert_eq!(output.stdout_str(), "5\n");
    assert!(output.stderr_str().is_empty());
}

