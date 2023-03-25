use tokio::{
    io::{AsyncBufReadExt, AsyncWriteExt, BufReader},
    net::TcpStream,
};
use std::io::stdin;
use chrono::Local;
use openssl::{
    rsa::Rsa,
    symm::Cipher
};

#[tokio::main]
async fn main() {
    let mut name = String::new();
    println!("What is your name:");
    let _ = stdin().read_line(&mut name).unwrap();

    let mut passphrase = String::new();

    println!("Creating RSA Key....");
    println!("Input passphrase for RSA Key:");

    let _ = stdin().read_line(&mut passphrase).unwrap();
    let rsa = Rsa::generate(2048).unwrap();
    let _private_key: Vec<u8> = rsa.private_key_to_pem_passphrase(Cipher::aes_128_cbc(), passphrase.as_bytes()).unwrap();
    let public_key: Vec<u8> = rsa.public_key_to_pem().unwrap();

    println!("RSA Key Created\nInitiating steps for server connection");

    let mut ip_addr = String::new();
    let mut port_1 = String::new();
    println!("Enter the IP address of the server:");
    stdin().read_line(&mut ip_addr).unwrap();
    println!("Enter the port you will be using");
    stdin().read_line(&mut port_1).unwrap();

    if ip_addr.find("\r") != None {
        ip_addr.pop();
        ip_addr.pop();

        port_1.pop();
        port_1.pop();
    } else {
        ip_addr.pop();
        port_1.pop();
    }

    ip_addr.push(':');
    ip_addr.push_str(&port_1);

    println!("{}", &ip_addr);

    let stream = TcpStream::connect(ip_addr).await.unwrap();
    println!("CONNECTED {}", Local::now());
    let (stream_read, mut stream_write) = stream.into_split();
    let mut reader_1 = BufReader::new(stream_read);

    println!("Sending Public RSA Key to the other user");
    stream_write.write_all(&public_key).await.unwrap();

    println!("Recieving RSA Key...");
    let mut public_key_pem = String::new();
    reader_1.read_line(&mut public_key_pem).await.unwrap();
    println!("{}", public_key_pem);
    let _rsa = Rsa::public_key_from_pem(public_key_pem.as_bytes()).unwrap(); // Make this return a vector
}