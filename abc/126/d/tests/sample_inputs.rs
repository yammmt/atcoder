use cli_test_dir::*;

const BIN: &'static str = "./main";

#[test]
fn sample1() {
    let testdir = TestDir::new(BIN, "");
    let output = testdir
        .cmd()
        .output_with_stdin(r#"3
1 2 2
2 3 1
"#)
        .tee_output()
        .expect_success();
    assert_eq!(output.stdout_str(), r#"0
0
1
"#);
    assert!(output.stderr_str().is_empty());
}

#[test]
fn sample2() {
    let testdir = TestDir::new(BIN, "");
    let output = testdir
        .cmd()
        .output_with_stdin(r#"5
2 5 2
2 3 10
1 3 8
3 4 2
"#)
        .tee_output()
        .expect_success();
    assert_eq!(output.stdout_str(), r#"1
0
1
0
1
"#);
    assert!(output.stderr_str().is_empty());
}
