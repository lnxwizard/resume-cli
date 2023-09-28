mod about;

use about::print_about;
use colored::Colorize;
use inquire::Select;
use std::process::exit;

const NAME: &str = "Alper Akça";
const GH_PROFILE: &str = "https://github.com/lnxwizard";

fn main() {
    println!(
        "\n\nHi! I'm {}, a Rust and Go developer.",
        NAME.bold().green()
    );

    let options = vec!["About", "Skills", "Tools", "Contact", "Exit"];

    'main: loop {
        let choice = Select::new("What would you like to know about me?", options.clone()).prompt();

        match choice {
            Ok(choice) => {
                if choice == options[0] {
                    print_about();
                } else if choice == options[1] {
                    println!("Skills");
                } else if choice == options[2] {
                    println!("Tools");
                } else if choice == options[3] {
                    println!("Contact");
                } else if choice == options[4] {
                    println!("Bye Bye...");
                    exit(0);
                }
            }
            Err(_) => println!(
                "{:?} is not a valid option, please select a valid option.",
                choice
            ),
        }
    }
}
