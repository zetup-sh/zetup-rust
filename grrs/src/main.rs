use std::process::{Command, Stdio};

//// Search for a pattern in a file and display the lines that contain it.
// #[derive(StructOpt)]
// struct Cli {
//     #[structopt(long)]
//     github_username: String,
//     #[structopt(long, required=false, default_value="")]
//     github_password: String,
// }

fn command_exists(cmd: &str) -> bool {
    let status = Command::new("powershell")
        .args(&[format!("Get-Command {}", cmd)])
        .stdin(Stdio::null())
        .stdout(Stdio::null())
        .stderr(Stdio::null())
        .status()
        .expect("couldn't test command exists in powershell");
    status.success()
}

fn install_choco() {
    let install_choco_cmd = "Set-ExecutionPolicy Bypass -Scope Process -Force; [System.Net.ServicePointManager]::SecurityProtocol = [System.Net.ServicePointManager]::SecurityProtocol -bor 3072; iex ((New-Object System.Net.WebClient).DownloadString('https://chocolatey.org/install.ps1'))";
    Command::new("powershell")
        .args(&[install_choco_cmd])
        .status()
        .expect("couldn't install choco");
}

fn install_from_choco(item: &str) {
    Command::new("powershell")
        .args(&[format!("choco install -y {}", item)])
        .status()
        .expect(format!("couldn't install {}", item).as_str());
}

fn main() {
    if cfg!(target_os = "windows") {
        if !command_exists("choco") {
            install_choco();
        }
        if !command_exists("git") {
            install_from_choco("git"); // installs openssl too
        }
    }

    // let args = Cli::from_args();
    // let username = &args.github_username;
    // let password = &args.github_password;
    // if password == "" {
    //     panic!("You must enter a password.")
    // }
    // // println!("username: {}", username);
    // println!("username: {}, password; {}", username, password);
}