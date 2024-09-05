mod gcdomain;

use std::fs;

use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
struct Activity {
    name: String,
    age: u8,
    phones: Vec<String>,
}


fn main() {
    println!("Hello, world!");
    let path = "./gc-1972-01-01-2024-09-05.json";
    let data = fs::read_to_string(path);
    match data {
        Ok(jsonStr) => {
        let res = serde_json::from_str::<serde_json::Value>(&jsonStr);
        match res {
            Ok(json) => {
                println!("amazing json {}", json);
            }
            Err(bad) => {
                println!("baad json !! {}", bad);
            }
        }
        }
        Err(why) =>  println!("Open file failed : {:?}", why.kind()),
    }
}
