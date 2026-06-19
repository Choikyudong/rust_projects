fn main() {
    let my_number: i32 = 5;
    let your_number: u32 = 10;
    let result = my_number + your_number as i32;

    if result == 15 {
        test1();
    } else {
        test2();
    }

    let arr: [i8; 5] = [5, 4, 3, 2, 1];
    for (i, num) in arr.iter().enumerate() {
        println!("[{}]: [{}]", i, num);
    }

    let test_num: u8 = 0;
    let result: bool = match test_num {
        1 => {
            println!("Hello");
            true
        }
        0 => {
            println!("Get out");
            false
        }
        _ => test3()
    };

    if result {
        println!("World")
    }

    if !result {
        print!("T T");
    }

    let mut count: u8 = 0;
    while count < 100 {
        count += 1;
    }

    let mut str: String = String::from("Hi");
    {
        let count: u16 = 999;
        println!("count: {}", count);
        let length: u32 = test4(&str);
        str.push_str(&length.to_string());
        println!("{}", str);
    }
    println!("{}", str);
}

fn test1() {
    println!("Good")
}

fn test2() {
    println!("Bad")
}

fn test3() -> bool {
    panic!("!!!!!")
}

fn test4(str: &String) -> u32 {
    str.len() as u32
}