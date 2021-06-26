use std::io;

fn main() {
    let degree_type: i32 = check_type();
    let degree: f64 = check_value(degree_type);
    if degree_type == 1 {
        println!("轉換後的溫度是：{}度C", degree);
    } else {
        println!("轉換後的溫度是：{}度F", degree);
    }
}

fn check_type() -> i32 {
    println!("選擇輸入溫度的原始單位 1) 華氏 2) 攝氏，請輸入相對應的數字！");
    loop {
        let mut degree_type = String::new();
        io::stdin()
            .read_line(&mut degree_type)
            .expect("讀取該行失敗");
        let degree_type: i32 = match degree_type.trim().parse() {
            Ok(num) => num,
            Err(_) => {
                println!("請輸入數字 1) 華氏 2) 攝氏！");
                continue;
            }
        };
        if degree_type != 1 && degree_type != 2 {
            check_type();
        }
        break degree_type;
    }
}

fn check_value(degree_type: i32) -> f64 {
    println!("請輸入溫度！");
    loop {
        let mut degree_string = String::new();
        io::stdin()
            .read_line(&mut degree_string)
            .expect("讀取該行失敗");
        let mut degree: f64 = match degree_string.trim().parse() {
            Ok(num) => num,
            Err(_) => {
                println!("請輸入數字！");
                continue;
            }
        };
        if degree_type == 1 {
            degree = (degree - 32.0) * 5.0 / 9.0
        } else {
            degree = degree * 9.0 / 5.0 + 32.0
        }
        return degree;
        //degree_string = "轉換後的溫度為：{degree}".to_string();
    }
}
