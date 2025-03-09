#[test]
fn compile_pass() {
    tryexpand::expand(["tests/expand/pass/*.rs"])
        .args(["--profile", "test"])
        .and_check()
        .expect_pass();

    tryexpand::expand(["tests/expand/pass/*.rs"])
        .args(["--profile", "test"])
        .skip_overwrite()
        .and_run_tests()
        .expect_pass();
}

#[test]
fn compile_fail() {
    tryexpand::expand(["tests/expand/fail/*.rs"])
        .and_check()
        .expect_fail();
}
