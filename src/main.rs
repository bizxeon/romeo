use std::{process::abort, net::SocketAddr};
use serde::{Serialize, Deserialize};
use tokio::{self, net::{TcpListener, TcpStream}, task, io::{AsyncReadExt, AsyncWriteExt}};

const NIC_DEAFULT_MTU: usize = 1500;

#[derive(Debug, Default, Serialize, Deserialize, Clone)]
enum RomeoConfigValueBlockingEnum {
    #[default]
    Blocking,
    Passing,
}

#[derive(Debug, Default, Serialize, Deserialize, Clone)]
enum RomeoConfigValueHttpMethodEnum {
    #[default]
    All,
    Get,
    Head,
    Post,
    Put,
    Delete,
    Connect,
    Options,
    Trace,
    Patch,
}

#[derive(Debug, Default, Serialize, Deserialize, Clone)]
struct RomeoLocationRule {
    ingress_rule: RomeoConfigValueBlockingEnum,
    location: String,
    method: RomeoConfigValueHttpMethodEnum,
}

#[derive(Debug, Default, Serialize, Deserialize, Clone)]
struct RomeoConfigGeneral {
    listening_port: u16,
    destination: String,
    destination_port: u16,
    location_rules: Vec<RomeoLocationRule>,
}


#[derive(Default)]
struct ClientRequestProcessObject {
    client_request_header: String,
}

fn client_process_request(config: &RomeoConfigGeneral, object: &mut ClientRequestProcessObject, block: &[u8], clientaddr: &SocketAddr) -> bool {
    match String::from_utf8(block.to_vec()) {
        Ok(block) => {                                                            
            if let Some(end_of_header) = block.find("\r\n\r\n") {
                object.client_request_header.push_str(&block.as_str()[..(end_of_header + 4)]);

                let mut http_headers = [httparse::EMPTY_HEADER; 128];
                let mut http_request = httparse::Request::new(&mut http_headers);

                let request_data = object.client_request_header.clone();
                object.client_request_header = String::new();

                match http_request.parse(request_data.as_bytes()) {
                    Ok(httparse::Status::Complete(_)) => {
                        if let Some(request_method) = http_request.method {
                            if let Some(request_path) = http_request.path {
                                for rule in &config.location_rules {
                                    match rule.method.clone() {
                                        RomeoConfigValueHttpMethodEnum::All => {
                                            if request_path.to_lowercase() == rule.location.to_lowercase() {
                                                match rule.ingress_rule.clone() {
                                                    RomeoConfigValueBlockingEnum::Blocking => {
                                                        return true;
                                                    },
                                                    RomeoConfigValueBlockingEnum::Passing => { },
                                                };
                                            }
                                        },
                                        RomeoConfigValueHttpMethodEnum::Get => {
                                            if (request_method.to_uppercase() == "GET") && (request_path.to_lowercase() == rule.location.to_lowercase()) {
                                                match rule.ingress_rule.clone() {
                                                    RomeoConfigValueBlockingEnum::Blocking => {
                                                        return true;
                                                    },
                                                    RomeoConfigValueBlockingEnum::Passing => { },
                                                };
                                            }
                                        },
                                        RomeoConfigValueHttpMethodEnum::Head => {
                                            if (request_method.to_uppercase() == "HEAD") && (request_path.to_lowercase() == rule.location.to_lowercase()) {
                                                match rule.ingress_rule.clone() {
                                                    RomeoConfigValueBlockingEnum::Blocking => {
                                                        return true;
                                                    },
                                                    RomeoConfigValueBlockingEnum::Passing => {
                                                        return false;
                                                    },
                                                };
                                            }
                                        },
                                        RomeoConfigValueHttpMethodEnum::Post => {
                                            if (request_method.to_uppercase() == "POST") && (request_path.to_lowercase() == rule.location.to_lowercase()) {
                                                match rule.ingress_rule.clone() {
                                                    RomeoConfigValueBlockingEnum::Blocking => {
                                                        return true;
                                                    },
                                                    RomeoConfigValueBlockingEnum::Passing => {
                                                        return false;
                                                    },
                                                };
                                            }
                                        },
                                        RomeoConfigValueHttpMethodEnum::Put => {
                                            if (request_method.to_uppercase() == "PUT") && (request_path.to_lowercase() == rule.location.to_lowercase()) {
                                                match rule.ingress_rule.clone() {
                                                    RomeoConfigValueBlockingEnum::Blocking => {
                                                        return true;
                                                    },
                                                    RomeoConfigValueBlockingEnum::Passing => {
                                                        return false;
                                                    },
                                                };
                                            }
                                        },
                                        RomeoConfigValueHttpMethodEnum::Delete => {
                                            if (request_method.to_uppercase() == "DELETE") && (request_path.to_lowercase() == rule.location.to_lowercase()) {
                                                match rule.ingress_rule.clone() {
                                                    RomeoConfigValueBlockingEnum::Blocking => {
                                                        return true;
                                                    },
                                                    RomeoConfigValueBlockingEnum::Passing => {
                                                        return false;
                                                    },
                                                };
                                            }
                                        },
                                        RomeoConfigValueHttpMethodEnum::Connect => {
                                            if (request_method.to_uppercase() == "CONNECT") && (request_path.to_lowercase() == rule.location.to_lowercase()) {
                                                match rule.ingress_rule.clone() {
                                                    RomeoConfigValueBlockingEnum::Blocking => {
                                                        return true;
                                                    },
                                                    RomeoConfigValueBlockingEnum::Passing => {
                                                        return false;
                                                    },
                                                };
                                            }
                                        },
                                        RomeoConfigValueHttpMethodEnum::Options => {
                                            if (request_method.to_uppercase() == "OPTIONS") && (request_path.to_lowercase() == rule.location.to_lowercase()) {
                                                match rule.ingress_rule.clone() {
                                                    RomeoConfigValueBlockingEnum::Blocking => {
                                                        return true;
                                                    },
                                                    RomeoConfigValueBlockingEnum::Passing => {
                                                        return false;
                                                    },
                                                };
                                            }
                                        },
                                        RomeoConfigValueHttpMethodEnum::Trace => {
                                            if (request_method.to_uppercase() == "TRACE") && (request_path.to_lowercase() == rule.location.to_lowercase()) {
                                                match rule.ingress_rule.clone() {
                                                    RomeoConfigValueBlockingEnum::Blocking => {
                                                        return true;
                                                    },
                                                    RomeoConfigValueBlockingEnum::Passing => {
                                                        return false;
                                                    },
                                                };
                                            }
                                        },
                                        RomeoConfigValueHttpMethodEnum::Patch => {
                                            if (request_method.to_uppercase() == "PATCH") && (request_path.to_lowercase() == rule.location.to_lowercase()) {
                                                match rule.ingress_rule.clone() {
                                                    RomeoConfigValueBlockingEnum::Blocking => {
                                                        return true;
                                                    },
                                                    RomeoConfigValueBlockingEnum::Passing => {
                                                        return false;
                                                    },
                                                };
                                            }
                                        },
                                    }
                                }
                            }
                        }

                        // println!("complete {}", object.client_request_header);
                    },
                    Ok(httparse::Status::Partial) => {
                        eprintln!("received partial header from client '{}', closing the connection", &clientaddr);
                        return true;
                    },
                    Err(err) => {
                        eprintln!("failed to parse the client '{}' request, error: {}", &clientaddr, err.to_string());
                        return true;
                    }
                }
            } else {
                object.client_request_header.push_str(&block.as_str());
            }

            if object.client_request_header.len() > (64 * 1024) {
                eprintln!("client '{}' send header larger than 32 KiB, closing the connection", &clientaddr.to_string());
                return true;
            }
        },
        Err(err) => {
            eprintln!("failed to parse the header of client '{}', error: {}", &clientaddr.to_string(), err.to_string());
            return true;
        }
    }

    false
}

async fn client_edge_server_procedure(config: RomeoConfigGeneral, mut client: TcpStream, clientaddr: SocketAddr) {
    println!("connection accepted: {}", clientaddr.to_string());

    let edge_server_ip = format!("{}:{}", config.destination, &config.destination_port);
    let mut edge_server = match TcpStream::connect(edge_server_ip).await {
        Ok(value) => { value },
        Err(err) => {
            eprintln!("failed to connect to the edge server, error: {}", err.to_string());
            return;
        }
    };

    let mut client_process_object: ClientRequestProcessObject = ClientRequestProcessObject::default();
    let mut client_buffer = [0 as u8; NIC_DEAFULT_MTU];
    let mut edge_buffer = [0 as u8; NIC_DEAFULT_MTU];

    loop {
        tokio::select! {
            client_read = client.read(&mut client_buffer) => {
                match client_read {
                    Ok(0) => {
                        eprintln!("reading 0 bytes from the client '{}', closing the connection", clientaddr.to_string());
                        return;
                    },
                    Ok(size) => {
                        if client_process_request(&config, &mut client_process_object, &client_buffer[..size], &clientaddr) == true {
                            println!("request blocked from client '{}'", clientaddr.to_string());
                            return;
                        }

                        match edge_server.write(&client_buffer[..size]).await {
                            Ok(size) if size == 0 => {
                                eprintln!("client '{}' did send 0 bytes to the edge server, closing connection", &clientaddr.to_string());
                                return;
                            }
                            Ok(_) => {},
                            Err(err) => {
                                eprintln!("client '{}' failed to send to the edge server, error: {}", &clientaddr.to_string(), err.to_string());
                                return;
                            }
                        }
                    },
                    Err(err) => {
                        eprintln!("failed to read from client '{}', error: {}", &clientaddr.to_string(), err.to_string());
                        return;
                    }
                }
            }
    
            edge_read = edge_server.read(&mut edge_buffer) => {
                match edge_read {
                    Ok(0) => {
                        eprintln!("reading 0 bytes from the edge for client '{}', closing the connection", &clientaddr.to_string());
                        return;
                    },
                    Ok(size) => {
                        match client.write(&edge_buffer[..size]).await {
                            Ok(size) if size == 0 => {
                                eprintln!("edge did send 0 bytes to client '{}', closing connection", &clientaddr.to_string());
                                return;
                            }
                            Ok(_) => {},
                            Err(err) => {
                                eprintln!("edge server failed to send to client '{}', error: {}", &clientaddr.to_string(), err.to_string());
                                return;
                            }
                        }
                    },
                    Err(err) => {
                        eprintln!("failed to read from edge server for client '{}', error: {}", &clientaddr.to_string(), err.to_string());
                        return;
                    }
                }
            }
        }
    }
}

#[tokio::main]
async fn main() {
    let mut config_general: RomeoConfigGeneral = RomeoConfigGeneral::default();

    config_general.destination = "192.168.122.211".to_string();
    config_general.destination_port = 80;
    config_general.listening_port = 8080;
    config_general.location_rules.push(RomeoLocationRule { 
        ingress_rule: RomeoConfigValueBlockingEnum::Blocking,
        location: "/test.html".to_string(),
        method: RomeoConfigValueHttpMethodEnum::All
    });

    let listen_nic = format!("0.0.0.0:{}", &config_general.listening_port);

    match TcpListener::bind(&listen_nic).await {
        Ok(listener) => {
            loop {
                match listener.accept().await {
                    Ok((socket, socketaddr)) => {
                        task::spawn(client_edge_server_procedure(config_general.clone(), socket, socketaddr));
                    },
                    Err(err) => {
                        eprintln!("failed to accept a connection, error: {}", err.to_string());
                    }
                }
            }
        },
        Err(err) => {
            eprintln!("failed to start listening on '{}', error: {}", &listen_nic, err.to_string());
            abort();
        }
    }
}
