#![allow(unused_imports)]

use tokio::{
    io::{AsyncBufReadExt, AsyncWriteExt, BufReader},
    net::TcpListener,
};
use serde::Deserialize;
use get_if_addrs;
use std::fs;
use get_if_addrs::Interface;

#[derive(Debug, Deserialize)]
struct Data {
    ip_address: String,
    listening_ports: Vec<String>
}

#[tokio::main]
async fn main() {
    let configuration_file = fs::read_to_string("./server/Resources/data.json").expect("Should have been able to read the file");
    let json: Data = serde_json::from_str(&configuration_file).expect("JSON was not well-formatted");
    
    let listener_1 = TcpListener::bind(concat_ip_and_port(&json.ip_address, &json.listening_ports[0])).await.unwrap();
    let listener_2 = TcpListener::bind(concat_ip_and_port(&json.ip_address, &json.listening_ports[1])).await.unwrap();


    loop {
        let (socket_1, _addr1) = listener_1.accept().await.unwrap();
        let (socket_1_read, mut socket_1_write) = socket_1.into_split();
        let mut reader_1 = BufReader::new(socket_1_read);
        let mut line_1 = String::new();

        let (socket_2, _addr2) = listener_2.accept().await.unwrap();
        let (socket_2_read, mut socket_2_write) = socket_2.into_split();
        let mut reader_2 = BufReader::new(socket_2_read);
        let mut line_2 = String::new();

        tokio::spawn(async move {
            loop {
                let read_bytes = reader_1.read_line(&mut line_1).await.unwrap();
                if read_bytes == 0 {
                    break;
                }
                socket_2_write.write_all(line_1.as_bytes()).await.unwrap();
                line_1.clear();
            }
        });

        tokio::spawn(async move  {
            loop {
                let read_bytes = reader_2.read_line(&mut line_2).await.unwrap();
                if read_bytes == 0 {
                    break;
                }
                socket_1_write.write_all(line_2.as_bytes()).await.unwrap();
                line_2.clear();
            }
        });
    }
}

fn concat_ip_and_port(ip_addr: &str, port: &str) -> String {
    return String::from(ip_addr) + ":" + port;
}