use cli_test_dir::*;

const BIN: &'static str = "./main";

#[test]
fn sample1() {
    let testdir = TestDir::new(BIN, "");
    let output = testdir
        .cmd()
        .output_with_stdin(r#"5 4
1 4
2 5
3 3
1 5
"#)
        .tee_output()
        .expect_success();
    assert_eq!(output.stdout_str(), "01010\n");
    assert!(output.stderr_str().is_empty());
}

#[test]
fn sample2() {
    let testdir = TestDir::new(BIN, "");
    let output = testdir
        .cmd()
        .output_with_stdin(r#"20 8
1 8
4 13
8 8
3 18
5 20
19 20
2 7
4 9
"#)
        .tee_output()
        .expect_success();
    assert_eq!(output.stdout_str(), "10110000011110000000\n");
    assert!(output.stderr_str().is_empty());
}
