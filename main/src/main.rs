//! # password manager that syncs with Google Drive
//! So what I want to do is create a command line application which I can use as follows:
//! ```
//! $ passman save for github.com user maheshbansod pass thegithubpassword
//! $ passman gen-pass [12]
//! $ passman save for <website> user <username> (pass <password>|genpass [<len>])
//! $ passman get for <website> user <username>
//! $ passman sync
//!

use passman::ErrorKind;
use std::io::{stdin, stdout, Write};
use structopt::StructOpt;

fn main() {
    let args = Cli::from_args();

    // TODO: 1. search for argument --config
    // 2. if 1 not found, search for environment variable PASSMAN_CONFIG
    // 3. if 2 not found, use the default config path
    let config_path = shellexpand::full("~/.passman/config").unwrap().to_string();

    let mut passman = match passman::PassMan::new(&config_path) {
        Ok(passman) => passman,
        Err(err) => match err.kind {
            ErrorKind::IOError(err) => {
                if err.kind() == std::io::ErrorKind::NotFound {
                    print!("Where do you want to store the database?(default: ~/.passman/): ");

                    let mut s = String::new();
                    let _ = stdout().flush();
                    stdin()
                        .read_line(&mut s)
                        .expect("Did not enter a correct string");
                    if let Some('\n') = s.chars().next_back() {
                        s.pop();
                    }
                    if let Some('\r') = s.chars().next_back() {
                        s.pop();
                    }
                    let default_path = shellexpand::full("~/.passman").unwrap().to_string();
                    s = if s.is_empty() {
                        default_path.clone()
                    } else {
                        shellexpand::full(&s).unwrap().to_string()
                    };

                    std::fs::create_dir_all(&s)
                        .expect("Failed to create directories on that path.");
                    std::fs::create_dir_all(&default_path)
                        .expect("Failed to create directory for storing settings");

                    if !s.ends_with("/") {
                        s.push_str("/db")
                    } else {
                        s.push_str("db")
                    };
                    println!("{}", s);
                    let path = s;

                    let mut s = String::new();
                    print!("Enter secret key: ");
                    let _ = stdout().flush();
                    stdin()
                        .read_line(&mut s)
                        .expect("Did not enter a correct string");
                    if let Some('\n') = s.chars().next_back() {
                        s.pop();
                    }
                    if let Some('\r') = s.chars().next_back() {
                        s.pop();
                    }
                    //TODO: check key strength
                    let key = s;
                    let config = passman::Config::new(&key).unwrap().set_db(path);
                    println!("Writing config to a file.");
                    config
                        .to_file(&config_path, &key)
                        .expect("Couldn't write config to file");
                    println!("Done");
                    passman::PassMan::with_config(config).unwrap()
                } else {
                    panic!("Error occurred: {}", err);
                }
            }
            _ => {
                panic!("An unknown error occurred while trying to create passman");
            }
        },
    };

    match args {
        Cli::Save {
            for_what,
            user,
            pass,
            genpass,
        } => {
            let pass = pass.unwrap_or_else(|| {
                let len = genpass.expect("Expected genpass if pass is not provided");
                let pass = passman::genpass(len);
                println!("{}", pass);
                pass
            });
            passman.save_or_update(&for_what, &user, &pass);
        }
        Cli::GenPass { len } => {
            println!("{}", passman::genpass(len));
        }
        Cli::Get { for_what, user } => match passman.get(&for_what, &user) {
            Some(pass) => {
                println!("{}", pass);
            }
            _ => {
                println!("Couldn't find the password you're looking for.");
            }
        },
        Cli::Sync {} => {}
    }

    match passman.save() {
        Ok(_) => {}
        Err(e) => {
            //TODO: make more descriptive + proper error return codes
            println!("There was an error while trying to save the file.\n{:?}", e);
        }
    }
    // println!("{:?}", args);
}

#[derive(Debug, StructOpt)]
enum Cli {
    Save {
        #[structopt(short = "f", long = "for")]
        for_what: String,
        #[structopt(short, long)]
        user: String,
        #[structopt(short, long, required_unless = "genpass")]
        pass: Option<String>,
        #[structopt(short, long, required_unless = "pass")]
        genpass: Option<Option<usize>>,
    },
    GenPass {
        len: Option<usize>,
    },
    Get {
        #[structopt(short = "f", long = "for")]
        for_what: String,
        #[structopt(short, long)]
        user: String,
    },
    Sync {},
}
