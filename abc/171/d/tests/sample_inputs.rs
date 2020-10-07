use cli_test_dir::*;

const BIN: &'static str = "./main";

#[test]
fn sample1() {
    let testdir = TestDir::new(BIN, "");
    let output = testdir
        .cmd()
        .output_with_stdin(r#"4
1 2 3 4
3
1 2
3 4
2 4
"#)
        .tee_output()
        .expect_success();
    assert_eq!(output.stdout_str(), r#"11
12
16
"#);
    assert!(output.stderr_str().is_empty());
}

#[test]
fn sample2() {
    let testdir = TestDir::new(BIN, "");
    let output = testdir
        .cmd()
        .output_with_stdin(r#"4
1 1 1 1
3
1 2
2 1
3 5
"#)
        .tee_output()
        .expect_success();
    assert_eq!(output.stdout_str(), r#"8
4
4
"#);
    assert!(output.stderr_str().is_empty());
}

#[test]
fn sample3() {
    let testdir = TestDir::new(BIN, "");
    let output = testdir
        .cmd()
        .output_with_stdin(r#"2
1 2
3
1 100
2 100
100 1000
"#)
        .tee_output()
        .expect_success();
    assert_eq!(output.stdout_str(), r#"102
200
2000
"#);
    assert!(output.stderr_str().is_empty());
}

