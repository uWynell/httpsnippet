use reqwest::blocking::Client;
use maplit::hashmap;

fn main() {
    let client = Client::new();

    let form = hashmap!{
        "foo" => vec!["bar"],
        "hello" => vec!["world"],
    };

    let resp = client
        .post("http://mockbin.com/har")
        .form(&form)
        .send()
        .unwrap();

    println!("{:?}", resp);
}
