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

mod passman;

fn main() {
    let args = Cli::from_args();

    let default_file_name = ".passman-db".to_string();

    let mut passman = passman::PassMan::new(&default_file_name);

    match args {
        Cli::Save {
            for_what,
            user,
            pass,
        } => {
            passman.save_or_update(&for_what, &user, &pass);
        }
        Cli::GenPass {} => {}
        Cli::Get { for_what, user } => match passman.get(&for_what, &user) {
            Some(pass) => {
                println!("{}", pass);
            }
            _ => {
                println!("Couldn't find the password you're looking for.");
            }
        },
        Cli::ToClip {} => {}
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
        #[structopt(short, long)]
        pass: String,
    },
    GenPass {},
    Get {
        #[structopt(short = "f", long = "for")]
        for_what: String,
        #[structopt(short, long)]
        user: String,
    },
    ToClip {},
    Sync {},
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn it_should_save_a_pass_and_get_it_inmemory() {
        let mut passman = passman::PassMan::new(&"testdb.test");
        passman.save_or_update("test1", "user1", "pass1");
        passman.save_or_update("test2", "user2", "pass2");
        passman.save_or_update("test1", "user3", "pass3");

        assert_eq!(passman.get("test1", "user1"), Some(String::from("pass1")));
        assert_eq!(passman.get("test2", "user2"), Some(String::from("pass2")));
        assert_eq!(passman.get("test1", "user3"), Some(String::from("pass3")));
    }
    #[test]
    fn it_should_save_a_pass_and_get_it_in_a_file() {
        let mut passman = passman::PassMan::new(&std::path::PathBuf::from("testdb.test"));
        passman.save_or_update("test1", "user1", "pass1");
        passman.save_or_update("test2", "user2", "pass2");
        passman.save_or_update("test1", "user3", "pass3");
        passman.save().unwrap();

        let passman = passman::PassMan::new(&std::path::PathBuf::from("testdb.test"));

        assert_eq!(passman.get("test1", "user1"), Some(String::from("pass1")));
        assert_eq!(passman.get("test2", "user2"), Some(String::from("pass2")));
        assert_eq!(passman.get("test1", "user3"), Some(String::from("pass3")));
    }
}
