use reqwest::blocking::Client;

fn main() {
    let client = Client::new();

    let body = "-----011000010111000001101001\r\nContent-Disposition: form-data; name=\"foo\"; filename=\"hello.txt\"\r\nContent-Type: text/plain\r\n\r\n\r\n-----011000010111000001101001--\r\n";

    let resp = client
        .post("http://mockbin.com/har")
        .body(body)
        .send()
        .unwrap();

    println!("{:?}", resp);
}
