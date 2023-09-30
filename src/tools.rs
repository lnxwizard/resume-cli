use colored::Colorize;
use toml::Table;

pub fn print_tools(data: &str) {
    let value: Table = data.parse::<Table>().unwrap();

    // print ide's
    println!(
        "\n{}: {}",
        "IDE's".bold(),
        value["ide"].as_str().unwrap().green()
    );

    // print text editors
    println!(
        "{}: {}",
        "Text Editors".bold(),
        value["text_editor"].as_str().unwrap().green()
    );

    // print version control systems
    println!(
        "{}: {}\n",
        "Version Control System".bold(),
        value["version_control_system"].as_str().unwrap().green()
    );
}
