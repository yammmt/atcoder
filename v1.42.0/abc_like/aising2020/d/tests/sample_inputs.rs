use cli_test_dir::*;

const BIN: &'static str = "./main";

#[test]
fn sample1() {
    let testdir = TestDir::new(BIN, "");
    let output = testdir
        .cmd()
        .output_with_stdin(r#"3
011
"#)
        .tee_output()
        .expect_success();
    assert_eq!(output.stdout_str(), r#"2
1
1
"#);
    assert!(output.stderr_str().is_empty());
}

#[test]
fn sample2() {
    let testdir = TestDir::new(BIN, "");
    let output = testdir
        .cmd()
        .output_with_stdin(r#"23
00110111001011011001110
"#)
        .tee_output()
        .expect_success();
    assert_eq!(output.stdout_str(), r#"2
1
2
2
1
2
2
2
2
2
2
2
2
2
2
2
2
2
2
2
2
1
3
"#);
    assert!(output.stderr_str().is_empty());
}
