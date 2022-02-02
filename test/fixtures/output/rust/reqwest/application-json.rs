use reqwest::blocking::Client;
use serde_json::json;

fn main() {
    let client = Client::new();

    let json = json!("{\"number\":1,\"string\":\"f\\\"oo\",\"arr\":[1,2,3],\"nested\":{\"a\":\"b\"},\"arr_mix\":[1,\"a\",{\"arr_mix_nested\":{}}],\"boolean\":false}");

    let resp = client
        .post("http://mockbin.com/har")
        .json(&json)
        .send()
        .unwrap();

    println!("{:?}", resp);
}
