use cli_test_dir::*;

const BIN: &'static str = "./main";

#[test]
fn sample1() {
    let testdir = TestDir::new(BIN, "");
    let output = testdir
        .cmd()
        .output_with_stdin(r#"3
5 7
2 6
8 10
"#)
        .tee_output()
        .expect_success();
    assert_eq!(output.stdout_str(), "18\n");
    assert!(output.stderr_str().is_empty());
}

#[test]
fn sample2() {
    let testdir = TestDir::new(BIN, "");
    let output = testdir
        .cmd()
        .output_with_stdin(r#"5
1 71
43 64
13 35
14 54
79 85
"#)
        .tee_output()
        .expect_success();
    assert_eq!(output.stdout_str(), "334\n");
    assert!(output.stderr_str().is_empty());
}

#[test]
fn sample3() {
    let testdir = TestDir::new(BIN, "");
    let output = testdir
        .cmd()
        .output_with_stdin(r#"11
15004200 341668840
277786703 825590503
85505967 410375631
797368845 930277710
90107929 763195990
104844373 888031128
338351523 715240891
458782074 493862093
189601059 534714600
299073643 971113974
98291394 443377420
"#)
        .tee_output()
        .expect_success();
    assert_eq!(output.stdout_str(), "8494550716\n");
    assert!(output.stderr_str().is_empty());
}

