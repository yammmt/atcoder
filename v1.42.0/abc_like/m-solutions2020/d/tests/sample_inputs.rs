use cli_test_dir::*;

const BIN: &'static str = "./main";

#[test]
fn sample1() {
    let testdir = TestDir::new(BIN, "");
    let output = testdir
        .cmd()
        .output_with_stdin(r#"7
100 130 130 130 115 115 150
"#)
        .tee_output()
        .expect_success();
    assert_eq!(output.stdout_str(), "1685\n");
    assert!(output.stderr_str().is_empty());
}

#[test]
fn sample2() {
    let testdir = TestDir::new(BIN, "");
    let output = testdir
        .cmd()
        .output_with_stdin(r#"6
200 180 160 140 120 100
"#)
        .tee_output()
        .expect_success();
    assert_eq!(output.stdout_str(), "1000\n");
    assert!(output.stderr_str().is_empty());
}

#[test]
fn sample3() {
    let testdir = TestDir::new(BIN, "");
    let output = testdir
        .cmd()
        .output_with_stdin(r#"2
157 193
"#)
        .tee_output()
        .expect_success();
    assert_eq!(output.stdout_str(), "1216\n");
    assert!(output.stderr_str().is_empty());
}

