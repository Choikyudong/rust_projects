use std::io;
use rand::Rng;

fn main() {
    println!("가위, 바위, 보... 게임 시작");

    let mut is_continue: bool = true;
    loop {
        loop {
            println!("가위, 바위, 보 중 하나를 입력하세요!");
            println!("가위: 1, 바위: 2, 보: 3");

            let mut input = String::new();
            io::stdin().read_line(&mut input).expect("읽기 실패...");

            let computer_number = rand::thread_rng().gen_range(1..=3);

            let input: i8 = match input.trim().parse() {
                Ok(num) => num,
                Err(_) => {
                    println!("숫자를 입력하세욧");
                    continue;
                },
            };

            match (input, computer_number) {
                (u, c) if u == c => println!("무승부!"),
                (1, 3) | (2, 1) | (3, 2) => println!("당신의 승리!"),
                _ => println!("당신의 패배 ㅜ.."),
            }

            break;
        }


        loop {
            println!("계속 게임을 할까요?");
            println!("예: 1, 아니요: 2");
            let mut retry = String::new();
            io::stdin().read_line(&mut retry).unwrap();

            let retry = retry.trim();
            if retry == "1" ||  retry == "2" {
                if retry == "2" {
                    is_continue = false;
                }
                break;
            } else {
                println!("잘못 입력! input={retry}")
            }
        }

        if !is_continue {
            break;
        }
    }

    println!("게임이 종료되었습니다!");
}
