//! # password manager that syncs with Google Drive
//! So what I want to do is create a command line application which I can use as follows:
//! ```
//! $ passman save for github.com user maheshbansod pass thegithubpassword
//! $ passman genpass 12
//! $ passman save for <website> user <username> (pass <password>|genpass [<len>])
//! $ passman get for <website> user <username>
//! $ passman toclip for <website> user <username>
//! $ passman sync
//!
//!

use structopt::StructOpt;

fn main() {
    let args = Cli::from_args();

    match args {
        Cli::Save {
            for_what,
            user,
            pass,
        } => {
            println!("{} {} {}", for_what, user, pass);
        }
        Cli::GenPass {} => {}
        Cli::Get {} => {}
        Cli::ToClip {} => {}
        Cli::Sync {} => {}
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
        #[structopt(short, long)]
        pass: String,
    },
    GenPass {},
    Get {},
    ToClip {},
    Sync {},
}
