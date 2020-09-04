use cli_test_dir::*;

const BIN: &'static str = "./main";

#[test]
fn sample1() {
    let testdir = TestDir::new(BIN, "");
    let output = testdir
        .cmd()
        .output_with_stdin(r#"5 3
50 100 80 120 80
"#)
        .tee_output()
        .expect_success();
    assert_eq!(output.stdout_str(), "210\n");
    assert!(output.stderr_str().is_empty());
}

#[test]
fn sample2() {
    let testdir = TestDir::new(BIN, "");
    let output = testdir
        .cmd()
        .output_with_stdin(r#"1 1
1000
"#)
        .tee_output()
        .expect_success();
    assert_eq!(output.stdout_str(), "1000\n");
    assert!(output.stderr_str().is_empty());
}
