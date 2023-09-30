use colored::Colorize;
use toml::Table;

pub fn print_skills(data: &str) {
    let value: Table = data.parse::<Table>().unwrap();

    // print web technologies
    println!(
        "\n{}: {}",
        "Web Technologies".bold(),
        value["web_technologies"].as_str().unwrap().green()
    );

    // print languages
    println!(
        "{}: {}",
        "Languages".bold(),
        value["languages"].as_str().unwrap().green()
    );

    // print dev ops
    println!(
        "{}: {}\n",
        "Dev Ops".bold(),
        value["dev_ops"].as_str().unwrap().green()
    );
}
