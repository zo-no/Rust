/*
@Date		:2023/12/12 16:07:44
@Author		:zono
@Description:
按照rust官方文档的教程,学习rust语言
2 创建一个猜测游戏
2.1完成一次猜测 (12-12)
2.2生成一个随机数 (12-14)
使用rand库
这里介绍了强大的cargo包管理工具
2.3比较输入的字符串和随机数 (12-14)
讲到了类型转换
和match语法（比较语句？）
2.4允许多次猜测 (12-14)
循环
2.5处理无效输入 (12-14)

感觉：每个语句后面跟一个错误处理，真的太优雅了。
*/
// 2.1完成一次猜测 (12-12)
use rand::Rng; //引入rand库的Rng特征,triat是rust中的接口概念
use std::cmp::Ordering;
use std::io; //引入标准库io模块 //引入标准库cmp模块的Ordering枚举

// main函数是程序的入口
fn main() {
    println!("猜测一个数"); //输出字符串的宏

    let secret_number = rand::thread_rng().gen_range(1..=100); //调用rand库的thread_rng函数,并调用gen_range方法,生成一个1-100的随机数

    println!("the secret number is:{secret_number}");

    loop {
        println!("输入你的猜测:");

        let _foo: i32 = 2; //创建一个不可变的变量
        let mut guess: String = String::new(); //创建一个可变的字符串变量
                                               /* let:创建一个变量,rust默认变量是不可变的
                                                * mut:创建一个可变的变量
                                                * guess:变量名
                                                * String::new():创建一个空的字符串,
                                                * ::语法表明new是String类型的一个关联函数(associated function)
                                                */

        io::stdin() //调用io模块的stdin函数
            .read_line(&mut guess) //read_line函数读取用户输入的字符串,并将其追加存入guess变量中
            .expect("Failed to read line"); //如果read_line函数返回错误,则调用expect函数,并传入一个字符串作为参数,expect函数会导致程序崩溃,并显示传入的字符串

        // let guess: u32 = guess.trim().parse().expect("Please type a number!"); //将guess变量的值转换为u32类型,并将其存入guess变量中
                                                                               //trim方法去除字符串首尾的空白字符
                                                                               //parse方法将字符串转换为数字

        let guess: u32 = match guess.trim().parse() {
            Ok(num) => num,//Ok(num)表示parse方法返回的结果是一个数字
            Err(_) => continue,//Err(_)表示parse方法返回的结果是一个错误
        };

        println!("You guessed: {guess}"); //输出字符串,并将guess变量的值传入字符串中

        // 2.3比较输入的字符串和随机数
        match guess.cmp(&secret_number) {
            //cmp方法比较两个值,返回一个Ordering枚举
            Ordering::Less => println!("Too small!"),
            Ordering::Greater => println!("Too big!"),
            Ordering::Equal => {
                println!("You win!");
                break;
            }
        }
    }
}
