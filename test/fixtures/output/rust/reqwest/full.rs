use reqwest::blocking::Client;
use reqwest::header::HeaderMap;
use maplit::hashmap;

fn main() {
    let client = Client::new();

    let mut headers = HeaderMap::new();
    headers.insert("accept", "application/json".parse().unwrap());
    headers.insert("content-type", "application/x-www-form-urlencoded".parse().unwrap());

    let form = hashmap!{
        "foo" => vec!["bar"],
    };

    let resp = client
        .post("http://mockbin.com/har?foo=bar&foo=baz&baz=abc&key=value")
        .headers(headers)
        .form(&form)
        .send()
        .unwrap();

    println!("{:?}", resp);
}
