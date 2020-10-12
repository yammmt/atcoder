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
    assert_eq!(output.stdout_str(), r#"1
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
        .output_with_stdin(r#"3 3
1 2
1 3
2 3
"#)
        .tee_output()
        .expect_success();
    assert_eq!(output.stdout_str(), r#"0
0
0
"#);
    assert!(output.stderr_str().is_empty());
}

#[test]
fn sample3() {
    let testdir = TestDir::new(BIN, "");
    let output = testdir
        .cmd()
        .output_with_stdin(r#"8 12
1 6
1 7
1 8
2 5
2 6
3 5
3 6
4 5
4 8
5 6
5 7
7 8
"#)
        .tee_output()
        .expect_success();
    assert_eq!(output.stdout_str(), r#"4
4
4
5
2
3
4
2
"#);
    assert!(output.stderr_str().is_empty());
}

