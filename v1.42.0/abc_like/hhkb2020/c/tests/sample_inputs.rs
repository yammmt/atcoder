use cli_test_dir::*;

const BIN: &'static str = "./main";

#[test]
fn sample1() {
    let testdir = TestDir::new(BIN, "");
    let output = testdir
        .cmd()
        .output_with_stdin(r#"4
1 1 0 2
"#)
        .tee_output()
        .expect_success();
    assert_eq!(output.stdout_str(), r#"0
0
2
3
"#);
    assert!(output.stderr_str().is_empty());
}

#[test]
fn sample2() {
    let testdir = TestDir::new(BIN, "");
    let output = testdir
        .cmd()
        .output_with_stdin(r#"10
5 4 3 2 1 0 7 7 6 6
"#)
        .tee_output()
        .expect_success();
    assert_eq!(output.stdout_str(), r#"0
0
0
0
0
6
6
6
8
8
"#);
    assert!(output.stderr_str().is_empty());
}
