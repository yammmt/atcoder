use cli_test_dir::*;

const BIN: &'static str = "./main";

#[test]
fn sample1() {
    let testdir = TestDir::new(BIN, "");
    let output = testdir
        .cmd()
        .output_with_stdin(r#"4
3 4 2 1
"#)
        .tee_output()
        .expect_success();
    assert_eq!(output.stdout_str(), "1\n");
    assert!(output.stderr_str().is_empty());
}

#[test]
fn sample2() {
    let testdir = TestDir::new(BIN, "");
    let output = testdir
        .cmd()
        .output_with_stdin(r#"3
1 1000 1
"#)
        .tee_output()
        .expect_success();
    assert_eq!(output.stdout_str(), "0\n");
    assert!(output.stderr_str().is_empty());
}

#[test]
fn sample3() {
    let testdir = TestDir::new(BIN, "");
    let output = testdir
        .cmd()
        .output_with_stdin(r#"7
218 786 704 233 645 728 389
"#)
        .tee_output()
        .expect_success();
    assert_eq!(output.stdout_str(), "23\n");
    assert!(output.stderr_str().is_empty());
}

// Am I wrong?
// #[test]
// fn sample4() {
//     let testdir = TestDir::new(BIN, "");
//     let output = testdir
//         .cmd()
//         .output_with_stdin(r#"6
// 3 3 3 5 5 5
// "#)
//         .tee_output()
//         .expect_success();
//     assert_eq!(output.stdout_str(), "4\n");
//     assert!(output.stderr_str().is_empty());
// }
