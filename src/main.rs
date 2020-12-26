mod types;
mod serialization;
mod relayer;

#[macro_use]
extern crate serde_derive;

#[macro_use]
extern crate serde_json;

// use std::fs;
use types::header::Header;
use relayer::Relayer;

#[tokio::main]
async fn main(){
    let relayer: Relayer = Relayer::new("http://127.0.0.1:8545".to_string());

    let header: Header = relayer.get_block_header_by_number("latest".to_string()).await.unwrap();
    println!("{}", serde_json::to_string_pretty(&header).unwrap());

    //let contents = fs::read_to_string("/tmp/t")
        //.expect("Something went wrong reading the file");
    //let header: Header = serde_json::from_str(contents.as_str()).unwrap();
    //println!("{}", serde_json::to_string_pretty(&header).unwrap());
}
