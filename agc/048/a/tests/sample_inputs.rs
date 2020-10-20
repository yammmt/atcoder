use cli_test_dir::*;

const BIN: &'static str = "./main";

#[test]
fn sample1() {
    let testdir = TestDir::new(BIN, "");
    let output = testdir
        .cmd()
        .output_with_stdin(r#"3
atcodeer
codeforces
aaa
"#)
        .tee_output()
        .expect_success();
    assert_eq!(output.stdout_str(), r#"1
0
-1
"#);
    assert!(output.stderr_str().is_empty());
}

#[test]
fn sample2() {
    let testdir = TestDir::new(BIN, "");
    let output = testdir
        .cmd()
        .output_with_stdin(r#"2
acotder
atcocf
"#)
        .tee_output()
        .expect_success();
    assert_eq!(output.stdout_str(), r#"1
1
"#);
    assert!(output.stderr_str().is_empty());
}


#[test]
fn sample3() {
    let testdir = TestDir::new(BIN, "");
    let output = testdir
        .cmd()
        .output_with_stdin(r#"1
atcoder
"#)
        .tee_output()
        .expect_success();
    assert_eq!(output.stdout_str(), r#"1
"#);
    assert!(output.stderr_str().is_empty());
}

#[test]
fn sample4() {
    let testdir = TestDir::new(BIN, "");
    let output = testdir
        .cmd()
        .output_with_stdin(r#"1
acto
"#)
        .tee_output()
        .expect_success();
    assert_eq!(output.stdout_str(), r#"1
"#);
    assert!(output.stderr_str().is_empty());
}

#[test]
fn sample5() {
    let testdir = TestDir::new(BIN, "");
    let output = testdir
        .cmd()
        .output_with_stdin(r#"1
atcodera
"#)
        .tee_output()
        .expect_success();
    assert_eq!(output.stdout_str(), r#"0
"#);
    assert!(output.stderr_str().is_empty());
}
