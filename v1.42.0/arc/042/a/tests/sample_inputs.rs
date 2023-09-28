use cli_test_dir::*;

const BIN: &'static str = "./main";

#[test]
fn sample1() {
    let testdir = TestDir::new(BIN, "");
    let output = testdir
        .cmd()
        .output_with_stdin(r#"3 3
2
3
1
"#)
        .tee_output()
        .expect_success();
    assert_eq!(output.stdout_str(), r#"1
3
2
"#);
    assert!(output.stderr_str().is_empty());
}

#[test]
fn sample2() {
    let testdir = TestDir::new(BIN, "");
    let output = testdir
        .cmd()
        .output_with_stdin(r#"3 3
1
1
1
"#)
        .tee_output()
        .expect_success();
    assert_eq!(output.stdout_str(), r#"1
2
3
"#);
    assert!(output.stderr_str().is_empty());
}

#[test]
fn sample3() {
    let testdir = TestDir::new(BIN, "");
    let output = testdir
        .cmd()
        .output_with_stdin(r#"10 10
3
1
4
1
5
9
2
6
5
3
"#)
        .tee_output()
        .expect_success();
    assert_eq!(output.stdout_str(), r#"3
5
6
2
9
1
4
7
8
10
"#);
    assert!(output.stderr_str().is_empty());
}

