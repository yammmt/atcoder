use cli_test_dir::*;

const BIN: &'static str = "./main";

#[test]
fn sample1() {
    let testdir = TestDir::new(BIN, "");
    let output = testdir
        .cmd()
        .output_with_stdin(r#"4 4 1
2 1
1 3
3 2
3 4
4 1
"#)
        .tee_output()
        .expect_success();
    assert_eq!(output.stdout_str(), "0 1 0 1\n");
    assert!(output.stderr_str().is_empty());
}

#[test]
fn sample2() {
    let testdir = TestDir::new(BIN, "");
    let output = testdir
        .cmd()
        .output_with_stdin(r#"5 10 0
1 2
1 3
1 4
1 5
3 2
2 4
2 5
4 3
5 3
4 5
"#)
        .tee_output()
        .expect_success();
    assert_eq!(output.stdout_str(), "0 0 0 0 0\n");
    assert!(output.stderr_str().is_empty());
}

#[test]
fn sample3() {
    let testdir = TestDir::new(BIN, "");
    let output = testdir
        .cmd()
        .output_with_stdin(r#"10 9 3
10 1
6 7
8 2
2 5
8 4
7 3
10 9
6 4
5 8
2 6
7 5
3 1
"#)
        .tee_output()
        .expect_success();
    assert_eq!(output.stdout_str(), "1 3 5 4 3 3 3 3 1 0\n");
    assert!(output.stderr_str().is_empty());
}

