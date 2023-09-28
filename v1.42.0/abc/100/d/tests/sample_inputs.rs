use cli_test_dir::*;

const BIN: &'static str = "./main";

#[test]
fn sample1() {
    let testdir = TestDir::new(BIN, "");
    let output = testdir
        .cmd()
        .output_with_stdin(
            r#"5 3
3 1 4
1 5 9
2 6 5
3 5 8
9 7 9
"#,
        )
        .tee_output()
        .expect_success();
    assert_eq!(output.stdout_str(), "56\n");
    assert!(output.stderr_str().is_empty());
}

#[test]
fn sample2() {
    let testdir = TestDir::new(BIN, "");
    let output = testdir
        .cmd()
        .output_with_stdin(
            r#"5 3
1 -2 3
-4 5 -6
7 -8 -9
-10 11 -12
13 -14 15
"#,
        )
        .tee_output()
        .expect_success();
    assert_eq!(output.stdout_str(), "54\n");
    assert!(output.stderr_str().is_empty());
}

#[test]
fn sample3() {
    let testdir = TestDir::new(BIN, "");
    let output = testdir
        .cmd()
        .output_with_stdin(
            r#"10 5
10 -80 21
23 8 38
-94 28 11
-26 -2 18
-69 72 79
-26 -86 -54
-72 -50 59
21 65 -32
40 -94 87
-62 18 82
"#,
        )
        .tee_output()
        .expect_success();
    assert_eq!(output.stdout_str(), "638\n");
    assert!(output.stderr_str().is_empty());
}

#[test]
fn sample4() {
    let testdir = TestDir::new(BIN, "");
    let output = testdir
        .cmd()
        .output_with_stdin(
            r#"3 2
2000000000 -9000000000 4000000000
7000000000 -5000000000 3000000000
6000000000 -1000000000 8000000000
"#,
        )
        .tee_output()
        .expect_success();
    assert_eq!(output.stdout_str(), "30000000000\n");
    assert!(output.stderr_str().is_empty());
}
