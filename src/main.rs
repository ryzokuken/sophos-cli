use xml::reader::{EventReader, XmlEvent};

// TODO: use Result objects if possible.
fn validate_login(username: &str, password: &str) -> bool {
    if username.replace(" ", "").is_empty() || password.is_empty() {
        return false;
    }
    true
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let args: Vec<String> = std::env::args().collect();
    let username = args[1].as_str();
    let password = args[2].as_str();
    if !validate_login(username, password) {
        panic!("Invalid parameters")
    }

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
