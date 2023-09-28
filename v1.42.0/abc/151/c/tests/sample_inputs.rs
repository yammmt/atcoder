use cli_test_dir::*;

const BIN: &'static str = "./main";

#[test]
fn sample1() {
    let testdir = TestDir::new(BIN, "");
    let output = testdir
        .cmd()
        .output_with_stdin(r#"2 5
1 WA
1 AC
2 WA
2 AC
2 WA
"#)
        .tee_output()
        .expect_success();
    assert_eq!(output.stdout_str(), "2 2\n");
    assert!(output.stderr_str().is_empty());
}

#[test]
fn sample2() {
    let testdir = TestDir::new(BIN, "");
    let output = testdir
        .cmd()
        .output_with_stdin(r#"100000 3
7777 AC
7777 AC
7777 AC
"#)
        .tee_output()
        .expect_success();
    assert_eq!(output.stdout_str(), "1 0\n");
    assert!(output.stderr_str().is_empty());
}

#[test]
fn sample3() {
    let testdir = TestDir::new(BIN, "");
    let output = testdir
        .cmd()
        .output_with_stdin(r#"6 0
"#)
        .tee_output()
        .expect_success();
    assert_eq!(output.stdout_str(), "0 0\n");
    assert!(output.stderr_str().is_empty());
}

