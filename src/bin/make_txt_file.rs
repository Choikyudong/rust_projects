use std::fs;
use std::fs::File;
use std::{env, io};
use std::io::{Write};
use std::path::{Path, PathBuf};

fn main() -> io::Result<()> {
    println!("파일명을 입력해주세요.");

    let mut input = String::new();
    io::stdin().read_line(& mut input)
        .expect("입력에 실패하였습니다.");

    let mut file_name = String::from(&input).trim().to_string();
    loop {
        println!("{file_name} 로 파일을 생성할까요?");
        if check_yes_or_no() {
            break;
        }

        file_name.clear();
        println!("파일명을 다시 입력해주세요.");
        io::stdin().read_line(&mut file_name)
            .expect("파일 이름을 확인해주세요.");

        file_name = file_name.trim().to_string();
    }

    let mut file_dir: PathBuf = match env::current_dir() {
        Ok(path) => {
            println!("현재 경로: {:?}", path);
            path
        },
        Err(e) => {
            panic!("경로를 가져오는데 실패하였습니다!!!, {:?}", e);
        }
    };

    println!("현재 경로에 생성할까요?");
    loop {
        if check_yes_or_no() {
            if is_writable(&file_dir) {
                break
            }
            println!("다른 경로를 입력해주세요.");
            file_dir = select_write_dir();
        } else {
            println!("경로를 입력해주세요.");
            file_dir = select_write_dir();
            break
        }
    }

    file_dir.push(&file_name);
    let mut file = File::create(&file_dir)
        .expect("파일명을 확인해주세요.");

    println!("파일에 입력할 내용을 입력해주세요.");
    println!("입력을 끝내고 싶을경우 :wq를 입력해주세요.");

    let mut content = String::new();

    loop {
        let mut input = String::new();
        match io::stdin().read_line(&mut input) {
            Ok(_) => {
                if input.trim() == ":wq" {
                    println!("입력을 종료합니다.");
                    break;
                }
                content.push_str(&input);
            }
            Err(e) => {
                print!("오류 발생: {:?}", e);
                break
            }
        }
    }

    file.write_all(content.as_bytes())
        .expect("파일 입력에 실패하였습니다.");

    println!("파일이 생성되었습니다.");
    println!("경로: {}, 파일명: {})", file_dir.display(), file_name);
    Ok(())
}

fn check_yes_or_no() -> bool {
    loop {
        println!("1: 예, 2: 아니요");
        let mut input = String::new();
        io::stdin().read_line(&mut input)
            .expect("입력을 확인해주세요.");

        let input: i8 = match input.trim().parse() {
            Ok(input) => input,
            Err(_) => {
                println!("1과 2로 입력해주세요.");
                continue
            }
        };

        return input == 1
    }
}

fn is_writable(path: &Path) -> bool {
    if let Ok(meta) = fs::metadata(path) {
        if meta.permissions().readonly() {
            println!("생성 권한이 없는 폴더입니다.");
            println!("다른 폴더를 입력해주세요.");
            false;
        }
    }

    let temp_file = path.join(".write_test");
    match File::create(&temp_file) {
        Ok(_) => {
            let _ = fs::remove_file(&temp_file);
            true
        },
        Err(e) if e.kind() == {
            println!("생성 권한이 없는 폴더입니다.");
            println!("다른 폴더를 입력해주세요.");
            io::ErrorKind::PermissionDenied
        } => false,
        Err(_) => false
    }
}

fn select_write_dir() -> PathBuf {
    loop {
        let mut input = String::new();
        io::stdin().read_line(&mut input)
            .expect("입력을 확인해주세요.");

        let target_path = Path::new(input.trim());

        if !target_path.exists() {
            println!("존재하지 않는 경로입니다: {}", target_path.display());
            println!("경로를 다시 입력해주세요.");
            continue
        }

        if !target_path.is_dir() {
            println!("폴더가 아닙니다");
            println!("경로를 다시 입력해주세요.");
            continue
        }

        println!("{} 경로에 생성할까요?", target_path.display());
        if check_yes_or_no() {
            if is_writable(&target_path) {
                return PathBuf::from(target_path);
            }
            continue;
        }
    }
}