/*
@Date		:2023/12/14 21:33:26
@Author		:zono
@Description:
3.常见编程概念
3.1 变量和可变性
let 和 mut
3.2 数据类型
标量类型（scalar type）和复合类型（compound type）
有点像javascript的基本类型和复合类型

标量类型：
1.（有无符号u、i，代表有无正负）整型【6*2种】
2.浮点型【*2】用IEEE-754标准表示，所以0.1+0.2 != 0.3，和js一样
2.5 数值运算
3.布尔类型【*2】
4.字符类型char，4个字节，用单引号表示

javascript：number、string、boolean、null、undefined、symbol（es6）

复合类型：
1.元组类型tuple，固定长度，不可变，用小括号表示，可以包含不同类型的值，可以通过解构或者索引访问，单元
2.数组类型array，固定长度（与其他语言不同，而vector可能才更像数组），可变，用中括号表示，只能包含相同类型的值，可以通过索引访问，单元
js：array、object

最后靠array遍历吹了一波rust的错误处理。
*/
// use std::io;

fn main() {
    println!("Hello, world!");
}
