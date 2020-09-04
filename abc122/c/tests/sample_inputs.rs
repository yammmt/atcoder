use cli_test_dir::*;

const BIN: &'static str = "./main";

#[test]
fn sample1() {
    let testdir = TestDir::new(BIN, "");
    let output = testdir
        .cmd()
        .output_with_stdin(r#"8 3
ACACTACG
3 7
2 3
1 8
"#)
        .tee_output()
        .expect_success();
    assert_eq!(output.stdout_str(), r#"2
0
3
"#);
    assert!(output.stderr_str().is_empty());
}
