use cli_test_dir::*;

const BIN: &'static str = "./main";

#[test]
fn sample1() {
    let testdir = TestDir::new(BIN, "");
    let output = testdir
        .cmd()
        .output_with_stdin(r#"3
2 6 6
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
        .output_with_stdin(r#"15
546 3192 1932 630 2100 4116 3906 3234 1302 1806 3528 3780 252 1008 588
"#)
        .tee_output()
        .expect_success();
    assert_eq!(output.stdout_str(), "42\n");
    assert!(output.stderr_str().is_empty());
}
