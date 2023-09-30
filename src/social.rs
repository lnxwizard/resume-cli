use colored::Colorize;
use toml::Table;

pub fn print_social(data: &str) {
    let value: Table = data.parse::<Table>().expect("Couldn't read data.");

    println!("\nYou can find me in this platforms:");

    // print e-mail
    println!(
        "{}: {}",
        "E-Mail".bold(),
        value["email"].as_str().unwrap().green()
    );

    // print github
    println!(
        "{}: {}",
        "GitHub".bold(),
        value["github"].as_str().unwrap().green()
    );

    // print reddit
    println!(
        "{}: {}",
        "Reddit".bold(),
        value["reddit"].as_str().unwrap().green()
    );

    // print gitlab
    println!(
        "{}: {}",
        "GitLab".bold(),
        value["gitlab"].as_str().unwrap().green()
    );

    // print medium
    println!(
        "{}: {}",
        "Medium".bold(),
        value["medium"].as_str().unwrap().green()
    );

    // print dev.to
    println!(
        "{}: {}\n",
        "Dev.to".bold(),
        value["devto"].as_str().unwrap().green()
    );
}
