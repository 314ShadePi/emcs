use std::io::{BufRead, BufReader};
use std::process::{Command, Stdio};

fn main() {
    println!("cargo:rerun-if-changed=styles/");
    println!("cargo:rerun-if-changed=src/style.css");
    let compile_sass = Command::new("sass")
        .args(["styles/index.scss", "src/style.css"])
        .stdout(Stdio::piped())
        .stderr(Stdio::piped())
        .spawn()
        .unwrap();

    let output = compile_sass.stdout.unwrap();

    let out = BufReader::new(output);

    out.lines()
        .filter_map(|line| line.ok())
        .for_each(|line| println!("{}", line));

    let output = compile_sass.stderr.unwrap();

    let out = BufReader::new(output);

    out.lines()
        .filter_map(|line| line.ok())
        .for_each(|line| println!("{}", line));
}
