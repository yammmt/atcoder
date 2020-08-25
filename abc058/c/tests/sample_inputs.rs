use cli_test_dir::*;

const BIN: &'static str = "./main";

#[test]
fn sample1() {
    let testdir = TestDir::new(BIN, "");
    let output = testdir
        .cmd()
        .output_with_stdin(r#"3
cbaa
daacc
acacac
"#)
        .tee_output()
        .expect_success();
    assert_eq!(output.stdout_str(), "aac\n");
    assert!(output.stderr_str().is_empty());
}

#[test]
fn sample2() {
    let testdir = TestDir::new(BIN, "");
    let output = testdir
        .cmd()
        .output_with_stdin(r#"3
a
aa
b
"#)
        .tee_output()
        .expect_success();
    assert_eq!(output.stdout_str(), "\n");
    assert!(output.stderr_str().is_empty());
}
