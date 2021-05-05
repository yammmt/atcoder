use cli_test_dir::*;

const BIN: &'static str = "./main";

#[test]
fn sample1() {
    let testdir = TestDir::new(BIN, "");
    let output = testdir
        .cmd()
        .output_with_stdin(r#"2 2 3
1 1 3
2 1 4
1 2 5
"#)
        .tee_output()
        .expect_success();
    assert_eq!(output.stdout_str(), "8\n");
    assert!(output.stderr_str().is_empty());
}

#[test]
fn sample2() {
    let testdir = TestDir::new(BIN, "");
    let output = testdir
        .cmd()
        .output_with_stdin(r#"2 5 5
1 1 3
2 4 20
1 2 1
1 3 4
1 4 2
"#)
        .tee_output()
        .expect_success();
    assert_eq!(output.stdout_str(), "29\n");
    assert!(output.stderr_str().is_empty());
}

#[test]
fn sample3() {
    let testdir = TestDir::new(BIN, "");
    let output = testdir
        .cmd()
        .output_with_stdin(r#"4 5 10
2 5 12
1 5 12
2 3 15
1 2 20
1 1 28
2 4 26
3 2 27
4 5 21
3 5 10
1 3 10
"#)
        .tee_output()
        .expect_success();
    assert_eq!(output.stdout_str(), "142\n");
    assert!(output.stderr_str().is_empty());
}

