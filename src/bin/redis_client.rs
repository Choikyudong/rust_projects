use std::io;
use std::io::{BufRead, BufReader, Write};
use std::net::TcpStream;

fn main() {
    let stream = TcpStream::connect("127.0.0.1:6379").unwrap();
    let mut writer = stream.try_clone().unwrap();
    let mut reader = BufReader::new(stream);

    let stdin = io::stdin();

    println!("Redis Connect Success");
    loop {
        print!("> ");
        io::stdout().flush().unwrap();

        let mut input = String::new();
        stdin.lock().read_line(&mut input).unwrap();

        if input.trim() == "EXIT" {
            println!("ByeBye");
            break;
        }

        writer.write_all(input.as_bytes()).unwrap();
        loop {
            let mut response = String::new();
            reader.read_line(&mut response).unwrap();

            if response.trim() == "END" {
                break;
            }
            print!("{}", response);
        }
    }
}