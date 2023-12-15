/*
@Date		:2023/12/15 09:11:47
@Author		:zono
@Description:
3的课后练习
*/
/*转换华氏温度和摄氏温度 */
use std::io;
fn main() {
    println!("Hello, world!");
    let mut temp: String = String::new();
    let mut unit: String = String::new();
    println!("请输入温度：");
    io::stdin().read_line(&mut temp).expect("读取失败");
    let temp: f32 = temp.trim().parse().expect("请输入数字");
    println!("请输入单位：");
    io::stdin().read_line(&mut unit).expect("读取失败");
    let unit: &str = unit.trim();
    let result: f32 = transform(temp, unit);
    println!("转换结果为：{}", result);
}

fn transform(temp: f32, unit: &str) -> f32 {
    if unit == "f" {
        (temp - 32.0) * 5.0 / 9.0
    } else {
        temp * 9.0 / 5.0 + 32.0
    }
}
