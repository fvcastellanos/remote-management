extern crate clap;

use clap::{ App, SubCommand, Arg };

pub const PING: &str = "ping";
pub const CONFIG: &str = "config";

pub fn build_application() -> App<'static, 'static> {

    App::new("Remote Management")
        .author("DoRefactor")
        .about("Management tool for remote hosts")
        .version("1.0")
        .subcommand(ping_command())
        .subcommand(config_command())
}

// ------------------------------------------------------------------------

fn ping_command() -> App<'static, 'static> {

    SubCommand::with_name(PING)
        .about("Ping remote host")
}

fn config_command() -> App<'static, 'static> {

    SubCommand::with_name(CONFIG)
        .about("Initial configuration")
        .arg(
            Arg::with_name("list")
                .takes_value(false)
                .short("l")
                .long("list")
                .help("Display configuration")
        )
}

