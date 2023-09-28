use cli_test_dir::*;

const BIN: &'static str = "./main";

#[test]
fn sample1() {
    let testdir = TestDir::new(BIN, "");
    let output = testdir
        .cmd()
        .output_with_stdin(r#"1
3
1 2 3
3
2 3 4
"#)
        .tee_output()
        .expect_success();
    assert_eq!(output.stdout_str(), "yes\n");
    assert!(output.stderr_str().is_empty());
}

#[test]
fn sample2() {
    let testdir = TestDir::new(BIN, "");
    let output = testdir
        .cmd()
        .output_with_stdin(r#"1
3
1 2 3
3
2 3 5
"#)
        .tee_output()
        .expect_success();
    assert_eq!(output.stdout_str(), "no\n");
    assert!(output.stderr_str().is_empty());
}

#[test]
fn sample3() {
    let testdir = TestDir::new(BIN, "");
    let output = testdir
        .cmd()
        .output_with_stdin(r#"1
3
1 2 3
10
1 2 3 4 5 6 7 8 9 10
"#)
        .tee_output()
        .expect_success();
    assert_eq!(output.stdout_str(), "no\n");
    assert!(output.stderr_str().is_empty());
}

#[test]
fn sample4() {
    let testdir = TestDir::new(BIN, "");
    let output = testdir
        .cmd()
        .output_with_stdin(r#"1
3
1 2 3
3
1 2 2
"#)
        .tee_output()
        .expect_success();
    assert_eq!(output.stdout_str(), "no\n");
    assert!(output.stderr_str().is_empty());
}

#[test]
fn sample5() {
    let testdir = TestDir::new(BIN, "");
    let output = testdir
        .cmd()
        .output_with_stdin(r#"2
5
1 3 6 10 15
3
4 8 16
"#)
        .tee_output()
        .expect_success();
    assert_eq!(output.stdout_str(), "yes\n");
    assert!(output.stderr_str().is_empty());
}
