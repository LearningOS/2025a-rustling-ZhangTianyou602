#[derive(Debug)]
enum Message {
    // 定义枚举的四个变体，均为无数据关联的单元结构体形式
    Quit,
    Echo,
    Move,
    ChangeColor,
}

fn main() {
    println!("{:?}", Message::Quit);
    println!("{:?}", Message::Echo);
    println!("{:?}", Message::Move);
    println!("{:?}", Message::ChangeColor);
}