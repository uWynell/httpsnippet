use reqwest::blocking::Client;
use reqwest::header::HeaderMap;

fn main() {
    let client = Client::new();

    let mut headers = HeaderMap::new();

    let resp = client
        .get("http://mockbin.com/har?foo=bar&foo=baz&baz=abc&key=value")
        .headers(headers)
        .send()
        .unwrap();

    println!("{:?}", resp);
}
