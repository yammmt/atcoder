use cli_test_dir::*;

const BIN: &'static str = "./main";

#[test]
fn sample1() {
    let testdir = TestDir::new(BIN, "");
    let output = testdir
        .cmd()
        .output_with_stdin(r#"2
10 20 30
20 20 20
"#)
        .tee_output()
        .expect_success();
    assert_eq!(output.stdout_str(), "12000\n");
    assert!(output.stderr_str().is_empty());
}

#[test]
fn sample2() {
    let testdir = TestDir::new(BIN, "");
    let output = testdir
        .cmd()
        .output_with_stdin(r#"3
10 20 30
20 20 20
30 20 10
"#)
        .tee_output()
        .expect_success();
    assert_eq!(output.stdout_str(), "12000\n");
    assert!(output.stderr_str().is_empty());
}

#[test]
fn sample3() {
    let testdir = TestDir::new(BIN, "");
    let output = testdir
        .cmd()
        .output_with_stdin(r#"4
10 20 30
20 20 20
30 20 10
10 40 10
"#)
        .tee_output()
        .expect_success();
    assert_eq!(output.stdout_str(), "16000\n");
    assert!(output.stderr_str().is_empty());
}

#[test]
fn sample4() {
    let testdir = TestDir::new(BIN, "");
    let output = testdir
        .cmd()
        .output_with_stdin(r#"2
10 10 10
11 1 1
"#)
        .tee_output()
        .expect_success();
    assert_eq!(output.stdout_str(), "1100\n");
    assert!(output.stderr_str().is_empty());
}
