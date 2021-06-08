use cli_test_dir::*;

const BIN: &'static str = "./main";

#[test]
fn sample1() {
    let testdir = TestDir::new(BIN, "");
    let output = testdir
        .cmd()
        .output_with_stdin(r#"5 7
1 2 2
1 4 1
2 3 7
1 5 12
3 5 2
2 5 3
3 4 5
"#)
        .tee_output()
        .expect_success();
    assert_eq!(output.stdout_str(), "13\n");
    assert!(output.stderr_str().is_empty());
}

#[test]
fn sample2() {
    let testdir = TestDir::new(BIN, "");
    let output = testdir
        .cmd()
        .output_with_stdin(r#"5 4
1 2 1
1 3 1
1 4 1
1 5 1
"#)
        .tee_output()
        .expect_success();
    assert_eq!(output.stdout_str(), "-1\n");
    assert!(output.stderr_str().is_empty());
}

#[test]
fn sample3() {
    let testdir = TestDir::new(BIN, "");
    let output = testdir
        .cmd()
        .output_with_stdin(r#"10 12
1 4 3
1 9 1
2 5 4
2 6 1
3 7 5
3 10 9
4 7 2
5 6 6
5 8 5
6 8 3
7 9 5
8 10 8
"#)
        .tee_output()
        .expect_success();
    assert_eq!(output.stdout_str(), "11\n");
    assert!(output.stderr_str().is_empty());
}

