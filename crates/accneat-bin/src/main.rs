use std::env::{args, Args};
use std::process::{Command, Output, Stdio};

pub fn build_cmd(args: &[String]) -> std::io::Result<Output> {
    let args = args.iter().skip(1).to_owned();
    if cfg!(target_os = "windows") {
        Command::new("..\\..\\cmake-build-debug\\accneat.exe")
            .args(args).stdout(Stdio::piped()).output()
    } else {
        Command::new("accneat")
            .args(args).stdout(Stdio::piped()).output()
    }
    // cmd.stdout(Stdio::piped()).output()
}

pub fn execute_cmd_line(args: Args) {
    // println!("Got args: {:?}", args);
    let args: Vec<String> = args.collect();
    let o = build_cmd(args.as_slice());
    // println!("Got result: {:?}", o);
    let o = o.expect("failed to execute process");
    // assert!(o.status.success());
    println!("{}", String::from_utf8(o.stdout).unwrap());
    println!("{}", String::from_utf8(o.stderr).unwrap_or(String::default()));
}


fn main() {
    execute_cmd_line(args());
}
