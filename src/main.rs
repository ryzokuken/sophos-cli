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
    let args: Vec<String> = std::env::args().collect();
    let username = args[1].as_str();
    let password = args[2].as_str();
    login(username, password)
}
