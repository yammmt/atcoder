use cli_test_dir::*;

const BIN: &'static str = "./main";

#[test]
fn sample1() {
    let testdir = TestDir::new(BIN, "");
    let output = testdir
        .cmd()
        .output_with_stdin(r#"2 5
4 9
2 4
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
        .output_with_stdin(r#"4 30
6 18
2 5
3 10
7 9
"#)
        .tee_output()
        .expect_success();
    assert_eq!(output.stdout_str(), "130\n");
    assert!(output.stderr_str().is_empty());
}

#[test]
fn sample3() {
    let testdir = TestDir::new(BIN, "");
    let output = testdir
        .cmd()
        .output_with_stdin(r#"1 100000
1000000000 100000
"#)
        .tee_output()
        .expect_success();
    assert_eq!(output.stdout_str(), "100000000000000\n");
    assert!(output.stderr_str().is_empty());
}

