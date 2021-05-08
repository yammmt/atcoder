use cli_test_dir::*;

const BIN: &'static str = "./main";

#[test]
fn sample1() {
    let testdir = TestDir::new(BIN, "");
    let output = testdir
        .cmd()
        .output_with_stdin(r#"4 2
1 2 -3 -4
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
        .output_with_stdin(r#"4 3
-1 -2 -3 -4
"#)
        .tee_output()
        .expect_success();
    assert_eq!(output.stdout_str(), "1000000001\n");
    assert!(output.stderr_str().is_empty());
}

#[test]
fn sample3() {
    let testdir = TestDir::new(BIN, "");
    let output = testdir
        .cmd()
        .output_with_stdin(r#"2 1
-1 1000000000
"#)
        .tee_output()
        .expect_success();
    assert_eq!(output.stdout_str(), "1000000000\n");
    assert!(output.stderr_str().is_empty());
}

#[test]
fn sample4() {
    let testdir = TestDir::new(BIN, "");
    let output = testdir
        .cmd()
        .output_with_stdin(r#"10 10
1000000000 100000000 10000000 1000000 100000 10000 1000 100 10 1
"#)
        .tee_output()
        .expect_success();
    assert_eq!(output.stdout_str(), "999983200\n");
    assert!(output.stderr_str().is_empty());
}

#[test]
fn sample5() {
    let testdir = TestDir::new(BIN, "");
    let output = testdir
        .cmd()
        .output_with_stdin(r#"6 2
1 3 5 -3 -2 10
"#)
        .tee_output()
        .expect_success();
    assert_eq!(output.stdout_str(), "50\n");
    assert!(output.stderr_str().is_empty());
}

#[test]
fn sample6() {
    let testdir = TestDir::new(BIN, "");
    let output = testdir
        .cmd()
        .output_with_stdin(r#"6 2
-1 -3 -5 3 2 -10
"#)
        .tee_output()
        .expect_success();
    assert_eq!(output.stdout_str(), "50\n");
    assert!(output.stderr_str().is_empty());
}
