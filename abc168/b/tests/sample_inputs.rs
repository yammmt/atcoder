use cli_test_dir::*;

const BIN: &'static str = "./main";

#[test]
fn sample1() {
    let testdir = TestDir::new(BIN, "");
    let output = testdir
        .cmd()
        .output_with_stdin(r#"7
nikoandsolstice
"#)
        .tee_output()
        .expect_success();
    assert_eq!(output.stdout_str(), "nikoand...\n");
    assert!(output.stderr_str().is_empty());
}

#[test]
fn sample2() {
    let testdir = TestDir::new(BIN, "");
    let output = testdir
        .cmd()
        .output_with_stdin(r#"40
ferelibenterhominesidquodvoluntcredunt
"#)
        .tee_output()
        .expect_success();
    assert_eq!(output.stdout_str(), "ferelibenterhominesidquodvoluntcredunt\n");
    assert!(output.stderr_str().is_empty());
}
