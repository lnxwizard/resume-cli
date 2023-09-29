mod about;
mod contact;

use about::print_about;
use colored::Colorize;
use contact::print_contact;
use inquire::Select;

const NAME: &str = "Alper Akça";

fn main() {
    println!(
        "\n\nHi! I'm {}, a Rust and Go developer.",
        NAME.bold().green()
    );

    let options: Vec<&str> = vec!["About", "Skills", "Tools", "Contact", "Exit"];

    loop {
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
                    print_contact();
                } else if choice == options[4] {
                    println!("Bye Bye...");
                    break;
                }
            }
            Err(_) => {
                println!(
                    "{:?} is not a valid option, please select a valid option.",
                    choice
                );
            }
        }
    }
}
