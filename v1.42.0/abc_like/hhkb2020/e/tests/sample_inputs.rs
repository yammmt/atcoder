use cli_test_dir::*;

const BIN: &'static str = "./main";

#[test]
fn sample1() {
    let testdir = TestDir::new(BIN, "");
    let output = testdir
        .cmd()
        .output_with_stdin(r#"1 5
..#..
"#)
        .tee_output()
        .expect_success();
    assert_eq!(output.stdout_str(), "48\n");
    assert!(output.stderr_str().is_empty());
}

#[test]
fn sample2() {
    let testdir = TestDir::new(BIN, "");
    let output = testdir
        .cmd()
        .output_with_stdin(r#"2 3
..#
#..
"#)
        .tee_output()
        .expect_success();
    assert_eq!(output.stdout_str(), "52\n");
    assert!(output.stderr_str().is_empty());
}

#[test]
fn sample3() {
    let testdir = TestDir::new(BIN, "");
    let output = testdir
        .cmd()
        .output_with_stdin(r#"1 7
..#..##
"#)
        .tee_output()
        .expect_success();
    assert_eq!(output.stdout_str(), "48\n");
    assert!(output.stderr_str().is_empty());
}

#[test]
fn sample4() {
    let testdir = TestDir::new(BIN, "");
    let output = testdir
        .cmd()
        .output_with_stdin(r#"2 6
..#..#
######
"#)
        .tee_output()
        .expect_success();
    assert_eq!(output.stdout_str(), "48\n");
    assert!(output.stderr_str().is_empty());
}
