use std::fs::File;
use std::io;
use std::io::BufRead;
use std::process::exit;

fn main() {
    loop {
        println!("읽고 싶은 파일의 위치를 입력해주세요.");
        let mut file_dir = String::new();
        io::stdin().read_line(&mut file_dir)
            .expect("입력에 실패하였습니다.");

        let file = match File::open(file_dir.trim()) {
            Ok(file) => file,
            Err(e) => {
                println!("{}", e);
                exit(1);
            }
        };

        let reader = io::BufReader::new(file);
        for line in reader.lines() {
            println!("{}", line.unwrap());
        }

        println!("종료할까요?");
        println!("1: 예, 2: 아니오");

        let mut input = String::new();
        io::stdin().read_line(&mut input)
            .expect("입력에 실패하였습니다.");

        let input: i8 = match input.trim().parse() {
            Ok(input) => input,
            Err(_e) => {
                println!("잘못 입력하였습니다.");
                continue;
            }
        };

        if input == 1 {
            break;
        }
    }

}

