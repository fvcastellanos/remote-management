extern crate serde;

use std::fs::File;
use std::io::prelude::*;

use dialoguer::{theme::ColorfulTheme, Confirm, Input};
use serde::{Serialize, Deserialize};

use ansi_term::Colour::{ Yellow, Cyan };

#[derive(Serialize, Deserialize, Debug)]
pub struct Configuration {
    host: String,
    user: String,
    private_key_path: String
}

pub fn init_configuration() {
 
    if Confirm::with_theme(&ColorfulTheme::default())
        .with_prompt("Do you want to configure RemMan CLI?")
        .interact()
        .unwrap()
    {
        build_configuration();
    } else {
        println!("{}", Yellow.paint("Please execute this command to configure this application"));
    }
}

pub fn read_configuration() -> String {

    let mut file = std::fs::File::open("/home/fvcg/foo.txt").unwrap();
    let mut contents = String::new();
    file.read_to_string(&mut contents).unwrap();

    contents
}

pub fn get_configuration() -> Configuration {

    let content = read_configuration();
    let configuration: Configuration = serde_json::from_str(&content.as_str())
        .unwrap();

    configuration
}

// ------------------------------------------------------------------------------------

fn build_configuration() {

    let host: String = Input::with_theme(&ColorfulTheme::default())
    .with_prompt("Host name / IP: ")
    .interact_text()
    .unwrap();

    let user: String = Input::with_theme(&ColorfulTheme::default())
    .with_prompt("User configured in remote host: ")
    .interact_text()
    .unwrap();

    let private_key: String;

    if Confirm::with_theme(&ColorfulTheme::default())
        .with_prompt("Do you want to generate SSH key?")
        .interact()
        .unwrap()
    {
        private_key = build_private_key();
    } else {
        private_key = Input::with_theme(&ColorfulTheme::default())
        .with_prompt("Private key configured for remote host: ")
        .interact_text()
        .unwrap();    
    }

    store_configuration(host, user, private_key);
}

fn build_private_key() -> String {

    let private_key: String = Input::with_theme(&ColorfulTheme::default())
    .with_prompt("Path where private key will be stored locally: ")
    .interact_text()
    .unwrap();

    // create private key

    private_key
}

fn store_configuration(host: String, user: String, private_key_path: String) {

    let configuration = Configuration {
        host,
        user,
        private_key_path
    };

    let _ = write_configuration(&configuration);

}

fn write_configuration(configuration: &Configuration) -> std::io::Result<()> {

    let text = serde_json::to_string_pretty(&configuration).unwrap();

    let mut file = File::create("/home/fvcg/foo.txt")?;
    file.write_all(text.as_bytes()).unwrap();
    Ok(())
}
