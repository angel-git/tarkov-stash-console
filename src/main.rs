use spinners::{Spinner, Spinners};
use std::net::TcpStream;
use inquire::Text;

fn main() {
    const NAME: &str = env!("CARGO_PKG_NAME");
    const VERSION: &str = env!("CARGO_PKG_VERSION");
    println!(">>> Welcome to {NAME} v{VERSION} <<<");

    let mut sp = Spinner::new(Spinners::Dots9, "Checking SPT server...".into());
    match TcpStream::connect("127.0.0.1:6969") {
        Ok(_) => {
            sp.stop_with_message("❌ Looks like your SPT server is running, please stop it and try again".into());
            return;
        }
        _ => {
            sp.stop_with_message("✅ SPT server is not running".into());
        }
    }



    // Text::new("What is your name?");
    // match name {
    //     Ok(name) => println!("Hello {name}"),
    //     Err(_) => println!("An error happened when asking for your name, try again later."),
    // }
}
