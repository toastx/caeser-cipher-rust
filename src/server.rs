use std::io::{Read, Write};
use std::net::{TcpListener, TcpStream};
use std::thread;

fn decrypt_caesar(cipher_text: &str, shift: i8) -> String {
    cipher_text.chars()
        .map(|c| {
            if c.is_ascii_alphabetic() {
                let a = if c.is_ascii_lowercase() { b'a' } else { b'A' };
                (((c as u8 - a + 26 - shift as u8) % 26) + a) as char
            } else {
                c
            }
        })
        .collect()
}

fn handle_client(mut stream: TcpStream) {
    let mut buffer = [0; 512];
    loop {
        match stream.read(&mut buffer) {
            Ok(0) => break, 
            Ok(n) => {
                let received_data = String::from_utf8_lossy(&buffer[..n]);
                println!("Received from client: {}", received_data);

                let correct_data = decrypt_caesar(&received_data,3);
                println!("Decrypted data {}",correct_data);
            }
            Err(_) => {
                println!("Error reading from stream.");
                break;
            }
        }
    }
}

fn main() -> std::io::Result<()> {
    let listener = TcpListener::bind("127.0.0.1:7878")?;
    println!("Server listening on port 7878");
    for stream in listener.incoming() {
        match stream {
            Ok(stream) => {
                thread::spawn(|| {
                    handle_client(stream);
                });
            }
            Err(e) => {
                println!("Connection failed: {}", e);
            }
        }
    }
    Ok(())
}
