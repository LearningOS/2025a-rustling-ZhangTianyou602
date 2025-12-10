// options3.rs
struct Point {
    x: i32,
    y: i32,
}

fn main() {
    let y: Option<Point> = Some(Point { x: 100, y: 200 });

    // 关键修改：匹配引用而非所有权，避免 y 被移动
    match &y {
        Some(p) => println!("Co-ordinates are {},{} ", p.x, p.y),
        _ => panic!("no match!"),
    }
    y; // 此时 y 仍保有所有权，可正常使用
}