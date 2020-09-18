use cli_test_dir::*;

const BIN: &'static str = "./main";

#[test]
fn sample1() {
    let testdir = TestDir::new(BIN, "");
    let output = testdir
        .cmd()
        .output_with_stdin(r#"6
khabarovsk 20
moscow 10
kazan 50
kazan 35
moscow 60
khabarovsk 40
"#)
        .tee_output()
        .expect_success();
    assert_eq!(output.stdout_str(), r#"3
4
6
1
5
2
"#);
    assert!(output.stderr_str().is_empty());
}

#[test]
fn sample2() {
    let testdir = TestDir::new(BIN, "");
    let output = testdir
        .cmd()
        .output_with_stdin(r#"10
yakutsk 10
yakutsk 20
yakutsk 30
yakutsk 40
yakutsk 50
yakutsk 60
yakutsk 70
yakutsk 80
yakutsk 90
yakutsk 100
"#)
        .tee_output()
        .expect_success();
    assert_eq!(output.stdout_str(), r#"10
9
8
7
6
5
4
3
2
1
"#);
    assert!(output.stderr_str().is_empty());
}
