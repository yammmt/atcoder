use cli_test_dir::*;

const BIN: &'static str = "./main";

#[test]
fn sample1() {
    let testdir = TestDir::new(BIN, "");
    let output = testdir
        .cmd()
        .output_with_stdin(
            r#"2 2 5
1 3
1 2 2
2 1 1
"#,
        )
        .tee_output()
        .expect_success();
    assert_eq!(output.stdout_str(), "6\n");
    assert!(output.stderr_str().is_empty());
}

#[test]
fn sample2() {
    let testdir = TestDir::new(BIN, "");
    let output = testdir
        .cmd()
        .output_with_stdin(
            r#"2 2 3
1 3
1 2 2
2 1 1
"#,
        )
        .tee_output()
        .expect_success();
    assert_eq!(output.stdout_str(), "3\n");
    assert!(output.stderr_str().is_empty());
}

#[test]
fn sample3() {
    let testdir = TestDir::new(BIN, "");
    let output = testdir
        .cmd()
        .output_with_stdin(
            r#"8 15 120
1 2 6 16 1 3 11 9
1 8 1
7 3 14
8 2 13
3 5 4
5 7 5
6 4 1
6 8 17
7 8 5
1 4 2
4 7 1
6 1 3
3 1 10
2 6 5
2 4 12
5 1 30
"#,
        )
        .tee_output()
        .expect_success();
    assert_eq!(output.stdout_str(), "1488\n");
    assert!(output.stderr_str().is_empty());
}
