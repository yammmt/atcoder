use cli_test_dir::*;

const BIN: &'static str = "./main";

#[test]
fn sample1() {
    let testdir = TestDir::new(BIN, "");
    let output = testdir
        .cmd()
        .output_with_stdin(r#"a
4
2 1 p
1
2 2 c
1
"#)
        .tee_output()
        .expect_success();
    assert_eq!(output.stdout_str(), "cpa\n");
    assert!(output.stderr_str().is_empty());
}

#[test]
fn sample2() {
    let testdir = TestDir::new(BIN, "");
    let output = testdir
        .cmd()
        .output_with_stdin(r#"a
6
2 2 a
2 1 b
1
2 2 c
1
1
"#)
        .tee_output()
        .expect_success();
    assert_eq!(output.stdout_str(), "aabc\n");
    assert!(output.stderr_str().is_empty());
}

#[test]
fn sample3() {
    let testdir = TestDir::new(BIN, "");
    let output = testdir
        .cmd()
        .output_with_stdin(r#"y
1
2 1 x
"#)
        .tee_output()
        .expect_success();
    assert_eq!(output.stdout_str(), "xy\n");
    assert!(output.stderr_str().is_empty());
}


#[test]
fn sample4() {
    let testdir = TestDir::new(BIN, "");
    let output = testdir
        .cmd()
        .output_with_stdin(r#"a
5
2 1 p
1
2 2 c
1
1
"#)
        .tee_output()
        .expect_success();
    assert_eq!(output.stdout_str(), "apc\n");
    assert!(output.stderr_str().is_empty());
}


#[test]
fn sample5() {
    let testdir = TestDir::new(BIN, "");
    let output = testdir
        .cmd()
        .output_with_stdin(r#"a
4
2 2 p
1
2 2 c
1
"#)
        .tee_output()
        .expect_success();
    assert_eq!(output.stdout_str(), "cap\n");
    assert!(output.stderr_str().is_empty());
}
