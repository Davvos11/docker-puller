use clap::Parser;
use std::ffi::OsStr;
use std::fmt::Display;
use std::path::{Path, PathBuf};
use std::process::{Command, exit};

#[derive(Parser)]
#[command(version, about)]
struct Args {
    service_name: String,

    #[arg(short, long)]
    path: Option<PathBuf>,
}

fn main() {
    let args = Args::parse();

    run("docker", ["compose", "pull", args.service_name.as_str()], args.path.clone());
    run("docker", ["compose", "up", "-d", args.service_name.as_str()], args.path.clone());
}

fn run<I, S, P>(program: S, args: I, path: Option<P>) 
where
    I: IntoIterator<Item = S> + Clone,
    S: AsRef<OsStr> + Display,
    P: AsRef<Path> + Clone,
{
    let mut cmd = Command::new("docker");
    cmd.args(args.clone());
    if let Some(path) = path.clone() {
        cmd.current_dir(path);
    }
    let status = cmd
        .status()
        .expect("Failed to execute command");
    if !status.success() {
        let args: Vec<_> = args.into_iter().map(|x| x.to_string()).collect();
        let arg_str = args.join(" ");
        let path_str = if let Some(path) = path {
            format!(" at {}", path.as_ref().to_str().unwrap_or_default())
        } else { String::new() };
        eprintln!("Failed to execute: `{program} {arg_str}`{path_str}");
        exit(status.code().unwrap_or(1));
    }
}
