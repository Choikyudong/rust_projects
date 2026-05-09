use std::io;
use std::io::{Read, Write};
use std::net::TcpStream;
use std::process::exit;
use std::sync::Arc;

fn main() {
    println!("요청할 주소를 입력해주세요.");
    let mut input = String::new();
    io::stdin().read_line(&mut input).unwrap();

    let (host, path, port) = parse_url(input.trim());
    if port != 80 && port != 443 {
        println!("잘못된 입력입니다.");
        exit(1);
    }

    let mut response = String::new();
    let request_path = format!("{}:{}", host, port);
    let request = format!("GET {} HTTP/1.1\r\nHost: {}\r\nConnection: close\r\n\r\n", path, host);
    if port == 443 {
        let mut root_store = rustls::RootCertStore::empty();
        root_store.add_parsable_certificates(
            rustls_native_certs::load_native_certs().unwrap()
        );

        let config = rustls::ClientConfig::builder()
            .with_root_certificates(root_store)
            .with_no_client_auth();

        let server_name = host.try_into().unwrap();
        let mut conn = rustls::ClientConnection::new(Arc::new(config), server_name).unwrap();

        let mut tcp_stream = TcpStream::connect(request_path).unwrap();
        let mut tls_stream = rustls::Stream::new(&mut conn, &mut tcp_stream);

        tls_stream.write_all(request.as_bytes()).unwrap();
        match tls_stream.read_to_string(&mut response) {
            Ok(_) => {},
            Err(e) if e.kind() == io::ErrorKind::UnexpectedEof => {},
            Err(e) => panic!("{}", e),
        }
    } else {
        let mut tcp_stream = TcpStream::connect(request_path).unwrap();

        tcp_stream.write_all(request.as_bytes()).unwrap();
        tcp_stream.read_to_string(&mut response).unwrap();
    }


    print!("{}", response);

}

fn parse_url(url: &str) -> (String, String, u16) {
    let (without_scheme, port) = if url.starts_with("https://") {
        (url.trim_start_matches("https://"), 443)
    } else {
        (url.trim_start_matches("http://"), 80)
    };

    let (host, path) = match without_scheme.find('/') {
        Some(index) => (&without_scheme[..index], &without_scheme[index..]),
        None => (without_scheme, "/"),
    };

    (host.to_string(), path.to_string(), port)
}