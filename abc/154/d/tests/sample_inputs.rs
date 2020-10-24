use cli_test_dir::*;

const BIN: &'static str = "./main";

#[test]
fn sample1() {
    let testdir = TestDir::new(BIN, "");
    let output = testdir
        .cmd()
        .output_with_stdin(r#"5 3
1 2 2 4 5
"#)
        .tee_output()
        .expect_success();
    assert_eq!(output.stdout_str(), "7\n");
    assert!(output.stderr_str().is_empty());
}

#[test]
fn sample2() {
    let testdir = TestDir::new(BIN, "");
    let output = testdir
        .cmd()
        .output_with_stdin(r#"4 1
6 6 6 6
"#)
        .tee_output()
        .expect_success();
    assert_eq!(output.stdout_str(), "3.5\n");
    assert!(output.stderr_str().is_empty());
}

#[test]
fn sample3() {
    let testdir = TestDir::new(BIN, "");
    let output = testdir
        .cmd()
        .output_with_stdin(r#"10 4
17 13 13 12 15 20 10 13 17 11
"#)
        .tee_output()
        .expect_success();
    assert_eq!(output.stdout_str(), "32\n");
    assert!(output.stderr_str().is_empty());
}

