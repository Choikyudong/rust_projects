use std::collections::HashMap;
use std::io::{BufRead, BufReader, Write};
use std::net::TcpListener;

fn main() {
    println!("Redis... start");

    let mut db: HashMap<String, String> = HashMap::new();

    let listener = TcpListener::bind("127.0.0.1:6379").unwrap();
    for stream in listener.incoming() {
        let mut stream = stream.unwrap();

        loop {
            let mut reader = BufReader::new(&mut stream);
            let mut lines = String::new();
            match reader.read_line(&mut lines) {
                Ok(0) => {
                    println!("DisConnect");
                    break;
                }
                Ok(_) => {} // 정상, 아래 로직 계속
                Err(_) => {
                    println!("SERVER ERROR");
                    break;
                }
            }

            let commands: Vec<&str> = lines.split_whitespace().collect();
            let mut response = String::new();
            match commands.as_slice() {
                ["SET", key, val] => {
                    db.insert(key.to_string(), val.to_string());
                    stream.write_all("OK\n".as_bytes()).unwrap();
                    stream.write_all("END\n".as_bytes()).unwrap();
                },
                ["GET", key] => {
                    match db.get(*key) {
                        Some(val) => stream.write_all(&format!("{}\n", val).as_bytes()).unwrap(),
                        None => stream.write_all("null\n".as_bytes()).unwrap(),
                    }
                    stream.write_all("END\n".as_bytes()).unwrap();
                },
                ["DEL", key] => {
                    db.remove(*key);
                    stream.write_all("OK\n".as_bytes()).unwrap();
                    stream.write_all("END\n".as_bytes()).unwrap();
                },
                ["SHUTDOWN"] => {
                    println!("Bye bye!");
                    break;
                },
                ["KEYS"] => {
                    for (key, _) in db.iter() {
                        response.push_str(&format!("{} {}\n", key, db.get(key).unwrap()));
                    }
                    stream.write_all(response.as_bytes()).unwrap();
                    stream.write_all("END\n".as_bytes()).unwrap();
                },
                ["SIZE"] => {
                    response.push_str(&format!("{}\n", db.len().to_string().as_str()));
                    stream.write_all(response.as_bytes()).unwrap();
                    stream.write_all("END\n".as_bytes()).unwrap();
                }
                _ => {
                    stream.write_all("ERROR : UNKNOWN COMMAND\n".as_bytes()).unwrap();
                }
            }
        }
    }
}