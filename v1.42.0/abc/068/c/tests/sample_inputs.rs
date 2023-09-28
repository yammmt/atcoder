use cli_test_dir::*;

const BIN: &'static str = "./main";

#[test]
fn sample1() {
    let testdir = TestDir::new(BIN, "");
    let output = testdir
        .cmd()
        .output_with_stdin(r#"3 2
1 2
2 3
"#)
        .tee_output()
        .expect_success();
    assert_eq!(output.stdout_str(), "POSSIBLE\n");
    assert!(output.stderr_str().is_empty());
}

#[test]
fn sample2() {
    let testdir = TestDir::new(BIN, "");
    let output = testdir
        .cmd()
        .output_with_stdin(r#"4 3
1 2
2 3
3 4
"#)
        .tee_output()
        .expect_success();
    assert_eq!(output.stdout_str(), "IMPOSSIBLE\n");
    assert!(output.stderr_str().is_empty());
}

#[test]
fn sample3() {
    let testdir = TestDir::new(BIN, "");
    let output = testdir
        .cmd()
        .output_with_stdin(r#"100000 1
1 99999
"#)
        .tee_output()
        .expect_success();
    assert_eq!(output.stdout_str(), "IMPOSSIBLE\n");
    assert!(output.stderr_str().is_empty());
}

#[test]
fn sample4() {
    let testdir = TestDir::new(BIN, "");
    let output = testdir
        .cmd()
        .output_with_stdin(r#"5 5
1 3
4 5
2 3
2 4
1 4
"#)
        .tee_output()
        .expect_success();
    assert_eq!(output.stdout_str(), "POSSIBLE\n");
    assert!(output.stderr_str().is_empty());
}
