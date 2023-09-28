use cli_test_dir::*;

const BIN: &'static str = "./main";

#[test]
fn sample1() {
    let testdir = TestDir::new(BIN, "");
    let output = testdir
        .cmd()
        .output_with_stdin(r#"4
20 11 9 24
"#)
        .tee_output()
        .expect_success();
    assert_eq!(output.stdout_str(), "26 5 7 22\n");
    assert!(output.stderr_str().is_empty());
}
