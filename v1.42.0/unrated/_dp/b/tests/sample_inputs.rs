use cli_test_dir::*;

const BIN: &'static str = "./main";

#[test]
fn sample1() {
    let testdir = TestDir::new(BIN, "");
    let output = testdir
        .cmd()
        .output_with_stdin(r#"5 3
10 30 40 50 20
"#)
        .tee_output()
        .expect_success();
    assert_eq!(output.stdout_str(), "30\n");
    assert!(output.stderr_str().is_empty());
}

#[test]
fn sample2() {
    let testdir = TestDir::new(BIN, "");
    let output = testdir
        .cmd()
        .output_with_stdin(r#"3 1
10 20 10
"#)
        .tee_output()
        .expect_success();
    assert_eq!(output.stdout_str(), "20\n");
    assert!(output.stderr_str().is_empty());
}

#[test]
fn sample3() {
    let testdir = TestDir::new(BIN, "");
    let output = testdir
        .cmd()
        .output_with_stdin(r#"2 100
10 10
"#)
        .tee_output()
        .expect_success();
    assert_eq!(output.stdout_str(), "0\n");
    assert!(output.stderr_str().is_empty());
}

#[test]
fn sample4() {
    let testdir = TestDir::new(BIN, "");
    let output = testdir
        .cmd()
        .output_with_stdin(r#"10 4
40 10 20 70 80 10 20 70 80 60
"#)
        .tee_output()
        .expect_success();
    assert_eq!(output.stdout_str(), "40\n");
    assert!(output.stderr_str().is_empty());
}
