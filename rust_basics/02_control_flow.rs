// 主题 02：控制流——决定执行哪个分支，以及重复执行多少次。
// 运行命令：cargo run --example 02_control_flow
// 本文件只使用标准库；不需要输入，每次运行的结果相同。
//
// 预期输出（不包含 Cargo 自己显示的编译、运行提示）：
// if 分支：通过
// if 表达式：10
// loop 返回值：6
// while 倒计时：3
// while 倒计时：2
// while 倒计时：1
// for 半开区间求和：6
// continue 后的奇数和：9

fn main() {
    // 一、if / else：条件必须是 bool，不需要在条件外面加圆括号。
    // score 根据整数的默认规则推断为 i32；比较运算产生 bool。
    let score = 82;
    // && 表示“并且”，采用短路求值：左侧为 false 时，不计算右侧。
    // 这里先判断是否及格，再判断分数是否没有超过 100。
    if score >= 60 && score <= 100 {
        println!("if 分支：通过");
    } else if score < 0 || score > 100 {
        // || 表示“或者”，左侧为 true 时，不再计算右侧。
        println!("if 分支：分数无效");
    } else {
        // 从上到下找到第一个条件为 true 的分支后，只执行该分支。
        // score 为 82，所以本次运行不会进入这里。
        println!("if 分支：继续努力");
    }

    // 二、if 还是表达式：它可以产生一个值，交给 let 绑定。
    // 分支最后的表达式没有分号，值才会成为该分支的结果。
    // 两个分支都返回 i32，因此 bonus 的类型也被推断为 i32。
    let bonus = if score >= 90 { 20 } else { 10 };
    // 整个 let 语句仍然以分号结束；分号的位置与上面分支内部不同。
    println!("if 表达式：{bonus}");

    // 三、loop：反复执行代码块，直到显式 break。
    // attempts 会重新赋值，因此声明时需要 mut。
    let mut attempts = 0;
    let result = loop {
        attempts += 1; // 相当于 attempts = attempts + 1。
        if attempts == 3 {
            // break 可以同时结束 loop 并交出一个值。
            // 此处交出的值是 3 * 2，也就是 6，绑定给外面的 result。
            break attempts * 2;
        }
        // 本轮执行到块末尾后，重新从 attempts += 1 开始。
    };
    // loop 在这里作为 let 的值使用，所以右花括号后面需要分号。
    println!("loop 返回值：{result}");

    // 四、while：每一轮开始前先检查条件。
    // 适合“当某个条件成立时继续”的情形；条件一开始为 false 就执行零次。
    let mut remaining = 3;
    while remaining > 0 {
        println!("while 倒计时：{remaining}");
        remaining -= 1;
        // remaining 依次变成 2、1、0；变成 0 后，下次检查会退出。
        // 若删除 remaining -= 1，remaining 就不再变化，会一直打印同一个数。
    }

    // 五、for：依次取出范围中的值，交给本轮的循环变量。
    // 0..4 是半开区间：包含 0，不包含 4，依次产生 0、1、2、3。
    // for 自动推进到下一个值，避免忘记更新计数器。
    let mut range_sum = 0;
    for number in 0..4 {
        // number 每轮获得新的值，循环体内没有给它重新赋值，无需 mut。
        range_sum += number;
    }
    println!("for 半开区间求和：{range_sum}");

    // 六、continue：跳过本轮剩余代码，进入下一轮。
    // 1..=5 是闭区间：两端都包含，依次产生 1、2、3、4、5。
    let mut odd_sum = 0;
    for number in 1..=5 {
        if number % 2 == 0 {
            // % 是取余；偶数除以 2 的余数是 0。
            // continue 只跳过当前数字，后续数字仍然会被处理。
            continue;
        }
        odd_sum += number; // 最终累加 1 + 3 + 5，结果为 9。
    }
    println!("continue 后的奇数和：{odd_sum}");

    // 常见错误 1：Rust 不会把整数自动当作真假条件。
    // 操作：仅取消下面三行代码的 //，再运行本文件的命令。
    // 编译器会提示条件需要 bool，却得到了整数；观察后重新注释即可恢复。
    // if 1 {
    //     println!("整数不能直接用作条件");
    // }
    // 修复：写出明确的比较，例如 if score != 0 { ... }。

    // 常见错误 2：作为值使用的 if，各分支的结果类型必须一致。
    // 操作：仅取消下一行代码的 //，会出现整数与 bool 类型不兼容的错误。
    // let inconsistent = if score >= 60 { 1 } else { false };
    // 修复：两个分支都返回整数，例如把 false 改为 0。
    // 不要在上面的某一分支中随意添加分号：加分号会丢弃该值，
    // 使分支结果变成单元类型 ()；函数主题还会进一步解释这一点。

    // 常见行为误解：break 会退出整个当前循环，continue 只跳过当前一轮。
    // 在奇数求和的循环中把 continue 改为 break，会在遇到 2 时直接退出，
    // 因此只累加了 1；这能编译，但行为与原来的目标不同。
}

// 少量练习（建议每次只改一处，先预测，再运行核对）：
// 1. 把 score 改为 95：哪些输出变化？提示：bonus 应变成 20。
// 2. 把 remaining 的初始值改为 0：while 循环体会执行几次？为什么？
// 3. 把奇数求和的范围改为 1..=10：先手算，再确认结果是否为 25。
