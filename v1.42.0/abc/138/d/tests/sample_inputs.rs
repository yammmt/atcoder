use cli_test_dir::*;

const BIN: &'static str = "./main";

#[test]
fn sample1() {
    let testdir = TestDir::new(BIN, "");
    let output = testdir
        .cmd()
        .output_with_stdin(r#"4 3
1 2
2 3
2 4
2 10
1 100
3 1
"#)
        .tee_output()
        .expect_success();
    assert_eq!(output.stdout_str(), "100 110 111 110\n");
    assert!(output.stderr_str().is_empty());
}

#[test]
fn sample2() {
    let testdir = TestDir::new(BIN, "");
    let output = testdir
        .cmd()
        .output_with_stdin(r#"6 2
1 2
1 3
2 4
3 6
2 5
1 10
1 10
"#)
        .tee_output()
        .expect_success();
    assert_eq!(output.stdout_str(), "20 20 20 20 20 20\n");
    assert!(output.stderr_str().is_empty());
}
