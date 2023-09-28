use cli_test_dir::*;

const BIN: &'static str = "./main";

#[test]
fn sample1() {
    let testdir = TestDir::new(BIN, "");
    let output = testdir
        .cmd()
        .output_with_stdin(r#"3 2
1 3 5
1 2
"#)
        .tee_output()
        .expect_success();
    assert_eq!(output.stdout_str(), "0.2500000000\n");
    assert!(output.stderr_str().is_empty());
}

#[test]
fn sample2() {
    let testdir = TestDir::new(BIN, "");
    let output = testdir
        .cmd()
        .output_with_stdin(r#"9 10
11 2 33 4 55 6 77 8 99
10 11 14 19 55 1000000000 4 5 7 8
"#)
        .tee_output()
        .expect_success();
    assert_eq!(output.stdout_str(), "0.2666666667\n");
    assert!(output.stderr_str().is_empty());
}
