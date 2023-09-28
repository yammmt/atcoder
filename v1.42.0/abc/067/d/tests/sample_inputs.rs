use cli_test_dir::*;

const BIN: &'static str = "./main";

#[test]
fn sample1() {
    let testdir = TestDir::new(BIN, "");
    let output = testdir
        .cmd()
        .output_with_stdin(r#"7
3 6
1 2
3 1
7 4
5 7
1 4
"#)
        .tee_output()
        .expect_success();
    assert_eq!(output.stdout_str(), "Fennec\n");
    assert!(output.stderr_str().is_empty());
}

#[test]
fn sample2() {
    let testdir = TestDir::new(BIN, "");
    let output = testdir
        .cmd()
        .output_with_stdin(r#"4
1 4
4 2
2 3
"#)
        .tee_output()
        .expect_success();
    assert_eq!(output.stdout_str(), "Snuke\n");
    assert!(output.stderr_str().is_empty());
}
