use clap::{Arg, Command};
use std::{os::unix::process::CommandExt, process};

fn ensure_flatpak(app: &str) -> Result<(), ()> {
    let args = vec!["info", app];
    let output = match process::Command::new("flatpak")
        .args(&args)
        .stderr(std::process::Stdio::inherit())
        .output()
    {
        Ok(output) => output,
        Err(_err) => {
            std::process::exit(-1);
        }
    };
    if !output.status.success() {
        std::process::exit(-1);
    }
    Ok(())
}

fn setup_socket_proxy(socket_path: &str, port: u32) -> Result<(), ()> {
    let _output = process::Command::new("socat")
        .arg(format!("UNIX-LISTEN:{},fork", socket_path))
        .arg(format!("VSOCK-CONNECT:2:{}", port))
        .spawn()
        .unwrap();

    Ok(())
}

fn main() {
    println!("Running agent");
    let cli = Command::new("flatkrun-agent").arg(Arg::new("APP").required(true).index(1));
    let matches = cli.get_matches();
    let flatpak_app = matches.get_one::<String>("APP").unwrap();
    ensure_flatpak(flatpak_app).expect("Unable to run flatpak");
    setup_socket_proxy("/home/kark/wayland-host", 6000).expect("");
    process::Command::new("/usr/bin/bash").exec();
}
