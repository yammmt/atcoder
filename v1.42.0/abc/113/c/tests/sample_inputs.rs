use cli_test_dir::*;

const BIN: &'static str = "./main";

#[test]
fn sample1() {
    let testdir = TestDir::new(BIN, "");
    let output = testdir
        .cmd()
        .output_with_stdin(r#"2 3
1 32
2 63
1 12
"#)
        .tee_output()
        .expect_success();
    assert_eq!(output.stdout_str(), r#"000001000002
000002000001
000001000001
"#);
    assert!(output.stderr_str().is_empty());
}

#[test]
fn sample2() {
    let testdir = TestDir::new(BIN, "");
    let output = testdir
        .cmd()
        .output_with_stdin(r#"2 3
2 55
2 77
2 99
"#)
        .tee_output()
        .expect_success();
    assert_eq!(output.stdout_str(), r#"000002000001
000002000002
000002000003
"#);
    assert!(output.stderr_str().is_empty());
}
