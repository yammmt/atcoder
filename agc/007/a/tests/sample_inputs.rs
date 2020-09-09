use cli_test_dir::*;

const BIN: &'static str = "./main";

#[test]
fn sample1() {
    let testdir = TestDir::new(BIN, "");
    let output = testdir
        .cmd()
        .output_with_stdin(r#"4 5
##...
.##..
..##.
...##
"#)
        .tee_output()
        .expect_success();
    assert_eq!(output.stdout_str(), "Possible\n");
    assert!(output.stderr_str().is_empty());
}

#[test]
fn sample2() {
    let testdir = TestDir::new(BIN, "");
    let output = testdir
        .cmd()
        .output_with_stdin(r#"5 3
###
..#
###
#..
###
"#)
        .tee_output()
        .expect_success();
    assert_eq!(output.stdout_str(), "Impossible\n");
    assert!(output.stderr_str().is_empty());
}

#[test]
fn sample3() {
    let testdir = TestDir::new(BIN, "");
    let output = testdir
        .cmd()
        .output_with_stdin(r#"4 5
##...
.###.
.###.
...##
"#)
        .tee_output()
        .expect_success();
    assert_eq!(output.stdout_str(), "Impossible\n");
    assert!(output.stderr_str().is_empty());
}

