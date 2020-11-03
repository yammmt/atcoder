use cli_test_dir::*;

const BIN: &'static str = "./main";

#[test]
fn sample1() {
    let testdir = TestDir::new(BIN, "");
    let output = testdir
        .cmd()
        .output_with_stdin(r#"3 3 2
1 2
5 4
9 2
"#)
        .tee_output()
        .expect_success();
    assert_eq!(output.stdout_str(), "2\n");
    assert!(output.stderr_str().is_empty());
}

#[test]
fn sample2() {
    let testdir = TestDir::new(BIN, "");
    let output = testdir
        .cmd()
        .output_with_stdin(r#"9 4 1
1 5
2 4
3 3
4 2
5 1
6 2
7 3
8 4
9 5
"#)
        .tee_output()
        .expect_success();
    assert_eq!(output.stdout_str(), "5\n");
    assert!(output.stderr_str().is_empty());
}

#[test]
fn sample3() {
    let testdir = TestDir::new(BIN, "");
    let output = testdir
        .cmd()
        .output_with_stdin(r#"3 0 1
300000000 1000000000
100000000 1000000000
200000000 1000000000
"#)
        .tee_output()
        .expect_success();
    assert_eq!(output.stdout_str(), "3000000000\n");
    assert!(output.stderr_str().is_empty());
}

