use cli_test_dir::*;

const BIN: &'static str = "./main";

#[test]
fn sample1() {
    let testdir = TestDir::new(BIN, "");
    let output = testdir
        .cmd()
        .output_with_stdin(
            r#"0 0
4
100 100
-100 100
-100 -100
100 -100
"#,
        )
        .tee_output()
        .expect_success();
    assert_eq!(output.stdout_str(), "100\n");
    assert!(output.stderr_str().is_empty());
}

#[test]
fn sample2() {
    let testdir = TestDir::new(BIN, "");
    let output = testdir
        .cmd()
        .output_with_stdin(
            r#"10 10
3
0 100
-100 -100
100 -100
"#,
        )
        .tee_output()
        .expect_success();
    assert_eq!(output.stdout_str(), "31.3049516850\n");
    assert!(output.stderr_str().is_empty());
}

#[test]
fn sample3() {
    let testdir = TestDir::new(BIN, "");
    let output = testdir
        .cmd()
        .output_with_stdin(
            r#"34 6
7
-43 -65
-23 -99
54 -68
65 92
16 83
-18 43
-39 2
"#,
        )
        .tee_output()
        .expect_success();
    assert_eq!(output.stdout_str(), "25.0284205314\n");
    assert!(output.stderr_str().is_empty());
}
