use cli_test_dir::*;

const BIN: &'static str = "./main";

#[test]
fn sample1() {
    let testdir = TestDir::new(BIN, "");
    let output = testdir
        .cmd()
        .output_with_stdin(r#"3
3 4 6
"#)
        .tee_output()
        .expect_success();
    assert_eq!(output.stdout_str(), "10\n");
    assert!(output.stderr_str().is_empty());
}

#[test]
fn sample2() {
    let testdir = TestDir::new(BIN, "");
    let output = testdir
        .cmd()
        .output_with_stdin(r#"5
7 46 11 20 11
"#)
        .tee_output()
        .expect_success();
    assert_eq!(output.stdout_str(), "90\n");
    assert!(output.stderr_str().is_empty());
}

#[test]
fn sample3() {
    let testdir = TestDir::new(BIN, "");
    let output = testdir
        .cmd()
        .output_with_stdin(r#"7
994 518 941 851 647 2 581
"#)
        .tee_output()
        .expect_success();
    assert_eq!(output.stdout_str(), "4527\n");
    assert!(output.stderr_str().is_empty());
}

