/*
@Date		:2023/12/14 21:33:26
@Author		:zono
@Description:
3.常见编程概念
3.1 变量和可变性(12-13)
let 和 mut
3.2 数据类型(12-14)
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

/*3.3函数
使用snake case命名规范，函数参数必须声明类型，
函数体最后一行不用分号，返回值类型在->后面声明，
函数体中最后一个表达式的值作为返回值
3.4.1·语句和表达式
语句：执行一些操作但不返回值的指令
表达式：计算并产生一个值

 */
// fn plus_one(x: i32) -> i32 {
//     x + 1
//     // x+1; //报错，因为表达式才返回值，语句不返回值
// }

// fn main() {
//     let a: i32 = plus_one(5);
//     println!("Hello, world!{a}");
// }
/*3.5注释（没啥好说的） */
/*3.6控制流
3.6.1 if表达式
if表达式的条件必须是bool类型，不像js可以是任意类型
let中的if表达式，写法类似三元运算符
match表达式，类似switch，当初我就觉得switch应该更强大
loop表达式，类似while(true)
*/
// fn main() {
//     let number = 3;
//     if number < 5 {
//         println!("condition was true");
//     } else {
//         println!("condition was false");
//     }
// }
/*3.6.2循环
loop表达式，类似while(true)，可以用break跳出循环（可以加入标签），也可以用continue跳过本次循环

> 标签：'label，类似js的label，可以用来指定break或continue跳出的循环
> 比较：js的label可以指定任意语句，rust的label只能指定循环

while表达式，类似while
for表达式，类似for
*/
fn main() {
    let a = [10, 20, 30, 40, 50];
    let mut index = 0;

    // while index < 5 {
    //     println!("the value is: {}", a[index]);

    //     index += 1;
    // }
    for element in a {
        println!("the value is: {}", element);
    }
    for number in (1..4).rev() {
        println!("{number}!");
    }
    println!("LIFTOFF!!!");
}
