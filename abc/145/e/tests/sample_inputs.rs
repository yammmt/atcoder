use cli_test_dir::*;

const BIN: &'static str = "./main";

#[test]
fn sample1() {
    let testdir = TestDir::new(BIN, "");
    let output = testdir
        .cmd()
        .output_with_stdin(r#"2 60
10 10
100 100
"#)
        .tee_output()
        .expect_success();
    assert_eq!(output.stdout_str(), "110\n");
    assert!(output.stderr_str().is_empty());
}

#[test]
fn sample2() {
    let testdir = TestDir::new(BIN, "");
    let output = testdir
        .cmd()
        .output_with_stdin(r#"3 60
10 10
10 20
10 30
"#)
        .tee_output()
        .expect_success();
    assert_eq!(output.stdout_str(), "60\n");
    assert!(output.stderr_str().is_empty());
}

#[test]
fn sample3() {
    let testdir = TestDir::new(BIN, "");
    let output = testdir
        .cmd()
        .output_with_stdin(r#"3 60
30 10
30 20
30 30
"#)
        .tee_output()
        .expect_success();
    assert_eq!(output.stdout_str(), "50\n");
    assert!(output.stderr_str().is_empty());
}

#[test]
fn sample4() {
    let testdir = TestDir::new(BIN, "");
    let output = testdir
        .cmd()
        .output_with_stdin(r#"10 100
15 23
20 18
13 17
24 12
18 29
19 27
23 21
18 20
27 15
22 25
"#)
        .tee_output()
        .expect_success();
    assert_eq!(output.stdout_str(), "145\n");
    assert!(output.stderr_str().is_empty());
}
