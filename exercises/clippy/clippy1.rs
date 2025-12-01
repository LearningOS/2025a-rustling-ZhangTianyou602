// 引入 f32 的常量模块（包含精确的 PI）
use std::f32::consts::PI;

fn main() {
    let pi = PI; // 使用标准库的精确 π 值
    let radius = 5.00f32;

    // 直接调用 f32 类型的 powi 方法（无需全路径）
    let area = pi * radius.powi(2);

    println!(
        "The area of a circle with radius {:.2} is {:.5}!",
        radius, area
    );
}