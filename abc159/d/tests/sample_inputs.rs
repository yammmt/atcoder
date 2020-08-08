use cli_test_dir::*;

const BIN: &'static str = "./main";

#[test]
fn sample1() {
    let testdir = TestDir::new(BIN, "");
    let output = testdir
        .cmd()
        .output_with_stdin(r#"5
1 1 2 1 2
"#)
        .tee_output()
        .expect_success();
    assert_eq!(output.stdout_str(), r#"2
2
3
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
        .output_with_stdin(r#"4
1 2 3 4
"#)
        .tee_output()
        .expect_success();
    assert_eq!(output.stdout_str(), r#"0
0
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
        .output_with_stdin(r#"5
3 3 3 3 3
"#)
        .tee_output()
        .expect_success();
    assert_eq!(output.stdout_str(), r#"6
6
6
6
6
"#);
    assert!(output.stderr_str().is_empty());
}

#[test]
fn sample4() {
    let testdir = TestDir::new(BIN, "");
    let output = testdir
        .cmd()
        .output_with_stdin(r#"8
1 2 1 4 2 1 4 1
"#)
        .tee_output()
        .expect_success();
    assert_eq!(output.stdout_str(), r#"5
7
5
7
7
5
7
5
"#);
    assert!(output.stderr_str().is_empty());
}
