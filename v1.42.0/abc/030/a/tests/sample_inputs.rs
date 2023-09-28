use cli_test_dir::*;

const BIN: &'static str = "./main";

#[test]
fn sample1() {
    let testdir = TestDir::new(BIN, "");
    let output = testdir
        .cmd()
        .output_with_stdin(r#"5 2 6 3
"#)
        .tee_output()
        .expect_success();
    assert_eq!(output.stdout_str(), "AOKI\n");
    assert!(output.stderr_str().is_empty());
}

#[test]
fn sample2() {
    let testdir = TestDir::new(BIN, "");
    let output = testdir
        .cmd()
        .output_with_stdin(r#"100 80 100 73
"#)
        .tee_output()
        .expect_success();
    assert_eq!(output.stdout_str(), "TAKAHASHI\n");
    assert!(output.stderr_str().is_empty());
}

#[test]
fn sample3() {
    let testdir = TestDir::new(BIN, "");
    let output = testdir
        .cmd()
        .output_with_stdin(r#"66 30 55 25
"#)
        .tee_output()
        .expect_success();
    assert_eq!(output.stdout_str(), "DRAW\n");
    assert!(output.stderr_str().is_empty());
}

