//! # password manager that syncs with Google Drive
//! So what I want to do is create a command line application which I can use as follows:
//! ```
//! $ passman save for github.com user maheshbansod pass thegithubpassword
//! $ passman gen-pass [12]
//! $ passman save for <website> user <username> (pass <password>|genpass [<len>])
//! $ passman get for <website> user <username>
//! $ passman sync
//!

use structopt::StructOpt;

fn main() {
    let args = Cli::from_args();

    let default_file_name = shellexpand::full("~/.passman-db").unwrap().to_string();

    let mut passman = passman::PassMan::new(&default_file_name);

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
        _ => {
            //TODO: make more descriptive + proper error return codes
            println!("There was an error while trying to save the file.");
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
