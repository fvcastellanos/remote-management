
mod commands;

fn main() {
    let app = commands::build_application();

    let matches = app.get_matches();

    match matches.subcommand_name() {

        Some(commands::PING) => println!("ping command was selected"),
        Some(commands::CONFIG) => println!("config command was selected"),
        _ => println!("everything else")
    }
}
