mod about;
mod skills;
mod social;
mod tools;

use about::print_about;
use colored::Colorize;
use inquire::Select;
use skills::print_skills;
use social::print_social;
use std::fs;
use tools::print_tools;

const NAME: &str = "Alper Akça";

// file paths
const SOCIAL_FILE_PATH: &str = "data/social.toml";
const SKILLS_FILE_PATH: &str = "data/skills.toml";
const TOOLS_FILE_PATH: &str = "data/tools.toml";

fn main() {
    println!(
        "\n\nHi! I'm {}, a Rust and Go developer.",
        NAME.bold().green()
    );

    let options: Vec<&str> = vec!["About", "Skills", "Tools", "Social", "Exit"];

    loop {
        let choice = Select::new("What would you like to know about me?", options.clone()).prompt();

        match choice {
            Ok(choice) => {
                if choice == options[0] {
                    print_about();
                } else if choice == options[1] {
                    let contents = fs::read_to_string(SKILLS_FILE_PATH)
                        .expect("Couldn't find skills.toml file");

                    // print skills
                    print_skills(&contents);
                } else if choice == options[2] {
                    let contents = fs::read_to_string(TOOLS_FILE_PATH)
                        .expect("Couldn't find tools.toml file.");

                    // print tools
                    print_tools(&contents);
                } else if choice == options[3] {
                    let contents = fs::read_to_string(SOCIAL_FILE_PATH)
                        .expect("Couldn't find social.toml file.");

                    // print social information's
                    print_social(&contents)
                } else if choice == options[4] {
                    println!("Bye Bye...");
                    break;
                }
            }
            Err(_) => {
                println!("{:?} is not a valid option.", choice);
            }
        }
    }
}
