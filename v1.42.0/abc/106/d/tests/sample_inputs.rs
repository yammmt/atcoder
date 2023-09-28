use cli_test_dir::*;

const BIN: &'static str = "./main";

#[test]
fn sample1() {
    let testdir = TestDir::new(BIN, "");
    let output = testdir
        .cmd()
        .output_with_stdin(r#"2 3 1
1 1
1 2
2 2
1 2
"#)
        .tee_output()
        .expect_success();
    assert_eq!(output.stdout_str(), "3\n");
    assert!(output.stderr_str().is_empty());
}

#[test]
fn sample2() {
    let testdir = TestDir::new(BIN, "");
    let output = testdir
        .cmd()
        .output_with_stdin(r#"10 3 2
1 5
2 8
7 10
1 7
3 10
"#)
        .tee_output()
        .expect_success();
    assert_eq!(output.stdout_str(), r#"1
1
"#);
    assert!(output.stderr_str().is_empty());
}

#[test]
fn sample3() {
    let testdir = TestDir::new(BIN, "");
    let output = testdir
        .cmd()
        .output_with_stdin(r#"10 10 10
1 6
2 9
4 5
4 7
4 7
5 8
6 6
6 7
7 9
10 10
1 8
1 9
1 10
2 8
2 9
2 10
3 8
3 9
3 10
1 10
"#)
        .tee_output()
        .expect_success();
    assert_eq!(output.stdout_str(), r#"7
9
10
6
8
9
6
7
8
10
"#);
    assert!(output.stderr_str().is_empty());
}

