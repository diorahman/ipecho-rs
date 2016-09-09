extern crate hyper;
extern crate url;

use hyper::client::Client;
use std::io::Read;

fn main() {
  let client = Client::new();
  let mut res = client.get("http://ipecho.net/plain").send().unwrap();
  let mut body = String::new();
  res.read_to_string(&mut body).unwrap();
  println!("{}", body);
}
