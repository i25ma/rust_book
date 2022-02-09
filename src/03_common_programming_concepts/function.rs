
fn main() {
    println!("hello, functions");

    // 1. 使用fn 开头定义，后面跟着函数名和 一对圆括号， 括号内定义函数的参数
    // 2. 函数规范风格： 函数和变量名使用snake case 风格
    // 3. 参数： 在函数签名中必须申明每个参数的 类型
    // 4. 函数体： 分为语句和表达式
    //   4.1. 语句：执行一些操作但不返回值的指令
        let mut x = 5 + 1;

    //   4.2. 表达式：计算并产生一个值
        let y = {
            let y:u32 = 4;
            y +1 
        };

        println!("y= {}", y);
}