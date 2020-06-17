use structopt::StructOpt;

/// Search for a pattern in a file and display the lines that contain it.
#[derive(StructOpt)]
struct Cli {
    #[structopt(long)]
    github_username: String,
    #[structopt(long, required=false, default_value="")]
    github_password: String,
}

fn main() {
    let args = Cli::from_args();
    let username = &args.github_username;
    let password = &args.github_password;
    if password == "" {
        panic!("You must enter a password.")
    }
    // println!("username: {}", username);
    println!("username: {}, password; {}", username, password);
}
