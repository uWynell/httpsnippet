use reqwest::blocking::Client;

fn main() {
    let client = Client::new();

    let resp = client
        .post("http://mockbin.com/har") //! Can't use method "PROPFIND"
        .send()
        .unwrap();

    println!("{:?}", resp);
}
