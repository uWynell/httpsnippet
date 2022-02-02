use reqwest::blocking::Client;

fn main() {
    let client = Client::new();

    let resp = client
        .get("https://mockbin.com/har")
        .send()
        .unwrap();

    println!("{:?}", resp);
}
