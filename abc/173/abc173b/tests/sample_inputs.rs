use cli_test_dir::*;

const BIN: &'static str = "./main";

#[test]
fn sample1() {
    let testdir = TestDir::new(BIN, "");
    let output = testdir
        .cmd()
        .output_with_stdin(r#"6
AC
TLE
AC
AC
WA
TLE
"#)
        .tee_output()
        .expect_success();
    assert_eq!(output.stdout_str(), r#"AC x 3
WA x 1
TLE x 2
RE x 0
"#);
    assert!(output.stderr_str().is_empty());
}

#[test]
fn sample2() {
    let testdir = TestDir::new(BIN, "");
    let output = testdir
        .cmd()
        .output_with_stdin(r#"10
AC
AC
AC
AC
AC
AC
AC
AC
AC
AC
"#)
        .tee_output()
        .expect_success();
    assert_eq!(output.stdout_str(), r#"AC x 10
WA x 0
TLE x 0
RE x 0
"#);
    assert!(output.stderr_str().is_empty());
}
