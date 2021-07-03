use nix::fcntl::{renameat2, RenameFlags};
use nix::libc::AT_FDCWD;
use std::env;
use std::fs::File;
use std::io::prelude::*;
use std::io::{BufRead, BufReader};
use std::path::Path;
use std::process::{Command, Stdio};

fn format_vmstat_output(line: std::io::Result<String>) -> String {
    match line {
        Ok(line) => {
            let t: Vec<&str> = line.split_whitespace().collect();
            format!(
                "r: {}, b: {}, swpd: {}, free: {}, buff: {}, cache: {}, si: {}, so: {}, bi: {}, bo: {}, in: {}, cs: {}, us: {}, sy: {}, id: {}, wa: {}, st: {}",
                t[0], t[1], t[2], t[3], t[4], t[5], t[6], t[7], t[8], t[9], t[10], t[11], t[12], t[13], t[14], t[15], t[16]
            )
        }
        Err(_) => String::from(""),
    }
}

fn main() {
    let args: Vec<String> = env::args().collect();
    let file_path = Path::new(&args[1]);
    {
        File::create(file_path).unwrap();
    }
    let temp_file_name = [&args[1], ".temp"].join("");
    let temp_file_path = Path::new(&temp_file_name);

    let child = Command::new("vmstat")
        .args(&["--wide", "--one-header", "--unit", "M", "1"])
        .stdout(Stdio::piped())
        .spawn()
        .expect(format!("Unable to start vmstat").as_str());

    let reader = BufReader::new(child.stdout.unwrap());
    for line in reader.lines().skip(2) {
        let mut f = File::create(temp_file_path).unwrap();
        f.write(format_vmstat_output(line).as_bytes()).unwrap();
        renameat2(
            Some(AT_FDCWD),
            temp_file_path,
            Some(AT_FDCWD),
            file_path,
            RenameFlags::RENAME_EXCHANGE,
        )
        .unwrap();
    }
}
