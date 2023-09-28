use cli_test_dir::*;

const BIN: &'static str = "./main";

#[test]
fn sample1() {
    let testdir = TestDir::new(BIN, "");
    let output = testdir
        .cmd()
        .output_with_stdin(r#"4
5 6
12 4
14 7
21 2
"#)
        .tee_output()
        .expect_success();
    assert_eq!(output.stdout_str(), "23\n");
    assert!(output.stderr_str().is_empty());
}

#[test]
fn sample2() {
    let testdir = TestDir::new(BIN, "");
    let output = testdir
        .cmd()
        .output_with_stdin(r#"6
100 1
100 1
100 1
100 1
100 1
1 30
"#)
        .tee_output()
        .expect_success();
    assert_eq!(output.stdout_str(), "105\n");
    assert!(output.stderr_str().is_empty());
}
