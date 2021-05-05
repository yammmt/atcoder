use cli_test_dir::*;

const BIN: &'static str = "./main";

#[test]
fn sample1() {
    let testdir = TestDir::new(BIN, "");
    let output = testdir
        .cmd()
        .output_with_stdin(r#"6 3
8 1
6 2
9 3
1 1
2 2
1 3
4 3
2 1
1 2
"#)
        .tee_output()
        .expect_success();
    assert_eq!(output.stdout_str(), r#"6
2
6
"#);
    assert!(output.stderr_str().is_empty());
}

#[test]
fn sample2() {
    let testdir = TestDir::new(BIN, "");
    let output = testdir
        .cmd()
        .output_with_stdin(r#"2 2
4208 1234
3056 5678
1 2020
2 2020
"#)
        .tee_output()
        .expect_success();
    assert_eq!(output.stdout_str(), r#"3056
4208
"#);
    assert!(output.stderr_str().is_empty());
}
