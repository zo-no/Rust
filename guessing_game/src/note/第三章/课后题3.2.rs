/*
@Date		:2023/12/15 09:17:06
@Author		:zono
@Description:
课后题3.2
*/
/*斐波那契数列 */
fn main() {
    let mut n: String = String::new();
    println!("请输入n：");
    std::io::stdin().read_line(&mut n).expect("读取失败");
    let n: u32 = n.trim().parse().expect("请输入数字");
    let result: u32 = fib(n);
    println!("结果为：{}", result);
}

fn fib(n: u32) -> u32 {
  //递归
    if n == 1 || n == 2 {
        1
    } else {
        fib(n - 1) + fib(n - 2)
    }
}