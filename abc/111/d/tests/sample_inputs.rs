use cli_test_dir::*;

const BIN: &'static str = "./main";

#[test]
fn sample1() {
    let testdir = TestDir::new(BIN, "");
    let output = testdir
        .cmd()
        .output_with_stdin(r#"3
-1 0
0 3
2 -1
"#)
        .tee_output()
        .expect_success();
    assert_eq!(output.stdout_str(), r#"2
1 2
RL
UU
DR
"#);
    assert!(output.stderr_str().is_empty());
}

#[test]
fn sample2() {
    let testdir = TestDir::new(BIN, "");
    let output = testdir
        .cmd()
        .output_with_stdin(r#"5
0 0
1 0
2 0
3 0
4 0
"#)
        .tee_output()
        .expect_success();
    assert_eq!(output.stdout_str(), "-1\n");
    assert!(output.stderr_str().is_empty());
}

#[test]
fn sample3() {
    let testdir = TestDir::new(BIN, "");
    let output = testdir
        .cmd()
        .output_with_stdin(r#"2
1 1
1 1
"#)
        .tee_output()
        .expect_success();
    assert_eq!(output.stdout_str(), r#"2
1 1
RU
UR
"#);
    assert!(output.stderr_str().is_empty());
}

#[test]
fn sample4() {
    let testdir = TestDir::new(BIN, "");
    let output = testdir
        .cmd()
        .output_with_stdin(r#"3
-7 -3
7 3
-3 -7
"#)
        .tee_output()
        .expect_success();
    assert_eq!(output.stdout_str(), r#"5
3 1 4 1 5
LRDUL
RDULR
DULRD
"#);
    assert!(output.stderr_str().is_empty());
}
