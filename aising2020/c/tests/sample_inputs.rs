use cli_test_dir::*;

const BIN: &'static str = "./main";

#[test]
fn sample1() {
    let testdir = TestDir::new(BIN, "");
    let output = testdir
        .cmd()
        .output_with_stdin(r#"20
"#)
        .tee_output()
        .expect_success();
    assert_eq!(output.stdout_str(), r#"0
0
0
0
0
1
0
0
0
0
3
0
0
0
0
0
3
3
0
0
"#);
    assert!(output.stderr_str().is_empty());
}
