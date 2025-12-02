#[derive(Debug)]
enum Message {
    // 1. 命名字段结构体变体：存储 x 和 y 坐标（命名字段）
    Move { x: i32, y: i32 },
    // 2. 元组变体：存储一个 String 类型的回声消息
    Echo(String),
    // 3. 元组变体：存储 RGB 颜色值（三个 i32 类型的数值）
    ChangeColor(i32, i32, i32),
    // 4. 单元变体：无关联数据
    Quit,
}

impl Message {
    fn call(&self) {
        println!("{:?}", self);
    }
}

fn main() {
    let messages = [
        Message::Move { x: 10, y: 30 },
        Message::Echo(String::from("hello world")),
        Message::ChangeColor(200, 255, 255),
        Message::Quit,
    ];

    for message in &messages {
        message.call();
    }
}