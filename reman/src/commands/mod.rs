use clap::ArgMatches;
use crate::configuration;
use ansi_term::Colour::{ Cyan };

pub fn configuration(matches: &ArgMatches) {

    println!("{}", Cyan.paint("ReMan CLI configuration"));
    println!("{}", Cyan.paint("-----------------------"));

    if matches.is_present("list") {

        let config = configuration::read_configuration();
        println!("foo: {}", config);
        return;
    }

    configuration::init_configuration();
}
