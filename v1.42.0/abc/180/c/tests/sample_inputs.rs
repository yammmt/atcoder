use cli_test_dir::*;

const BIN: &'static str = "./main";

#[test]
fn sample1() {
    let testdir = TestDir::new(BIN, "");
    let output = testdir
        .cmd()
        .output_with_stdin(r#"6
"#)
        .tee_output()
        .expect_success();
    assert_eq!(output.stdout_str(), r#"1
2
3
6
"#);
    assert!(output.stderr_str().is_empty());
}

#[test]
fn sample2() {
    let testdir = TestDir::new(BIN, "");
    let output = testdir
        .cmd()
        .output_with_stdin(r#"720
"#)
        .tee_output()
        .expect_success();
    assert_eq!(output.stdout_str(), r#"1
2
3
4
5
6
8
9
10
12
15
16
18
20
24
30
36
40
45
48
60
72
80
90
120
144
180
240
360
720
"#);
    assert!(output.stderr_str().is_empty());
}

#[test]
fn sample3() {
    let testdir = TestDir::new(BIN, "");
    let output = testdir
        .cmd()
        .output_with_stdin(r#"1000000007
"#)
        .tee_output()
        .expect_success();
    assert_eq!(output.stdout_str(), r#"1
1000000007
"#);
    assert!(output.stderr_str().is_empty());
}

