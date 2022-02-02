use reqwest::blocking::Client;

fn main() {
    let client = Client::new();

    let body = "Hello World";

    let resp = client
        .post("http://mockbin.com/har")
        .body(body)
        .send()
        .unwrap();

    println!("{:?}", resp);
}
