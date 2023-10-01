# resume-cli
resume-cli is command-line interface portfolio. Feel free to fork, edit and publish this project for yourself.

[![demo](https://asciinema.org/a/611406.svg)](https://asciinema.org/a/611406?autoplay=1)

# Features
- **Customizable:** You can easily customize resume-cli for yourself. Just edit the **toml** files inside the `/data` folder. For example take look [Customization](#customization) section.

# How to Download?
### Via [Git](https://github.com/git/git)
```shell
git clone https://github.com/lnxwizard/resume-cli.git
```

### Via [GitHub](https://github.com/cli/cli)
```shell 
gh repo clone lnxwizard/resume-cli
```

# Running & Building
You need to install [cargo](https://github.com/rust-lang/cargo) for running and building this project.

### Running
Change the working directory
```shell
cd resume-cli/
```

Run the project
```shell
cargo run
```

### Building
Change the working directory
```shell
cd resume-cli
```

Build the project and run
```shell
cargo build && ./resume-cli
```

# Customization
- **Skills**: Edit values for yourself.
    ```toml
    web_technologies = "HTML & CSS"
    languages = "Go, Rust, C#, C++, Bash & Java"
    dev_ops = "Docker"
    ```
    for example change the languages section.


- **Tools**: Edit values for yourself.
    ```toml
    ide = "RustRover & IntelliJ IDEA"
    text_editor = "VSCodium, Helix & Lapce"
    version_control_system = "Git"
    ```
    for example change the ide or text_editor sections.


- **Social**: Edit values for yourself.
    ```toml
    email = "alperakca79@outlook.com"
    github = "github.com/lnxwizard"
    reddit = "u/AlperAkca79"
    gitlab = "gitlab.com/lnxwizard"
    medium = "medium.com/@lnxwizard"
    devto = "dev.to/lnxwizard"
    ```
    for example change the email, github, reddit etc.. sections.

# License
[GNU General Public License v3.0](LICENSE)
