use cli_test_dir::*;

const BIN: &'static str = "./main";

#[test]
fn sample1() {
    let testdir = TestDir::new(BIN, "");
    let output = testdir
        .cmd()
        .output_with_stdin(r#"7
beat
vet
beet
bed
vet
bet
beet
"#)
        .tee_output()
        .expect_success();
    assert_eq!(output.stdout_str(), r#"beet
vet
"#);
    assert!(output.stderr_str().is_empty());
}

#[test]
fn sample2() {
    let testdir = TestDir::new(BIN, "");
    let output = testdir
        .cmd()
        .output_with_stdin(r#"8
buffalo
buffalo
buffalo
buffalo
buffalo
buffalo
buffalo
buffalo
"#)
        .tee_output()
        .expect_success();
    assert_eq!(output.stdout_str(), "buffalo\n");
    assert!(output.stderr_str().is_empty());
}

#[test]
fn sample3() {
    let testdir = TestDir::new(BIN, "");
    let output = testdir
        .cmd()
        .output_with_stdin(r#"7
bass
bass
kick
kick
bass
kick
kick
"#)
        .tee_output()
        .expect_success();
    assert_eq!(output.stdout_str(), "kick\n");
    assert!(output.stderr_str().is_empty());
}

#[test]
fn sample4() {
    let testdir = TestDir::new(BIN, "");
    let output = testdir
        .cmd()
        .output_with_stdin(r#"4
ushi
tapu
nichia
kun
"#)
        .tee_output()
        .expect_success();
    assert_eq!(output.stdout_str(), r#"kun
nichia
tapu
ushi
"#);
    assert!(output.stderr_str().is_empty());
}