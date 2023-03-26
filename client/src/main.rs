#![allow(unused_imports)]
#![allow(unused_variables)]
#![allow(dead_code)]

pub mod json_structs;

use tokio::{
    io::{AsyncWriteExt, BufReader, AsyncBufReadExt},
    net::TcpStream
};
use chrono::Local;
use openssl::{
    rsa::Rsa,
    symm::Cipher
};
use input_macro::input;
use std::fs;

#[tokio::main]
async fn main() {
    let name = input!("What is your name: ");

    println!("Creating RSA Key....");

    let passphrase = input!("Input passphrase for RSA Key: ");
    let rsa = Rsa::generate(4096).unwrap();
    let _private_key: Vec<u8> = rsa.private_key_to_pem_passphrase(Cipher::aes_128_cbc(), passphrase.as_bytes()).unwrap();
    let public_key: Vec<u8> = rsa.public_key_to_pem().unwrap();

    println!("RSA Key Created\nInitiating steps for server connection");

    let configuration_file = fs::read_to_string("./client/Resources/config.json").expect("Should have been able to read the file");
    let json: json_structs::config::ConfigurationFile = serde_json::from_str(&configuration_file).expect("JSON was not well-formatted");

    let config_option = input!("What configuration would you like to use\n1: Port 7180\n2: Port 7181\n::> ");
    let internal_config = input!("testing or production: ");

    let mut network_config = String::new();

    if internal_config == "testing" {
        network_config = String::from("NULL");
    } else {
        network_config = input!("Are you inside or outside the network: ");
    }

    let mut ip_address: String = String::new();
    let mut port: String = String::new();

    match config_option.parse::<u8>().unwrap() {
        1 => {
            if internal_config == "testing" && network_config == "NULL" {
                ip_address = json.config_1.test.ip_address;
                port = json.config_1.test.listening_port;
            }
            else if internal_config == "production" && network_config == "inside" {
                ip_address = json.config_1.production.inside_network.ip_address;
                port = json.config_1.production.inside_network.listening_port;
            }
            else if internal_config == "production" && network_config == "outside" {
                ip_address = json.config_1.production.outside_network.ip_address;
                port = json.config_1.production.outside_network.listening_port;
            }
            else {
                println!("One of your input values were invalid. Exiting...")
            }
        },
        2 => {
            if internal_config == "testing" && network_config == "NULL" {
                ip_address = json.config_2.test.ip_address;
                port = json.config_2.test.listening_port;
            }
            else if internal_config == "production" && network_config == "inside" {
                ip_address = json.config_2.production.inside_network.ip_address;
                port = json.config_2.production.inside_network.listening_port;
            }
            else if internal_config == "production" && network_config == "outside" {
                ip_address = json.config_2.production.outside_network.ip_address;
                port = json.config_2.production.outside_network.listening_port;
            }
            else {
                println!("One of your input values were invalid. Exiting...")
            }
        }
        _ => {
            println!("Invalid input. Exiting...");
        }
    }

    println!("Configuration: {}", config_option);
    println!("Internal Configuration: {}", internal_config);
    println!("Network Configuration: {}", network_config);
    println!("Ip Address: {}", ip_address);
    println!("Port: {}", port);


    let stream = TcpStream::connect(concat_ip_and_port(&ip_address, &port)).await.unwrap();
    println!("CONNECTED {}", Local::now());
    let (stream_read, mut stream_write) = stream.into_split();
    let reader_1 = BufReader::new(stream_read);

    println!("Sending Public RSA Key to the other user");
    stream_write.write_all(&public_key).await.unwrap();

    println!("Recieving RSA Key...");
    let mut public_key_pem = String::new();
    let mut lines = reader_1.lines();

    for _ in 0..9 {
        let line = lines.next_line().await.unwrap();
        public_key_pem.push_str(&line.unwrap());
        public_key_pem.push('\n');
    }

    let _rsa = Rsa::public_key_from_pem(public_key_pem.as_bytes()).unwrap();
}

fn concat_ip_and_port(ip_addr: &str, port: &str) -> String {
    return String::from(ip_addr) + ":" + port;
}