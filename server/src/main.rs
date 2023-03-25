use tokio::{
    io::{AsyncBufReadExt, AsyncWriteExt, BufReader},
    net::TcpListener,
};
use std::io::stdin;
use serde_json::Deserializer;
use get_if_addrs;

#[tokio::main]
async fn main() {
    for iface in get_if_addrs::get_if_addrs().unwrap() {
        println!("{:#?}", iface);
    }
    /* 
    let mut copy_addr = ip_addr.clone();

    ip_addr.push(':');
    ip_addr.push_str(&port_1);

    copy_addr.push(':');
    copy_addr.push_str(&port_2);
    

    let listener_1 = TcpListener::bind(ip_addr).await.unwrap();
    let listener_2 = TcpListener::bind(copy_addr).await.unwrap();
    
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
    */
}