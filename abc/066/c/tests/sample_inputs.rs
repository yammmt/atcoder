use cli_test_dir::*;

const BIN: &'static str = "./main";

#[test]
fn sample1() {
    let testdir = TestDir::new(BIN, "");
    let output = testdir
        .cmd()
        .output_with_stdin(r#"4
1 2 3 4
"#)
        .tee_output()
        .expect_success();
    assert_eq!(output.stdout_str(), "4 2 1 3\n");
    assert!(output.stderr_str().is_empty());
}

#[test]
fn sample2() {
    let testdir = TestDir::new(BIN, "");
    let output = testdir
        .cmd()
        .output_with_stdin(r#"3
1 2 3
"#)
        .tee_output()
        .expect_success();
    assert_eq!(output.stdout_str(), "3 1 2\n");
    assert!(output.stderr_str().is_empty());
}

#[test]
fn sample3() {
    let testdir = TestDir::new(BIN, "");
    let output = testdir
        .cmd()
        .output_with_stdin(r#"1
1000000000
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
        .output_with_stdin(r#"6
0 6 7 6 7 0
"#)
        .tee_output()
        .expect_success();
    assert_eq!(output.stdout_str(), "0 6 6 0 7 7\n");
    assert!(output.stderr_str().is_empty());
}
