#[test]
fn compile_pass() {
    // Strip "#![cfg_attr(rustfmt, rustfmt_skip)]\n" from all of the .out.rs files.
    let path = "tests/expand/pass";
    let files = std::fs::read_dir(path)
        .unwrap()
        .filter_map(|entry| {
            // Only accept paths that end in .out.rs
            let entry = entry.unwrap();
            let path = entry.path();
            let path_str = path.to_str().unwrap();
            if path_str.ends_with(".out.rs") {
                Some(path)
            } else {
                None
            }
        })
        .collect::<Vec<_>>();

    for file in files {
        let contents = std::fs::read_to_string(&file).unwrap();
        let contents = match contents.strip_prefix("#![cfg_attr(rustfmt, rustfmt_skip)]\n") {
            Some(contents) => contents,
            None => &contents,
        };
        std::fs::write(file, contents).unwrap();
    }

    tryexpand::expand(["tests/expand/pass/*.rs"])
        .args(["--profile", "test"])
        .and_check()
        .expect_pass();

    // tryexpand::expand(["tests/expand/pass/*.rs"])
    //     .args(["--profile", "test"])
    //     .skip_overwrite()
    //     .and_run()
    //     .expect_pass();

    // Add back the "#![cfg_attr(rustfmt, rustfmt_skip)]\n" to all of the .out.rs files.
    let files = std::fs::read_dir(path)
        .unwrap()
        .filter_map(|entry| {
            // Only accept paths that end in .out.rs
            let entry = entry.unwrap();
            let path = entry.path();
            let path_str = path.to_str().unwrap();
            if path_str.ends_with(".out.rs") {
                Some(path)
            } else {
                None
            }
        })
        .collect::<Vec<_>>();

    for file in files {
        let contents = std::fs::read_to_string(&file).unwrap();
        let contents = format!("#![cfg_attr(rustfmt, rustfmt_skip)]\n{}", contents);
        std::fs::write(file, contents).unwrap();
    }
}

#[test]
fn compile_fail() {
    tryexpand::expand(["tests/expand/fail/*.rs"])
        .and_check()
        .expect_fail();
}
