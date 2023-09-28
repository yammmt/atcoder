use cli_test_dir::*;

const BIN: &'static str = "./main";

#[test]
fn sample1() {
    let testdir = TestDir::new(BIN, "");
    let output = testdir
        .cmd()
        .output_with_stdin(r#"3 2 10 20
8 15 13
16 22
"#)
        .tee_output()
        .expect_success();
    assert_eq!(output.stdout_str(), "No War\n");
    assert!(output.stderr_str().is_empty());
}

#[test]
fn sample2() {
    let testdir = TestDir::new(BIN, "");
    let output = testdir
        .cmd()
        .output_with_stdin(r#"4 2 -48 -1
-20 -35 -91 -23
-22 66
"#)
        .tee_output()
        .expect_success();
    assert_eq!(output.stdout_str(), "War\n");
    assert!(output.stderr_str().is_empty());
}

#[test]
fn sample3() {
    let testdir = TestDir::new(BIN, "");
    let output = testdir
        .cmd()
        .output_with_stdin(r#"5 3 6 8
-10 3 1 5 -100
100 6 14
"#)
        .tee_output()
        .expect_success();
    assert_eq!(output.stdout_str(), "War\n");
    assert!(output.stderr_str().is_empty());
}

