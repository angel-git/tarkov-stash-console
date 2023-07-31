use std::io::{Error, ErrorKind};
use std::net::TcpStream;
use std::path::Path;

use inquire::{Select, Text};
use spinners::{Spinner, Spinners};
use stash::stash_utils::increase_currency;

pub mod spt;
pub mod stash;

fn main() {
    const NAME: &str = env!("CARGO_PKG_NAME");
    const VERSION: &str = env!("CARGO_PKG_VERSION");
    println!(">>> Welcome to {NAME} v{VERSION} <<<");
    println!(">>> This version has only been tested with SPT 3.5.7 <<<");

    let mut sp = Spinner::new(Spinners::Dots9, "Checking SPT server...".into());
    match TcpStream::connect("127.0.0.1:6969") {
        Ok(_) => {
            sp.stop_with_message(
                "❌ Looks like your SPT server is running, please stop it and try again".into(),
            );
            return;
        }
        _ => {
            sp.stop_with_message("✅ SPT server is not running".into());
        }
    }

    start();
}

fn start() {
    profile_prompt();
}

fn profile_prompt() {
    let file = Text::new("What is your profile path?")
        .with_help_message("Example: C:\\SPT\\user\\profiles\\4324234.json")
        .prompt()
        .unwrap();

    let file_str = file.as_str();

    let file_path = Path::new(file_str);
    if file_path.exists() {
        create_backup_if_needed(file_str);
        profile_edit_prompt(file_str);
    } else {
        println!("Something went wrong reading your profile file, is the path correct?")
    }
}

fn create_backup_if_needed(profile_path: &str) {
    let backup_pack = String::from(profile_path) + ".back";
    let backup_pack_str = backup_pack.as_str();
    let file_path = Path::new(backup_pack_str);
    if !file_path.exists() {
        println!("--------------------------------");
        println!("Looks like you don't have a backup of your profile, I will create one under: {backup_pack_str}");
        std::fs::copy(profile_path, backup_pack_str).unwrap();
        println!("Backup created, you can restore that if your profile gets broken.");
        println!("--------------------------------");
    }
}

fn profile_edit_prompt(profile_path: &str) {
    println!("This will increase your current stock to 500.000 on every slot that ALREADY contains currency.");

    let options = vec!["roubles", "USD", "euros"];

    let ans = Select::new("What would you like to increase?", options)
        .prompt()
        .unwrap();

    let increase_result = if ans == "roubles" {
        increase_currency(profile_path, "5449016a4bdc2d6f028b456f")
    } else if ans == "USD" {
        increase_currency(profile_path, "5696686a4bdc2da3298b456a")
    } else if ans == "euros" {
        increase_currency(profile_path, "569668774bdc2da2298b4568")
    } else {
        Err(Error::new(
            ErrorKind::Other,
            format!("Wrong option selected {ans}"),
        ))
    };

    match increase_result {
        Ok(_) => {
            println!("✅ Profile updated");
        }
        Err(e) => {
            println!("❌ Something went wrong when writing your profile: {e}");
        }
    }
}
