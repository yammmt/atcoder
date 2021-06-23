use cli_test_dir::*;

const BIN: &'static str = "./main";

#[test]
fn sample1() {
    let testdir = TestDir::new(BIN, "");
    let output = testdir
        .cmd()
        .output_with_stdin(
            r#"4 4
1 4
3 3
6 2
8 1
"#,
        )
        .tee_output()
        .expect_success();
    assert_eq!(output.stdout_str(), "21\n");
    assert!(output.stderr_str().is_empty());
}

#[test]
fn sample2() {
    let testdir = TestDir::new(BIN, "");
    let output = testdir
        .cmd()
        .output_with_stdin(
            r#"4 2
0 0
1 1
2 2
3 3
"#,
        )
        .tee_output()
        .expect_success();
    assert_eq!(output.stdout_str(), "1\n");
    assert!(output.stderr_str().is_empty());
}

#[test]
fn sample3() {
    let testdir = TestDir::new(BIN, "");
    let output = testdir
        .cmd()
        .output_with_stdin(
            r#"4 3
-1000000000 -1000000000
1000000000 1000000000
-999999999 999999999
999999999 -999999999
"#,
        )
        .tee_output()
        .expect_success();
    assert_eq!(output.stdout_str(), "3999999996000000001\n");
    assert!(output.stderr_str().is_empty());
}
