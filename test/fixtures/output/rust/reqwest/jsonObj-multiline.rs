use reqwest::blocking::Client;
use serde_json::json;

fn main() {
    let client = Client::new();

    let json = json!("{\"foo\":\"bar\"}");

    let resp = client
        .post("http://mockbin.com/har")
        .json(&json)
        .send()
        .unwrap();

    println!("{:?}", resp);
}
