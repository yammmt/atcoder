use cli_test_dir::*;

const BIN: &'static str = "./main";

#[test]
fn sample1() {
    let testdir = TestDir::new(BIN, "");
    let output = testdir
        .cmd()
        .output_with_stdin(r#"3
3 5 -1
"#)
        .tee_output()
        .expect_success();
    assert_eq!(output.stdout_str(), r#"12
8
10
"#);
    assert!(output.stderr_str().is_empty());
}

#[test]
fn sample2() {
    let testdir = TestDir::new(BIN, "");
    let output = testdir
        .cmd()
        .output_with_stdin(r#"5
1 1 1 2 0
"#)
        .tee_output()
        .expect_success();
    assert_eq!(output.stdout_str(), r#"4
4
4
2
4
"#);
    assert!(output.stderr_str().is_empty());
}

#[test]
fn sample3() {
    let testdir = TestDir::new(BIN, "");
    let output = testdir
        .cmd()
        .output_with_stdin(r#"6
-679 -2409 -3258 3095 -3291 -4462
"#)
        .tee_output()
        .expect_success();
    assert_eq!(output.stdout_str(), r#"21630
21630
19932
8924
21630
19288
"#);
    assert!(output.stderr_str().is_empty());
}

#[test]
fn sample4() {
    let testdir = TestDir::new(BIN, "");
    let output = testdir
        .cmd()
        .output_with_stdin(r#"2
-3 4
"#)
        .tee_output()
        .expect_success();
    assert_eq!(output.stdout_str(), r#"8
6
"#);
    assert!(output.stderr_str().is_empty());
}
