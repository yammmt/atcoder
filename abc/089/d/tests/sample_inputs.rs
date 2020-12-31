use cli_test_dir::*;

const BIN: &'static str = "./main";

#[test]
fn sample1() {
    let testdir = TestDir::new(BIN, "");
    let output = testdir
        .cmd()
        .output_with_stdin(r#"3 3 2
1 4 3
2 5 7
8 9 6
1
4 8
"#)
        .tee_output()
        .expect_success();
    assert_eq!(output.stdout_str(), "5\n");
    assert!(output.stderr_str().is_empty());
}

#[test]
fn sample2() {
    let testdir = TestDir::new(BIN, "");
    let output = testdir
        .cmd()
        .output_with_stdin(r#"4 2 3
3 7
1 4
5 2
6 8
2
2 2
2 2
"#)
        .tee_output()
        .expect_success();
    assert_eq!(output.stdout_str(), r#"0
0
"#);
    assert!(output.stderr_str().is_empty());
}

#[test]
fn sample3() {
    let testdir = TestDir::new(BIN, "");
    let output = testdir
        .cmd()
        .output_with_stdin(r#"5 5 4
13 25 7 15 17
16 22 20 2 9
14 11 12 1 19
10 6 23 8 18
3 21 5 24 4
3
13 13
2 10
13 13
"#)
        .tee_output()
        .expect_success();
        assert_eq!(output.stdout_str(), r#"0
5
0
"#);
    assert!(output.stderr_str().is_empty());
}

