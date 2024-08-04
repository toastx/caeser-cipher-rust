use std::io::{Read, Write};
use std::net::TcpStream;

fn encrypt_caeser(plaintext: &str, shift: u8) -> String{
    plaintext.chars().map(
        |c|{
            if c.is_ascii_alphabetic(){
                let l = if c.is_ascii_lowercase(){b'a'} else {b'A'};
            (((c as u8 - l +shift as u8)%26)+l) as char
            }
            else{
                c
            }
        })
        .collect()
}


fn main() -> std::io::Result<()> {
    let mut stream = TcpStream::connect("127.0.0.1:7878")?;
    println!("Connected to the server!");

    let unmsg = "Hello from client!";
    println!("Data to be send {}",unmsg);
    let msg = encrypt_caeser(unmsg,3);
    stream.write(msg.as_bytes())?;

    let mut buffer = [0; 512];
    let n = stream.read(&mut buffer)?;
    println!("Received from server: {}", String::from_utf8_lossy(&buffer[..n]));

    Ok(())
}