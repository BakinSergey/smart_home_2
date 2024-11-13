#![allow(unused)]

use serde_json::{json, Value};
use smart_home_api::home_client::RpcOverStpClient;
use smart_home_api::DEFAULT_SOCKET;
use std::error::Error;

fn main() -> Result<(), Box<dyn Error>> {
    let batch = include!("../commands_json");
    let mut client = RpcOverStpClient::new(DEFAULT_SOCKET)?;

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

    let _rpc_0 = batch[0].clone(); // addRoom: library
    let _rpc_1 = batch[1].clone(); // delRoom: kitchen
    let _rpc_2 = batch[2].clone(); // getDevices: living
    let _rpc_3 = batch[3].clone(); // createReport
    let _rpc_4 = batch[4].clone(); // reset (expect no any command in batch
                                   // after reset will not be executed)
    let _rpc_5 = batch[5].clone(); // deviceExecute: living room, Thermometer 1, get_temperature
    let _rpc_6 = batch[6].clone(); // deviceExecute: storeroom, Smart Socket 6 get_report
    let _rpc_7 = batch[7].clone(); // deviceExecute: bedroom, Smart Socket 3, switch on
                                   // (will be error, cannot switch on if broken)
    let _rpc_8 = batch[8].clone(); // deviceExecute: storeroom, Smart Socket 4, switch on (success: off -> on)
    let _rpc_9 = batch[9].clone(); // createProviderReport: provider.schema = "living": ["Smart Socket 2","Thermometer 1"]

    let v: Vec<Value> = vec![_rpc_0, _rpc_3]; // edit for test, feel free :)
    let req = Value::Array(v); // send as batch,  even if single request.

    // server will reply with JsonRPC 2.0, so, u can deser it, cool!
    match client.rr(req) {
        Ok(resp) => {
            println!("{resp}");
        }
        Err(err) => {
            println!("req_1 failed, err: {}", err);
        }
    };
    Ok(())
}
