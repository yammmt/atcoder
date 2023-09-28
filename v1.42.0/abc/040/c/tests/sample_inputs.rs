use cli_test_dir::*;

const BIN: &'static str = "./main";

#[test]
fn sample1() {
    let testdir = TestDir::new(BIN, "");
    let output = testdir
        .cmd()
        .output_with_stdin(r#"4
100 150 130 120
"#)
        .tee_output()
        .expect_success();
    assert_eq!(output.stdout_str(), "40\n");
    assert!(output.stderr_str().is_empty());
}

#[test]
fn sample2() {
    let testdir = TestDir::new(BIN, "");
    let output = testdir
        .cmd()
        .output_with_stdin(r#"4
100 125 80 110
"#)
        .tee_output()
        .expect_success();
    assert_eq!(output.stdout_str(), "40\n");
    assert!(output.stderr_str().is_empty());
}

#[test]
fn sample3() {
    let testdir = TestDir::new(BIN, "");
    let output = testdir
        .cmd()
        .output_with_stdin(r#"9
314 159 265 358 979 323 846 264 338
"#)
        .tee_output()
        .expect_success();
    assert_eq!(output.stdout_str(), "310\n");
    assert!(output.stderr_str().is_empty());
}

