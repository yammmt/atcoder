use cli_test_dir::*;

const BIN: &'static str = "./main";

#[test]
fn sample1() {
    let testdir = TestDir::new(BIN, "");
    let output = testdir
        .cmd()
        .output_with_stdin(r#"10 10 3
1 5
3 6
7 10
5 8
4 4
1 4
2 9
1 3
1 1
4 5
1 6
2 7
10 1
"#)
        .tee_output()
        .expect_success();
    assert_eq!(output.stdout_str(), r#"2
4
8
"#);
    assert!(output.stderr_str().is_empty());
}

#[test]
fn sample2() {
    let testdir = TestDir::new(BIN, "");
    let output = testdir
        .cmd()
        .output_with_stdin(r#"10 10 4
1 2
2 4
3 6
4 8
5 10
9 10
7 8
5 6
3 5
1 3
10 1
3 8
2 4
1 3
"#)
        .tee_output()
        .expect_success();
    assert_eq!(output.stdout_str(), r#"10
4
2
2
"#);
    assert!(output.stderr_str().is_empty());
}

#[test]
fn sample3() {
    let testdir = TestDir::new(BIN, "");
    let output = testdir
        .cmd()
        .output_with_stdin(r#"314159265 10 1
1 10000
500 12031
1414 113232
111111 777777
666661 23423423
12345678 123456789
111111111 314159265
112334 235235235
1 223445
314 1592
1 314159265
"#)
        .tee_output()
        .expect_success();
    assert_eq!(output.stdout_str(), "7\n");
    assert!(output.stderr_str().is_empty());
}

