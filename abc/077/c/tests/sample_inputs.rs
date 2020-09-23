use cli_test_dir::*;

const BIN: &'static str = "./main";

#[test]
fn sample1() {
    let testdir = TestDir::new(BIN, "");
    let output = testdir
        .cmd()
        .output_with_stdin(r#"2
1 5
2 4
3 6
"#)
        .tee_output()
        .expect_success();
    assert_eq!(output.stdout_str(), "3\n");
    assert!(output.stderr_str().is_empty());
}

#[test]
fn sample2() {
    let testdir = TestDir::new(BIN, "");
    let output = testdir
        .cmd()
        .output_with_stdin(r#"3
1 1 1
2 2 2
3 3 3
"#)
        .tee_output()
        .expect_success();
    assert_eq!(output.stdout_str(), "27\n");
    assert!(output.stderr_str().is_empty());
}

#[test]
fn sample3() {
    let testdir = TestDir::new(BIN, "");
    let output = testdir
        .cmd()
        .output_with_stdin(r#"6
3 14 159 2 6 53
58 9 79 323 84 6
2643 383 2 79 50 288
"#)
        .tee_output()
        .expect_success();
    assert_eq!(output.stdout_str(), "87\n");
    assert!(output.stderr_str().is_empty());
}

