fn main() {
    // Characters (`char`)

    // 注意单引号，与字符串的双引号不同
    let my_first_initial = 'C';
    if my_first_initial.is_alphabetic() {
        println!("Alphabetical!");
    } else if my_first_initial.is_numeric() {
        println!("Numerical!");
    } else {
        println!("Neither alphabetic nor numeric!");
    }

    // 示例1：字母（输出 Alphabetical!）
    // let your_character = '中'; 
    // 示例2：数字（输出 Numerical!）
    // let your_character = '9';
    // 示例3：特殊符号（输出 Neither...）
    // let your_character = '@';
    // 示例4：emoji（输出 Neither...）
    let your_character = '😜'; 
    
    if your_character.is_alphabetic() {
        println!("Alphabetical!");
    } else if your_character.is_numeric() {
        println!("Numerical!");
    } else {
        println!("Neither alphabetic nor numeric!");
    }
}