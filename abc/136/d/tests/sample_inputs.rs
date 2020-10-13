use cli_test_dir::*;

const BIN: &'static str = "./main";

#[test]
fn sample1() {
    let testdir = TestDir::new(BIN, "");
    let output = testdir
        .cmd()
        .output_with_stdin(r#"RRLRL
"#)
        .tee_output()
        .expect_success();
    assert_eq!(output.stdout_str(), "0 1 2 1 1\n");
    assert!(output.stderr_str().is_empty());
}

#[test]
fn sample2() {
    let testdir = TestDir::new(BIN, "");
    let output = testdir
        .cmd()
        .output_with_stdin(r#"RRLLLLRLRRLL
"#)
        .tee_output()
        .expect_success();
    assert_eq!(output.stdout_str(), "0 3 3 0 0 0 1 1 0 2 2 0\n");
    assert!(output.stderr_str().is_empty());
}

#[test]
fn sample3() {
    let testdir = TestDir::new(BIN, "");
    let output = testdir
        .cmd()
        .output_with_stdin(r#"RRRLLRLLRRRLLLLL
"#)
        .tee_output()
        .expect_success();
    assert_eq!(output.stdout_str(), "0 0 3 2 0 2 1 0 0 0 4 4 0 0 0 0\n");
    assert!(output.stderr_str().is_empty());
}

