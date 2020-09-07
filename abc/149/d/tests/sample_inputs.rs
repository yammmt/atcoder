use cli_test_dir::*;

const BIN: &'static str = "./main";

#[test]
fn sample1() {
    let testdir = TestDir::new(BIN, "");
    let output = testdir
        .cmd()
        .output_with_stdin(r#"5 2
8 7 6
rsrpr
"#)
        .tee_output()
        .expect_success();
    assert_eq!(output.stdout_str(), "27\n");
    assert!(output.stderr_str().is_empty());
}

#[test]
fn sample2() {
    let testdir = TestDir::new(BIN, "");
    let output = testdir
        .cmd()
        .output_with_stdin(r#"7 1
100 10 1
ssssppr
"#)
        .tee_output()
        .expect_success();
    assert_eq!(output.stdout_str(), "211\n");
    assert!(output.stderr_str().is_empty());
}

#[test]
fn sample3() {
    let testdir = TestDir::new(BIN, "");
    let output = testdir
        .cmd()
        .output_with_stdin(r#"30 5
325 234 123
rspsspspsrpspsppprpsprpssprpsr
"#)
        .tee_output()
        .expect_success();
    assert_eq!(output.stdout_str(), "4996\n");
    assert!(output.stderr_str().is_empty());
}

