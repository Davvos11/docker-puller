use crate::error::DockerError;
use clap::Parser;
use rouille::Response;
use std::ffi::OsStr;
use std::fmt::Display;
use std::path::{Path, PathBuf};
use std::process::{Command, exit};

mod error;

#[derive(Parser)]
#[command(version, about)]
enum Args {
    Cli {
        service_name: String,

        #[arg(short, long)]
        path: Option<PathBuf>,
    },
    Server {
        #[arg(short, long)]
        port: Option<u16>,
    },
}

fn main() {
    let args = Args::parse();

    match args {
        Args::Cli { service_name, path } => {
            if let Err(err) = pull_and_up(service_name, path) {
                eprintln!("{}", err);
                if let DockerError::Command { status, .. } = err {
                    exit(status)
                }
            }
        }
        Args::Server { port } => {
            let secret = std::env::var("SECRET").expect("SECRET env variable must be set");
            let host = format!("0.0.0.0:{}", port.unwrap_or(8080));
            println!("Starting server on https://{host}");
            rouille::start_server(host, move |request| {
                if let Some(token) = request.get_param("token") {
                    if token != secret {
                        return Response::text("Incorrect token").with_status_code(401);
                    }
                } else {
                    return Response::text("Please provide the `token` parameter")
                        .with_status_code(401);
                }
                let service_name = request.get_param("service");
                let path = request.get_param("path");
                if let Some(service_name) = service_name {
                    if let Err(err) = pull_and_up(service_name, path) {
                        Response::text(err.to_string()).with_status_code(500)
                    } else {
                        Response::text("Done")
                    }
                } else {
                    Response::text("Please provide the `service` parameter").with_status_code(400)
                }
            });
        }
    }
}

fn pull_and_up<P: AsRef<Path> + Clone>(
    service_name: String,
    path: Option<P>,
) -> Result<(), DockerError> {
    run(
        "docker",
        ["compose", "pull", service_name.as_str()],
        path.clone(),
    )?;
    run(
        "docker",
        ["compose", "up", "-d", service_name.as_str()],
        path.clone(),
    )?;
    Ok(())
}

fn run<I, S, P>(program: S, args: I, path: Option<P>) -> Result<(), DockerError>
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
    let status = cmd.status()?;
    if !status.success() {
        let args: Vec<_> = args.into_iter().map(|x| x.to_string()).collect();
        let arg_str = args.join(" ");
        let path_str = if let Some(path) = path {
            format!(" at {}", path.as_ref().to_str().unwrap_or_default())
        } else {
            String::new()
        };
        return Err(DockerError::Command {
            command: format!("`{program} {arg_str}`{path_str}"),
            status: status.code().unwrap_or(1),
        });
    }
    Ok(())
}
