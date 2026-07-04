use assert_cmd::cargo::cargo_bin_cmd;

#[test]
fn bytes_count() {
    let mut cmd = cargo_bin_cmd!("ccwc");
    cmd.arg("-c").arg("test.txt");
    cmd.assert().success().stdout("  342190 test.txt\n");
}

#[test]
fn lines_count() {
    let mut cmd = cargo_bin_cmd!("ccwc");
    cmd.arg("-l").arg("test.txt");
    cmd.assert().success().stdout("  7145 test.txt\n");
}

#[test]
fn words_count() {
    let mut cmd = cargo_bin_cmd!("ccwc");
    cmd.arg("-w").arg("test.txt");
    cmd.assert().success().stdout("  58164 test.txt\n");
}

#[test]
fn chars_count() {
    let mut cmd = cargo_bin_cmd!("ccwc");
    cmd.arg("-m").arg("test.txt");
    cmd.assert().success().stdout("  339292 test.txt\n");
}

#[test]
fn no_filename() {
    let mut cmd = cargo_bin_cmd!("ccwc");
    cmd.arg("-c");
    cmd.assert()
        .failure()
        .stderr("ccwc: missing filename as argument\n");
}

#[test]
fn empty_filename() {
    let mut cmd = cargo_bin_cmd!("ccwc");
    cmd.arg("-c").arg("");
    cmd.assert().failure().stderr("ccwc: unknown argument ``\n");
}

#[test]
fn unknown_arg() {
    let mut cmd = cargo_bin_cmd!("ccwc");
    cmd.arg("-d");
    cmd.assert()
        .failure()
        .stderr("ccwc: unknown argument `-d`\n");
}

#[test]
fn no_args() {
    let mut cmd = cargo_bin_cmd!("ccwc");
    cmd.assert()
        .failure()
        .stderr("ccwc: missing filename as argument\n");
}
