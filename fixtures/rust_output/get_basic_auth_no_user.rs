extern crate reqwest;

fn main() -> Result<(), reqwest::Error> {

    let res = reqwest::Client::new()
        .get("https://api.test.com/")
        .basic_auth("", Some("some_password"))
        .send()?
        .text()?;
    println!("{}", res);

    Ok(())
}
