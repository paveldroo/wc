use assert_cmd::cargo::cargo_bin_cmd;

#[test]
fn bytes_len() -> Result<(), Box<dyn std::error::Error>> {
    let mut cmd = cargo_bin_cmd!("ccwc");
    cmd.arg("-c").arg("test.txt");
    cmd.assert().stdout("  342190 test.txt\n");

    Ok(())
}

#[test]
fn no_filename() -> Result<(), Box<dyn std::error::Error>> {
    let mut cmd = cargo_bin_cmd!("ccwc");
    cmd.arg("-c");
    cmd.assert().stderr("ccwc: missing value for -c\n");

    Ok(())
}

#[test]
fn unknown_arg() -> Result<(), Box<dyn std::error::Error>> {
    let mut cmd = cargo_bin_cmd!("ccwc");
    cmd.arg("-d");
    cmd.assert().stderr("ccwc: unknown argument `-d`\n");

    Ok(())
}

#[test]
fn no_args() -> Result<(), Box<dyn std::error::Error>> {
    let mut cmd = cargo_bin_cmd!("ccwc");
    cmd.assert().stderr("ccwc: missing required argument -c\n");

    Ok(())
}
