#![allow(unused)]

use serde_json::{json, Value};
use smart_home_api::home_client::RpcOverStpClient;
use smart_home_api::DEFAULT_TCP_SOCKET;
use std::error::Error;
use std::io::stdin;
use std::str::FromStr;

// at server side was init, and serving that SmartHome structure:
//
// "MyHome": "127.0.0.1:55432"
//     "kitchen":
//         "Smart Socket 1", state: Off
//         "Smart Kettle 1", state: On
//     "living":
//         "Smart Socket 2", state: Off
//         "Thermometer 1" , state: On
//     "bedroom":
//         "Smart Socket 3", state: Broken
//         "Thermometer 2" , state: On
//     "storeroom":
//         "Smart Socket 4", state: Off
//         "Smart Kettle 2", state: Off

fn main() -> Result<(), Box<dyn Error>> {
    let batch = include!("../commands_json");

    println!();
    println!("------------------");
    println!("Smart Home Client");
    println!("Input any batch of num of given requests, separated by space");
    println!("for example: 1 2 5, or 1 1 2, and then press ENTER for rr via jsonRPC");
    println!("------------------");
    println!("Predefined requests:");
    println!("0 - addRoom library");
    println!("1 - delRoom kitchen");
    println!("2 - getDevices at living");
    println!("3 - create total report");
    println!("4 - reset afterwards cmds in batch(for ex: [1 2 4 7 8] -> 7,8 won't be executed)");
    println!("5 - deviceExecute: living room, Thermometer 1, get_current_info");
    println!("6 - deviceExecute: storeroom, Smart Socket 6 get_report");
    println!("7 - deviceExecute: bedroom, Smart Socket 3, switch on");
    println!("8 - deviceExecute: storeroom, Smart Socket 4, switch on");
    println!("9 - createProviderReport: schema = 'living': ['Socket 2','Thermo 1']");
    println!("10- deviceExecute: kitchen room, Thermometer 1, get_current_info");
    println!("11- deviceExecute: storeroom, Smart Socket 4, switch off");
    println!("------------------");
    println!();

    loop {
        let mut buf = String::new();

        stdin().read_line(&mut buf).expect("Failed to read line");
        let line = buf.trim();

        let v: Vec<Value> = line
            .split_whitespace()
            .filter_map(|w| usize::from_str(w).ok())
            .map(|x| batch[x].clone())
            .collect();

        let mut client = RpcOverStpClient::new(DEFAULT_TCP_SOCKET)?;

        let req = Value::Array(v); // send as batch,  even if single request.

        println!("request:\n{:#}", req);
        println!();
        println!("response:\n");
        // server will reply with JsonRPC 2.0, so, u can deser it, cool!

        match client.rr(req) {
            Ok(resp) => {
                println!("{resp:#}");
            }
            Err(err) => {
                println!("request failed, err: {:#}", err);
            }
        };
    }
}
