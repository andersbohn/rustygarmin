mod gcdomain;

use std::fs;
use gcdomain::Activity;

fn main() {
    println!("Hello, world!");
    let path = "./resources/local/gc-1972-01-01-2024-09-05.json";
    let data = fs::read_to_string(path);
    match data {
        Ok(jsonStr) => {
        let res = serde_json::from_str::<Vec<Activity>>(&jsonStr);
        match res {
            Ok(json) => {
                let prettyJson = serde_json::to_string_pretty(&json).expect("badpretty");
                let chopped  = &prettyJson[..100];
                println!("amazing json {}",chopped);
            }
            Err(bad) => {
                println!("baad json !! {}", bad);
            }
        }
        }
        Err(why) =>  println!("Open file failed : {:?}", why.kind()),
    }
}
