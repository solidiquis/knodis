use std::process::Output;

pub fn print_status(Output { status, stdout, stderr }: Output) {
    if !status.success() {
        eprintln!("{}", String::from_utf8(stderr).unwrap())
    } else {
        println!("{}", String::from_utf8(stdout).unwrap())
    }
}
