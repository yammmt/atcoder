use cli_test_dir::*;

const BIN: &'static str = "./main";

#[test]
fn sample1() {
    let testdir = TestDir::new(BIN, "");
    let output = testdir
        .cmd()
        .output_with_stdin(r#"3
0 0
1 0
0 1
"#)
        .tee_output()
        .expect_success();
    assert_eq!(output.stdout_str(), "2.2761423749\n");
    assert!(output.stderr_str().is_empty());
}

#[test]
fn sample2() {
    let testdir = TestDir::new(BIN, "");
    let output = testdir
        .cmd()
        .output_with_stdin(r#"2
-879 981
-866 890
"#)
        .tee_output()
        .expect_success();
    assert_eq!(output.stdout_str(), "91.9238815543\n");
    assert!(output.stderr_str().is_empty());
}

#[test]
fn sample3() {
    let testdir = TestDir::new(BIN, "");
    let output = testdir
        .cmd()
        .output_with_stdin(r#"8
-406 10
512 859
494 362
-955 -475
128 553
-986 -885
763 77
449 310
"#)
        .tee_output()
        .expect_success();
    assert_eq!(output.stdout_str(), "7641.9817824387\n");
    assert!(output.stderr_str().is_empty());
}

