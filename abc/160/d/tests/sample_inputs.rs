use cli_test_dir::*;

const BIN: &'static str = "./main";

#[test]
fn sample1() {
    let testdir = TestDir::new(BIN, "");
    let output = testdir
        .cmd()
        .output_with_stdin(r#"5 2 4
"#)
        .tee_output()
        .expect_success();
    assert_eq!(output.stdout_str(), r#"5
4
1
0
"#);
    assert!(output.stderr_str().is_empty());
}

#[test]
fn sample2() {
    let testdir = TestDir::new(BIN, "");
    let output = testdir
        .cmd()
        .output_with_stdin(r#"3 1 3
"#)
        .tee_output()
        .expect_success();
    assert_eq!(output.stdout_str(), r#"3
0
"#);
    assert!(output.stderr_str().is_empty());
}

#[test]
fn sample3() {
    let testdir = TestDir::new(BIN, "");
    let output = testdir
        .cmd()
        .output_with_stdin(r#"7 3 7
"#)
        .tee_output()
        .expect_success();
    assert_eq!(output.stdout_str(), r#"7
8
4
2
0
0
"#);
    assert!(output.stderr_str().is_empty());
}

#[test]
fn sample4() {
    let testdir = TestDir::new(BIN, "");
    let output = testdir
        .cmd()
        .output_with_stdin(r#"10 4 8
"#)
        .tee_output()
        .expect_success();
    assert_eq!(output.stdout_str(), r#"10
12
10
8
4
1
0
0
0
"#);
    assert!(output.stderr_str().is_empty());
}
