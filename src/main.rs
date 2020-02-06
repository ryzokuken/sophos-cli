extern crate clap;
use clap::{Arg, App};

mod error;

use error::SophosdError;
use std::error::Error;
use xml::reader::{EventReader, XmlEvent};

fn validate_login(username: &str, password: &str) -> Result<(), error::SophosdError> {
    if username.replace(" ", "").is_empty() {
        return Err(SophosdError::new_username());
    } else if password.is_empty() {
        return Err(SophosdError::new_password());
    }
    Ok(())
}

fn login(username: &str, password: &str) -> Result<(), Box<dyn Error>> {
    validate_login(username, password)?;
    let params = [
        ("mode", "191"),
        ("username", username),
        ("password", password),
    ];
    let client = reqwest::Client::new();
    let res = client
        .post("http://172.16.68.6:8090/login.xml")
        .form(&params)
        .send();

    let text = res?.text()?;
    let parser = EventReader::from_str(text.as_str());
    for e in parser {
        if let Ok(XmlEvent::CData(val)) = e {
            println!("{}", val);
        }
    }
    Ok(())
}

fn main() -> Result<(), Box<dyn Error>> {
	let sopd = App::new("sophosd")
						  .version("0.1")
						  .about("A CLI client to help you connect to Sophos")
						  .author("Ujjwal Sharma")

						  .arg(Arg::with_name("username")
						      .short("u")
							  .value_name("USERNAME")
							  .required(true)
						      .takes_value(true)
						      .number_of_values(1)
							  .help("Your Enrollment Number"))
							  
						  .arg(Arg::with_name("password")
						      .required(true)
							  .short("p")
							  .value_name("PASSWORD")
							  .takes_value(true)
							  .number_of_values(1)
							  .help("Your Password"))

						  .get_matches();

	let username = sopd.value_of("username").unwrap();
	let password = sopd.value_of("password").unwrap();

    login(username, password)
}
