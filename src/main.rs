use daemonize::Daemonize;
use rodio::{Decoder, DeviceSinkBuilder, Player};
use std::fs;
use std::fs::File;
use std::io::BufReader;
use std::io::Read;
use std::io::Write;
use std::os::unix::net::UnixListener;
use std::os::unix::process::CommandExt;
use std::path::Path;
use std::process::Command;

fn validate_package(package: &str) {
    let package_exists = Command::new("which")
        .arg(package)
        .output()
        .map(|out| out.status.success())
        .unwrap_or(false);

    if !package_exists {
        println!(
            "Error: Can't find '{}'. Make sure you have it installed and available in your PATH.",
            package
        );
        std::process::exit(1);
    }
}

fn play_baka(project_root: &str) {
    let handle =
        DeviceSinkBuilder::open_default_sink().expect("Erropr trying to send output audio device");
    let player = Player::connect_new(handle.mixer());

    let file =
        File::open(format!("{}/sfx/baka.mp3", project_root)).expect("can't open audio stuff");
    let buf_reader = BufReader::new(file);

    let source = Decoder::new(buf_reader).expect("can't decode file");

    player.append(source);
    player.sleep_until_end();

    println!("bakabaka");
}

fn main() {
    validate_package("gpu-screen-recorder");

    let project_root = env!("CARGO_MANIFEST_DIR");
    let records_path = Path::new(project_root).join("records");

    let tmp_dir = "/tmp/bakarecorder";

    let _ = fs::remove_dir_all(tmp_dir);
    fs::create_dir_all(tmp_dir).expect("Error trying to create directory");

    let stdout_path = format!("{}/output.log", tmp_dir);
    let stderr_path = format!("{}/error.log", tmp_dir);
    let pid_path = format!("{}/daemon.pid", tmp_dir);
    let socket_path = format!("{}/daemon.sock", tmp_dir);

    let stdout = File::create(stdout_path).unwrap();
    let stderr = File::create(stderr_path).unwrap();

    let daemonize = Daemonize::new()
        .pid_file(pid_path)
        .working_directory(tmp_dir)
        .stdout(stdout)
        .stderr(stderr);

    match daemonize.start() {
        Ok(_) => println!("Daemon started..."),
        Err(err) => {
            eprintln!("Error: {}", err);
            std::process::exit(1);
        }
    }

    let _ = fs::remove_file(&socket_path);

    let listener = UnixListener::bind(&socket_path).expect("can't create socket");

    println!("Listening commands on: {}", socket_path);

    let mut child = Command::new("sh")
        .arg("-c")
        .arg(format!(
            "gpu-screen-recorder -w screen -f 60 -k h264 -ac opus -r 15 -c mkv -o {} -a default_output -a \"$(pactl get-default-sink).monitor\"",
            records_path.display()
        ))
        .process_group(0)
        .spawn()
        .expect("Error trying to record");

    for stream in listener.incoming() {
        match stream {
            Ok(mut stream) => {
                let mut buffer = [0; 512];

                if let Ok(bytes_read) = stream.read(&mut buffer) {
                    let command = String::from_utf8_lossy(&buffer[..bytes_read])
                        .trim()
                        .to_string();

                    println!("Command received: {}", command);

                    match command.as_str() {
                        "SAVE" => {
                            let _ = stream.write_all(b"Saving...\n");

                            Command::new("sh")
                                .arg("-c")
                                .arg("pkill -SIGUSR1 gpu-screen-reco")
                                .process_group(0)
                                .spawn()
                                .expect("Error trying to save")
                                .wait()
                                .expect("Error waiting for process");

                            play_baka(project_root);
                        }
                        "EXIT" => {
                            let _ = stream.write_all(b"Exiting...\n");
                            let _ = fs::remove_dir_all(tmp_dir);

                            child.kill().expect("Error trying to kill screen recorder");
                            child.wait().expect("Error waiting for process");

                            std::process::exit(0);
                        }
                        _ => {
                            let _ = stream.write_all(b"Command doesn't exists\n");
                        }
                    }
                }
            }

            Err(err) => {
                eprintln!("Connection error: {}", err);
            }
        }
    }

    let _ = child.kill();
    let _ = child.wait();
}
