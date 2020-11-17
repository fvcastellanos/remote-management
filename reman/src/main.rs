
mod options;
mod shell;
mod configuration;
mod commands;

fn main() {
    let app = options::build_application();

    let matches = app.get_matches();

    match matches.subcommand_name() {

        Some(options::PING) => println!("ping command was selected"),
        Some(options::CONFIG) => commands::configuration(&matches),
        _ => println!("Use reman --help to see app usage")
    }

    // shell::execute("cat", &[ "/etc/hosts" ], true);
}
