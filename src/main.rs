use std::env;
use std::io::{self, Write};

fn main() {
    // 从命令行参数获取十进制数字
    let args: Vec<String> = env::args().collect();
    let decimal_number: u32;

    if args.len() == 2 {
        // 如果提供了命令行参数
        decimal_number = args[1].parse().expect("Please provide a valid number");
    } else {
        // 没有提供命令行参数，提示用户输入
        print!("请输入一个十进制数字: ");
        io::stdout().flush().unwrap(); // 确保提示语被打印
        let mut input = String::new();
        io::stdin().read_line(&mut input).expect("Failed to read line");
        decimal_number = input.trim().parse().expect("Please provide a valid number");
    }

    // 将十进制数字转换为十六进制表示，并确保是8位
    let hex_string = format!("{:08X}", decimal_number);
    println!("十进制数字 {} 转换为补足的八位十六进制: {}", decimal_number, hex_string);

    // 将十六进制字符串重新排列
    let rearranged_hex = rearrange_hex(&hex_string);
    println!("重新排列后的十六进制: {}", rearranged_hex);

    let uid_str = rearranged_hex;
    match calculate_bcc(&uid_str) {
        Some(bcc) => println!("UID为{}的BCC值为{:X}", uid_str, bcc),
        None => println!("无法计算BCC值，请提供有效的UID字符串"),
    }
}

// 函数：根据给定的重新排列规则
fn rearrange_hex(hex: &str) -> String {
    // 将字符串转化为字符数组
    let chars: Vec<char> = hex.chars().collect();
    
    // 按照特定规则重新排列字符
    let rearranged_chars = vec![
        chars[6], chars[7],
        chars[4], chars[5],
        chars[2], chars[3],
        chars[0], chars[1]
    ];

    // 将重新排列后的字符数组组合成字符串
    rearranged_chars.into_iter().collect()
}

fn calculate_bcc(uid_str: &str) -> Option<u8> {
    // 将十六进制字符串转换为字节数组
    let uid: Vec<u8> = match (0..uid_str.len())
        .step_by(2)
        .map(|i| u8::from_str_radix(&uid_str[i..i+2], 16))
        .collect()
    {
        Ok(bytes) => bytes,
        Err(_) => return None, // 返回None表示转换失败
    };

    if uid.is_empty() {
        return None; // 返回None表示UID长度不足
    }

    // 初始化BCC为UID的第一个字节
    let mut bcc = uid[0];

    // 对剩余字节进行异或运算
    for &byte in &uid[1..] {
        bcc ^= byte;
    }

    // 返回计算得到的BCC值
    Some(bcc)
}