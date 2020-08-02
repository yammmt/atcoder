use cli_test_dir::*;

const BIN: &'static str = "./main";

#[test]
fn sample1() {
    let testdir = TestDir::new(BIN, "");
    let output = testdir
        .cmd()
        .output_with_stdin(r#"5 3
96 98 95 100 20
"#)
        .tee_output()
        .expect_success();
    assert_eq!(output.stdout_str(), r#"Yes
No
"#);
    assert!(output.stderr_str().is_empty());
}

#[test]
fn sample2() {
    let testdir = TestDir::new(BIN, "");
    let output = testdir
        .cmd()
        .output_with_stdin(r#"3 2
1001 869120 1001
"#)
        .tee_output()
        .expect_success();
    assert_eq!(output.stdout_str(), "No\n");
    assert!(output.stderr_str().is_empty());
}

#[test]
fn sample3() {
    let testdir = TestDir::new(BIN, "");
    let output = testdir
        .cmd()
        .output_with_stdin(r#"15 7
3 1 4 1 5 9 2 6 5 3 5 8 9 7 9
"#)
        .tee_output()
        .expect_success();
    assert_eq!(output.stdout_str(), r#"Yes
Yes
No
Yes
Yes
No
Yes
Yes
"#);
    assert!(output.stderr_str().is_empty());
}

