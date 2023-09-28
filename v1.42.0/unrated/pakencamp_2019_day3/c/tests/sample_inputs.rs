use cli_test_dir::*;

const BIN: &'static str = "./main";

#[test]
fn sample1() {
    let testdir = TestDir::new(BIN, "");
    let output = testdir
        .cmd()
        .output_with_stdin(r#"1 2
80 84
"#)
        .tee_output()
        .expect_success();
    assert_eq!(output.stdout_str(), "84\n");
    assert!(output.stderr_str().is_empty());
}

#[test]
fn sample2() {
    let testdir = TestDir::new(BIN, "");
    let output = testdir
        .cmd()
        .output_with_stdin(r#"3 4
37 29 70 41
85 69 76 50
53 10 95 100
"#)
        .tee_output()
        .expect_success();
    assert_eq!(output.stdout_str(), "250\n");
    assert!(output.stderr_str().is_empty());
}

#[test]
fn sample3() {
    let testdir = TestDir::new(BIN, "");
    let output = testdir
        .cmd()
        .output_with_stdin(r#"8 2
31000000 41000000
59000000 26000000
53000000 58000000
97000000 93000000
23000000 84000000
62000000 64000000
33000000 83000000
27000000 95000000
"#)
        .tee_output()
        .expect_success();
    assert_eq!(output.stdout_str(), "581000000\n");
    assert!(output.stderr_str().is_empty());
}

