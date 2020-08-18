use cli_test_dir::*;

const BIN: &'static str = "./main";

#[test]
fn sample1() {
    let testdir = TestDir::new(BIN, "");
    let output = testdir
        .cmd()
        .output_with_stdin(r#"5 4 2
2 1 1
3 3 4
"#)
        .tee_output()
        .expect_success();
    assert_eq!(output.stdout_str(), "9\n");
    assert!(output.stderr_str().is_empty());
}

#[test]
fn sample2() {
    let testdir = TestDir::new(BIN, "");
    let output = testdir
        .cmd()
        .output_with_stdin(r#"5 4 3
2 1 1
3 3 4
1 4 2
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
        .output_with_stdin(r#"10 10 5
1 6 1
4 1 3
6 9 4
9 4 2
3 1 3
"#)
        .tee_output()
        .expect_success();
    assert_eq!(output.stdout_str(), "64\n");
    assert!(output.stderr_str().is_empty());
}

