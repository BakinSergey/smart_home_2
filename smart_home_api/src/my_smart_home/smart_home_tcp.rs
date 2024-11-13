#![allow(unused_assignments)]

use crate::info_provider::json_provider::JsonDeviceInfoProvider;
use crate::json_rpc::error::{
    api_error, internal_error, invalid_method, invalid_param, invalid_request, parse_error,
    unhandled_error,
};
use crate::json_rpc::reply::{reply, reply_error, JsonRpcReplyMsg};
use crate::my_smart_home::home::Home;
use crate::my_smart_home::smart_home::SmartHome;
use serde_json as json;

use std::error::Error;
use std::fs;

pub trait SmartHomeTcp {
    /// запустить цикл обслуживания клиентов
    fn serve(&mut self) -> Result<(), Box<dyn Error>>;
    /// выполнить RPC-запрос клиента
    fn execute(&mut self) -> String;
}

impl SmartHomeTcp for Home {
    fn serve(&mut self) -> Result<(), Box<dyn Error>> {
        let jdata: Result<String, std::io::Error> = fs::read_to_string("smart_home_api/api.json");
        let s = match jdata {
            Ok(s) => s,
            Err(_) => panic!("json schema file read failed"),
        };

        // jsonschema, по которой будем валидировать JsonRPC запросы к API
        let schema = json::from_str(&s).expect("parse json failed");
        let validator = match jsonschema::validator_for(&schema) {
            Ok(validator) => validator,
            Err(e) => panic!("{}", format!("parse schema error: {e}!")),
        };

        // цикл обработки запросов.

        loop {
            let Ok(mut connection) = self.bus.accept() else {
                continue;
            };

            let addr = match connection.peer_addr() {
                Ok(addr) => addr.to_string(),
                Err(_) => "unknown".into(),
            };

            println!("request received from: {:?}", addr);

            connection.process_request(|req| {
                let binding = json::from_str(&req);
                let data = match &binding {
                    Ok(jsondata) => jsondata,
                    Err(e) => return json::to_string(&parse_error(e.to_string())).unwrap(),
                };

                match validator.is_valid(data) {
                    true => {
                        let batch = json::from_value(data.to_owned()).unwrap();
                        self.requests.push(batch);
                        self.execute() // collect butch of result
                    }

                    false => {
                        let mut errors: Vec<String> = Vec::new();
                        for e in validator.iter_errors(data) {
                            let er = format!("Error: {}\n\n Location: {}\n\n", e, e.instance_path);
                            errors.push(er);
                        }
                        json::to_string(&invalid_request(errors.join(";"))).unwrap()
                    }
                }
            })?; // emit only tcp level errors
        }
    }

    fn execute(&mut self) -> String {
        let mut replies: Vec<Box<dyn JsonRpcReplyMsg>> = vec![];

        while let Some(rpc_cmd) = self.requests.pop() {
            let mut error_code: i32 = 0; // 1 - SmartHome Api Errors
            let resp = match rpc_cmd.method.as_str() {
                "addRoom" => {
                    let room_name = rpc_cmd.params["name"].to_string();
                    match self.add_room(room_name, vec![]) {
                        Ok(()) => "addRoom: success".to_string(),
                        Err(e) => {
                            error_code = 1;
                            format!("addRoom error: {e}")
                        }
                    }
                }
                "delRoom" => {
                    let room_name = rpc_cmd.params["name"].to_string();
                    match self.del_room(&room_name) {
                        Ok(()) => "delRoom: success".to_string(),
                        Err(e) => {
                            error_code = 1;
                            format!("delRoom error: {e}")
                        }
                    }
                }

                "getDevices" => {
                    let room_name = rpc_cmd.params["name"].to_string();
                    match self.get_devices(&room_name) {
                        Ok(res) => res
                            .iter()
                            .map(|x| x.to_string())
                            .collect::<Vec<_>>()
                            .join(";"),
                        Err(e) => {
                            error_code = 1;
                            format!("error: {e}")
                        }
                    }
                }
                "createReport" => self.create_report(),

                "createProviderReport" => {
                    let schema = rpc_cmd.params.clone()["provider"].clone();
                    match JsonDeviceInfoProvider::from_json(schema) {
                        Ok(provider) => self.create_provider_report(&provider),
                        Err(e) => {
                            error_code = -32602;
                            format!("error: {e}")
                        }
                    }
                }

                "reset" => {
                    self.requests.reset();
                    "reset: success".to_string()
                }

                "deviceExecute" => {
                    let room = rpc_cmd.params["room"].as_str().unwrap();
                    let device = rpc_cmd.params["device"].as_str().unwrap();
                    let command = rpc_cmd.params["command"].as_str().unwrap();
                    let data = rpc_cmd.params.clone()["data"].clone();

                    let res = match self.mut_device(room, device) {
                        Ok(dev) => {
                            return match command {
                                "get_name" => dev.get_name(),
                                "get_description" => dev.get_description(),
                                "get_current_info" => dev.get_current_info(),
                                "report" => dev.report(),
                                "switch" => match data[0].as_str() {
                                    Some(state) => dev.switch(state),
                                    None => {
                                        error_code = -32602;
                                        format!("wrong status provided: {}", data.as_str().unwrap())
                                    }
                                },
                                _ => {
                                    error_code = -32601;
                                    "wrong device command".to_string()
                                }
                            }
                        }
                        Err(e) => {
                            error_code = -32602;
                            format!("error: {e}")
                        }
                    };
                    // println!("{res}");
                    res
                }
                _ => {
                    error_code = -32603; // UB :))
                    "непредвиденный ответ на непредвиденный запрос :)".to_string()
                }
            };
            let dto: Box<dyn JsonRpcReplyMsg> = match error_code {
                0 => Box::new(reply(rpc_cmd.id, resp)),
                1 => Box::new(reply_error(rpc_cmd.id, api_error(resp))),
                -32601 => Box::new(reply_error(rpc_cmd.id, invalid_method(resp))),
                -32602 => Box::new(reply_error(rpc_cmd.id, invalid_param(resp))),
                -32603 => Box::new(reply_error(rpc_cmd.id, internal_error(resp))),
                _ => Box::new(reply_error(rpc_cmd.id, unhandled_error(resp))),
            };
            replies.push(dto);
        }

        json::to_string_pretty(&replies).unwrap()
    }
}
