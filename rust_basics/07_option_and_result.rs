// 第 7 课：Option 与 Result。
// 运行命令：cargo run --example 07_option_and_result
// 前置知识：先完成第 6 课，理解 enum、match、模式绑定与 if let。
// 学习目标：区分缺值与失败，选择显式处理、默认值或向调用者传播错误。
//
// 预期输出（不含 Cargo 自己的编译和运行提示）：
// Option 有值：20
// Option 缺值：索引 9 不存在
// if let：首项 = 10
// Option 默认值：0
// 解析 12：12
// 解析 abc：请输入有效的非负整数
// 解析 101：数量不能超过 100
// ? 计算 7 + 8：15
// ? 传播 7 + abc：请输入有效的非负整数
//
// Option 与 Result 都是标准库定义的 enum，常用类型与变体可直接使用，无需 use。
// Option<T>：Some(T) 表示存在一个 T 类型的值，None 表示缺值。
// Result<T, E>：Ok(T) 表示成功结果，Err(E) 表示失败原因。
// 尖括号里的 T、E 是类型占位符；本课先学使用，第 8 课再学习泛型的定义。
// “没找到”可以是正常情况，适合 Option；调用失败且需要解释原因时，适合 Result。

// 把输入解析成最多 100 的数量。返回类型把成功值和失败原因明确交给调用者。
// &'static str 在这里表示错误消息借用自字符串字面量，它在整个程序运行期间有效。
// 先把它理解为“固定文字形式的错误消息”；生命周期的其他用法不在本课展开。
fn parse_quantity(text: &str) -> Result<u32, &'static str> {
    // parse::<u32>() 指定要把文本转换为 u32；解析本身也会返回 Result。
    // 负数、非数字或超出 u32 范围的文本不能被这个解析方法接受。
    // 本例不自动删除空白；如需接受首尾空格，可以先调用 text.trim()。
    let quantity = match text.parse::<u32>() {
        Ok(value) => value, // 取出成功结果，quantity 的类型因此为 u32。
        Err(_) => return Err("请输入有效的非负整数"),
        // _ 忽略标准库给出的具体错误内容，改用本例统一且确定的错误消息。
        // return Err(...) 会结束当前函数，但不会让整个程序发生 panic。
    };

    if quantity > 100 {
        // 语法上能解析的数字仍可能不符合业务规则，这也是需要报告的错误。
        return Err("数量不能超过 100");
    }

    Ok(quantity) // 返回成功值时也必须包在 Ok 里，不能直接写 quantity。
}

// ? 用于把失败交给调用者处理。这里两个被调用函数与外层使用相同的错误类型。
fn combined_quantity(first: &str, second: &str) -> Result<u32, &'static str> {
    let first_value = parse_quantity(first)?;
    // 上一行相当于：match parse_quantity(first) {
    //     Ok(value) => value,
    //     Err(message) => return Err(message),
    // }
    // 成功时 ? 取出内部值；失败时立即返回 Err，下一行不会再执行。
    // 它不是 panic，也不是仅跳过当前循环；它结束的是当前这次函数调用。
    let second_value = parse_quantity(second)?;

    // 每个数量都已检查为 0..=100，所以这里的和最多为 200，不会超出 u32。
    Ok(first_value + second_value)
}

fn main() {
    // 一、接着第 5 课的 get：返回 Option<&i32>，不会因为索引缺失而 panic。
    let values: Vec<i32> = vec![10, 20];
    match values.get(1) {
        // value 是借用的 &i32，并非把 Vec 中的元素移动出来；格式化可以直接读取它。
        Some(value) => println!("Option 有值：{value}"),
        None => println!("Option 缺值：索引 1 不存在"),
    }
    match values.get(9) {
        Some(value) => println!("Option 有值：{value}"),
        None => println!("Option 缺值：索引 9 不存在"),
    }
    // 两个 match 都处理了 Some 与 None，编译器能检查是否遗漏了可能的情况。
    // None 不等于数字 0：Some(0) 是“有值，值为 0”，None 是“没有值”。

    // 二、只需要在有值时做事，可以用 if let；这里选择在缺值时不输出任何内容。
    if let Some(first) = values.get(0) {
        println!("if let：首项 = {first}");
    }

    // unwrap_or 按值接收 Option：Some 取出内部值，None 得到给定默认值。
    // 默认值表达式会先求值，并不是等到发现 None 才执行；本例的默认值只是数字 0。
    // 这里显式写类型，因为仅凭 None 无法判断“缺少的值”本应是什么类型。
    let absent: Option<i32> = None;
    let fallback = absent.unwrap_or(0);
    println!("Option 默认值：{fallback}");
    // Option<i32> 可以复制；Option<String> 不支持 Copy，调用 unwrap_or 会消耗它。
    // 默认值应符合实际需求；如果“缺值”需要报告，就应该用 match 明确处理。

    // 三、显式处理 Result：成功值与错误消息通过不同分支使用，不会混为一谈。
    // 同时运行合法输入、无效文本和超出业务范围的输入，观察三条执行路径。
    for input in ["12", "abc", "101"] {
        match parse_quantity(input) {
            Ok(quantity) => println!("解析 {input}：{quantity}"),
            Err(message) => println!("解析 {input}：{message}"),
        }
    }

    // 四、错误传播：主函数在边界处 match，决定遇到错误后怎样向用户报告。
    // 这里的元组数组让同一段处理代码分别运行成功案例与失败案例。
    for (first, second) in [("7", "8"), ("7", "abc")] {
        match combined_quantity(first, second) {
            Ok(total) => println!("? 计算 {first} + {second}：{total}"),
            Err(message) => println!("? 传播 {first} + {second}：{message}"),
        }
        // 第二组参数在解析 second 时失败，combined_quantity 的加法不会执行。
        // 错误被正常返回并由这里处理，main 仍能继续执行后面的代码。
    }

    // 常见编译错误 1：Option 是一个枚举值，不等于它内部可能包含的整数。
    // 观察方法：仅取消下一行代码的 //，再运行本课；观察后恢复注释。
    // let wrong: i32 = Some(10);
    // 修复：把目标类型改为 Option<i32>，或用 match 处理 Some/None 并取得需要的值。

    // 常见编译错误 2：在默认返回 () 的 main 中使用 Result 的 ?。
    // 观察方法：仅取消下一行代码的 //，再运行本课。
    // let _quantity = parse_quantity("12")?;
    // ? 必须能把失败返回给外层，当前 main 却没有声明返回 Result。
    // 修复：本课在 main 使用 match，或把传播错误的逻辑放进返回 Result 的函数。
    // 即使这里的输入恰好合法，编译器也会按类型检查可能的错误路径。

    // 常见编译错误 3：缺少足够信息，无法推断 None 对应的内部类型。
    // 观察方法：仅取消下一行代码的 //，再运行本课。
    // let unknown = None;
    // 修复：写出 let unknown: Option<i32> = None，或通过后续用法提供类型约束。

    // 常见编译错误 4：包在 Option 中，并不会让 String 自动变得可以复制。
    // 观察方法：同时取消下面三行代码的 //，再运行本课。
    // let owned = Some(String::from("Rust"));
    // let content = owned.unwrap_or(String::from("默认"));
    // println!("{owned:?} / {content}");
    // unwrap_or 已经消耗 owned；修复：若要保留它，可改用 match &owned 借用查看内容。
    // 只有确实需要独立副本时才考虑 clone，而不是为了绕过移动错误一律复制。

    // 常见运行时错误：对 None 调用 unwrap，会触发 panic，而不是返回一个默认值。
    // 观察方法：同时取消下面两行代码的 //，编译成功后运行到这里会 panic。
    // let unavailable: Option<i32> = None;
    // let _value = unavailable.unwrap();
    // expect("说明") 在缺值时也会 panic，只是可以附带说明；Result 的 Err 也类似。
    // 本例需要正常处理缺值，修复方式是 match、if let 或合适的 unwrap_or 默认值。
    // 每次只启用一个反例，观察后恢复注释，再执行最终编译检查。
}

// 练习（先预测，再修改、运行核对）：
// 1. 把 values.get(9) 改为 values.get(0)，预测会进入哪个分支以及变化后的输出内容。
// 2. 把 absent 改为 Some(5)，保持 unwrap_or(0)，解释为什么最终得到 5。
// 3. 在 main 的参数组合数组中追加 ("abc", "8")；再临时在解析第二个参数前输出
//    一行提示，确认第一次解析失败时为什么不会执行到提示，随后恢复原始输出。
