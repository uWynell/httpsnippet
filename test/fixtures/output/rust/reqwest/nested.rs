use reqwest::blocking::Client;
use reqwest::header::HeaderMap;

fn main() {
    let client = Client::new();

    let mut headers = HeaderMap::new();

    let resp = client
        .get("http://mockbin.com/har?foo%5Bbar%5D=baz%2Czap&fiz=buz&key=value")
        .headers(headers)
        .send()
        .unwrap();

    println!("{:?}", resp);
}
