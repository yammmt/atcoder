use cli_test_dir::*;

const BIN: &'static str = "./main";

#[test]
fn sample1() {
    let testdir = TestDir::new(BIN, "");
    let output = testdir
        .cmd()
        .output_with_stdin(r#"3
2 1 4
2
1 1
2 3
"#)
        .tee_output()
        .expect_success();
    assert_eq!(output.stdout_str(), r#"6
9
"#);
    assert!(output.stderr_str().is_empty());
}

#[test]
fn sample2() {
    let testdir = TestDir::new(BIN, "");
    let output = testdir
        .cmd()
        .output_with_stdin(r#"5
7 2 3 8 5
3
4 2
1 7
4 13
"#)
        .tee_output()
        .expect_success();
    assert_eq!(output.stdout_str(), r#"19
25
30
"#);
    assert!(output.stderr_str().is_empty());
}
