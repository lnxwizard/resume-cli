use colored::Colorize;

pub fn print_about() {
    println!("\n- I'm Software Developer, Pixel Artist and Linux Enthusiast.");
    println!("- I’m currently working on some CLI tool projects.");
    println!("- I’m currently learning Go Programming Language and Rust.");
    println!(
        "- All of my projects are available at {}.",
        "https://lnxwizard.github.io/projects".green()
    );
    println!(
        "- I regularly write articles on {}.",
        "https://lnxwizard.github.io/blog".green()
    );
    println!("- I'm From {}.\n", "Turkey".bright_red());
}
