use cli_test_dir::*;

const BIN: &'static str = "./main";

#[test]
fn sample1() {
    let testdir = TestDir::new(BIN, "");
    let output = testdir
        .cmd()
        .output_with_stdin(r#"7
abcdbbd
6
2 3 6
1 5 z
2 1 1
1 4 a
1 7 d
2 1 7
"#)
        .tee_output()
        .expect_success();
    assert_eq!(output.stdout_str(), r#"3
1
5
"#);
    assert!(output.stderr_str().is_empty());
}