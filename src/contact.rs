use colored::Colorize;
use toml::Table;

pub fn print_contact(data: &str) {
    let value: Table = data.parse::<Table>().expect("Couldn't read data.");

    println!("\n\nYou can contact me at:");

    // print e-mail
    println!(
        "{}: {}",
        "E-Mail".bold(),
        value["email"].as_str().unwrap().green()
    );

    // print reddit
    println!(
        "{}: {}",
        "Reddit".bold(),
        value["reddit"].as_str().unwrap().green()
    );

    println!(
        "{}: {}",
        "Medium".bold(),
        value["medium"].as_str().unwrap().green()
    );
}
