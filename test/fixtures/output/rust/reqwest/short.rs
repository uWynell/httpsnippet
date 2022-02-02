use reqwest::blocking::Client;

fn main() {
    let client = Client::new();

    let resp = client
        .get("http://mockbin.com/har")
        .send()
        .unwrap();

    println!("{:?}", resp);
}
