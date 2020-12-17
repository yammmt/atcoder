use cli_test_dir::*;

const BIN: &'static str = "./main";

#[test]
fn sample1() {
    let testdir = TestDir::new(BIN, "");
    let output = testdir
        .cmd()
        .output_with_stdin(r#"5
1 2 1
1 3 1
2 4 1
3 5 1
3 1
2 4
2 3
4 5
"#)
        .tee_output()
        .expect_success();
    assert_eq!(output.stdout_str(), r#"3
2
4
"#);
    assert!(output.stderr_str().is_empty());
}

#[test]
fn sample2() {
    let testdir = TestDir::new(BIN, "");
    let output = testdir
        .cmd()
        .output_with_stdin(r#"7
1 2 1
1 3 3
1 4 5
1 5 7
1 6 9
1 7 11
3 2
1 3
4 5
6 7
"#)
        .tee_output()
        .expect_success();
    assert_eq!(output.stdout_str(), r#"5
14
22
"#);
    assert!(output.stderr_str().is_empty());
}

#[test]
fn sample3() {
    let testdir = TestDir::new(BIN, "");
    let output = testdir
        .cmd()
        .output_with_stdin(r#"10
1 2 1000000000
2 3 1000000000
3 4 1000000000
4 5 1000000000
5 6 1000000000
6 7 1000000000
7 8 1000000000
8 9 1000000000
9 10 1000000000
1 1
9 10
"#)
        .tee_output()
        .expect_success();
    assert_eq!(output.stdout_str(), "17000000000\n");
    assert!(output.stderr_str().is_empty());
}

