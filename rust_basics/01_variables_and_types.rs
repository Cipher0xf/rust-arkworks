//! 第 1 课：变量与类型。
//!
//! 运行：cargo run --example 01_variables_and_types
//! 前置知识：本课是起点；先将 main 理解为程序入口，println! 理解为输出工具。
//! 学习目标：区分赋值与遮蔽，识别基础类型，并观察绑定的作用域。
//! 预期标准输出（Cargo 的编译提示不属于程序输出）：
//! ```text
//! 不可变绑定：score = 80
//! 可变绑定：attempts = 2
//! 遮蔽：外层 value = 5
//! 遮蔽：内层 value = 50
//! 离开内层：value = 5
//! 常量：MAX_POINTS = 100
//! 标量：signed = -12, byte = 255, ratio = 2.5, passed = true, symbol = 学
//! 改变类型的遮蔽：marker = true
//! 算术：sum = 12, integer_quotient = 3, remainder = 1, float_quotient = 3.5
//! 元组：count = 3, price = 4.5, mark = 学, first = 3
//! 数组：first = 2, len = 3, repeated = [7, 7, 7]
//! ```

// const 定义常量，必须写出类型，且初始值必须能在编译时求出。
// 常量名约定使用大写字母和下划线，适合表达固定的规则或上限。
// u32 是 32 位无符号整数；这里的 100 不是运行过程中可修改的状态。
const MAX_POINTS: u32 = 100;

fn main() {
    // let 把值绑定到名字；默认不可变，使意外修改在编译阶段就被发现。
    // 未写类型时编译器根据用法推断，这里的整数最终是 i32。
    let score = 80;
    // println! 是宏，末尾的 ! 用来区分宏调用和普通函数调用。
    // {score} 把该变量的值插入输出，println! 会自动添加换行。
    println!("不可变绑定：score = {score}");

    // mut 只在确实需要更新状态时使用；它允许修改值，不允许改变类型。
    let mut attempts: u32 = 1;
    attempts += 1; // 等价于 attempts = attempts + 1，结果仍是 u32。
    println!("可变绑定：attempts = {attempts}");

    // 再次写 let 会创建同名的新绑定，叫作“遮蔽”（shadowing）。
    // 右边先读取旧 value 的 4，再把计算结果 5 交给新的 value。
    // 这与给可变变量赋值不同：新的绑定甚至可以使用不同类型。
    let value = 4;
    let value = value + 1;
    println!("遮蔽：外层 value = {value}");
    {
        // 大括号形成内层作用域，同名绑定只在这对括号内遮蔽外层。
        let value = value * 10;
        println!("遮蔽：内层 value = {value}");
    }
    // 离开内层作用域后，外层的 5 重新可见；它没有被改为 50。
    println!("离开内层：value = {value}");
    println!("常量：MAX_POINTS = {MAX_POINTS}");

    // 标量类型表示单个值。i32 是有符号整数，可以表示负数。
    // u8 是 8 位无符号整数，范围为 0..=255；类型选择影响可表示范围。
    let signed: i32 = -12;
    let byte: u8 = 255;
    // f64 是双精度浮点数，也是没有额外约束时浮点字面量的默认类型。
    // 浮点数通常只近似表示实数；此处 2.5 恰好能用二进制精确表示。
    let ratio: f64 = 2.5;
    let passed: bool = true; // 布尔值只有 true 和 false。
    // char 使用单引号，表示一个 Unicode 标量值，不局限于 ASCII 字符。
    // 一个用户眼中可见的字符也可能由多个标量组成，不能一概等同。
    let symbol: char = '学';
    println!(
        "标量：signed = {signed}, byte = {byte}, ratio = {ratio}, passed = {passed}, symbol = {symbol}"
    );

    // 同名 let 还可以改变类型：旧 marker 是 i32，新的 marker 是 bool。
    // == 是相等比较，产生 bool；右边仍先读取旧绑定，再创建新的同名绑定。
    // 这里无需 mut，因为创建新绑定与修改原来变量的值是两回事。
    let marker: i32 = 1;
    let marker: bool = marker == 1;
    println!("改变类型的遮蔽：marker = {marker}");

    let sum = 7 + 5;
    // 整数除法仍得到整数：向零截断，因此 7 / 2 的结果是 3。
    let integer_quotient = 7 / 2;
    let remainder = 7 % 2; // % 求余数，此处结果为 1。
    // 写成浮点字面量后，两个操作数都是 f64，除法得到 3.5。
    // Rust 不会自动把任意整数变量隐式转换成浮点数。
    let float_quotient = 7.0 / 2.0;
    println!(
        "算术：sum = {sum}, integer_quotient = {integer_quotient}, remainder = {remainder}, float_quotient = {float_quotient}"
    );

    // 元组用圆括号把固定数量、允许类型不同的值组合起来。
    // 这里类型明确为 (i32, f64, char)，元素位置属于类型结构的一部分。
    let item: (i32, f64, char) = (3, 4.5, '学');
    let (count, price, mark) = item; // 解构：按位置给每个元素起名字。
    let first = item.0; // 也可以用 .0、.1、.2 按位置取值，从 0 开始编号。
    println!("元组：count = {count}, price = {price}, mark = {mark}, first = {first}");

    // 数组用方括号组合相同类型的元素，长度固定且也是类型的一部分。
    // [i32; 3] 表示“3 个 i32”，分号前是类型，后是长度。
    let numbers: [i32; 3] = [2, 4, 6];
    let repeated = [7; 3]; // 值形式 [值; 次数] 在这里生成 [7, 7, 7]。
    let first = numbers[0]; // 索引从 0 开始；此处用遮蔽复用了 first 名字。
    let len = numbers.len(); // .len() 返回数组元素个数，类型是 usize。
    // usize 是用于集合长度和索引的无符号整数，其位宽由目标平台决定。
    // 数组用 {:?} 输出调试表示；关于相关 trait，将在后续主题学习。
    println!("数组：first = {first}, len = {len}, repeated = {repeated:?}");

    // 常见编译错误：一次只取消下面某一行开头的 //，再运行本课命令。
    // 观察报错后恢复注释，以便继续运行其他示例。
    // score = 90;
    // 原因：score 不可变。修复：最初写 let mut score，或用 let score = 90 遮蔽。
    // attempts = 2.5;
    // 原因：attempts 是 u32，不能赋 f64。修复：赋整数，或另建浮点绑定。
    // let too_large: u8 = 256;
    // 原因：字面量超出 u8 范围。修复：使用 u16 等能容纳此值的类型。
    // let mixed = [1, true];
    // 原因：同一数组的元素类型必须相同。修复：使用 (1, true) 元组。

    // 运行时错误提醒：访问越界索引会触发 panic；本示例没有恢复 panic 的处理，
    // 若执行到越界访问，当前运行会提前结束并以失败状态退出。
    // 例如本数组仅允许索引 0、1、2；固定越界表达式也可能在编译时被发现。
    // 本课先记住范围，后续会学习用更安全的方式处理可能不存在的元素。

    // 练习（每次只改一处，先预测输出，再运行验证）：
    // 1. 把内层 value 改成 value + 10，解释为何外层输出仍为 5。
    // 2. 把整数除法改为 -7 / 2，观察向零截断与向下取整的区别。
    // 3. 把数组改成 [2, 4, 6, 8]，同步修改类型中的长度并输出最后一个元素。
}
