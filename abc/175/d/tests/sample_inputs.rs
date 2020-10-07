use cli_test_dir::*;

const BIN: &'static str = "./main";

#[test]
fn sample1() {
    let testdir = TestDir::new(BIN, "");
    let output = testdir
        .cmd()
        .output_with_stdin(r#"5 2
2 4 5 1 3
3 4 -10 -8 8
"#)
        .tee_output()
        .expect_success();
    assert_eq!(output.stdout_str(), "8\n");
    assert!(output.stderr_str().is_empty());
}

#[test]
fn sample2() {
    let testdir = TestDir::new(BIN, "");
    let output = testdir
        .cmd()
        .output_with_stdin(r#"2 3
2 1
10 -7
"#)
        .tee_output()
        .expect_success();
    assert_eq!(output.stdout_str(), "13\n");
    assert!(output.stderr_str().is_empty());
}

#[test]
fn sample3() {
    let testdir = TestDir::new(BIN, "");
    let output = testdir
        .cmd()
        .output_with_stdin(r#"3 3
3 1 2
-1000 -2000 -3000
"#)
        .tee_output()
        .expect_success();
    assert_eq!(output.stdout_str(), "-1000\n");
    assert!(output.stderr_str().is_empty());
}

#[test]
fn sample4() {
    let testdir = TestDir::new(BIN, "");
    let output = testdir
        .cmd()
        .output_with_stdin(r#"10 58
9 1 6 7 8 4 3 2 10 5
695279662 988782657 -119067776 382975538 -151885171 -177220596 -169777795 37619092 389386780 980092719
"#)
        .tee_output()
        .expect_success();
    assert_eq!(output.stdout_str(), "29507023469\n");
    assert!(output.stderr_str().is_empty());
}
