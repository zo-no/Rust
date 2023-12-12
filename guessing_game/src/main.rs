/*
@Date		:2023/12/12 16:07:44
@Author		:zono
@Description:
按照rust官方文档的教程,学习rust语言
2 创建一个猜测游戏
2.1完成一次猜测 (12-12)
*/
use std::io; //引入标准库io模块

// main函数是程序的入口
fn main() {
    println!("猜测一个数"); //输出字符串的宏

    println!("输入你的猜测:");

    let foo = 2; //创建一个不可变的变量
    let mut guess = String::new(); //创建一个可变的字符串变量
                                   /* let:创建一个变量,rust默认变量是不可变的
                                    * mut:创建一个可变的变量
                                    * guess:变量名
                                    * String::new():创建一个空的字符串,
                                    * ::语法表明new是String类型的一个关联函数(associated function)
                                    */

    io::stdin() //调用io模块的stdin函数
        .read_line(&mut guess) //read_line函数读取用户输入的字符串,并将其追加存入guess变量中
        .expect("Failed to read line"); //如果read_line函数返回错误,则调用expect函数,并传入一个字符串作为参数,expect函数会导致程序崩溃,并显示传入的字符串

    println!("You guessed: {guess}", guess = guess); //输出字符串,并将guess变量的值传入字符串中
}
