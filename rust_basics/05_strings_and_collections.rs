// 第 5 课：字符串与集合。
// 运行命令：cargo run --example 05_strings_and_collections
// 前置知识：先完成第 4 课，理解拥有数据与借用数据的区别。
// 学习目标：选择拥有文本或借用文本，理解 UTF-8 长度，并用集合保存和查询数据。
//
// 预期输出（不含 Cargo 自己的编译和运行提示）：
// 借用视图：Rust
// 可增长字符串：Rust 基础!
// UTF-8：字节数 = 7, Unicode 标量数 = 3
// 合法切片：你好
// Vec：values = [10, 25, 30, 40], len = 4
// Vec 查询：索引 1 = Some(25), 索引 9 = None
// Vec 遍历求和：105
// HashMap 查询：Rust = Some(95), Go = Some(88)
// HashMap 状态：键数量 = 2, 包含 Python = false
//
// 本课先学习最常用的操作，不追求一次掌握所有集合 API。
// Some(...) 表示“有这个值”，None 表示“没有这个值”；这里只观察打印结果，
// 它们所属的 Option 类型及处理方式会在第 7 课详细学习。

// String、Vec 等常用类型可以直接使用；HashMap 需要从标准库中引入名字。
// use 只是让后面可以写短名称 HashMap，并没有安装或添加第三方依赖。
use std::collections::HashMap;

fn main() {
    // 一、String 与 &str：一个拥有可增长文本，一个借用一段文本。
    // 字符串字面量 "Rust" 的类型是 &str；这里借用的是程序中的固定文本。
    // &str 是字符串切片引用，描述一段有效的 UTF-8 文本，不负责拥有这段数据。
    let literal: &str = "Rust";
    // String::from 把文本复制到 String 拥有的存储中，后续可以修改和增长。
    // mut 允许修改这个 String；它与“借用是否可变”是不同层面的限制。
    let mut title: String = String::from(literal);
    // as_str() 借用 String 的内容，不会复制文本，也不会转移 title 的所有权。
    // 接收只读文本的函数通常优先使用 &str，使调用者也能传入字面量或切片。
    let view: &str = title.as_str();
    println!("借用视图：{view}");

    // view 的最后一次使用已结束，所以这里可以继续可变借用 title。
    // push_str 追加一段 &str，push 追加一个 char；双引号和单引号不能混用。
    title.push_str(" 基础");
    title.push('!');
    println!("可增长字符串：{title}");

    // 二、UTF-8：字节长度、Unicode 标量数量与人眼看到的字符数量不一定相同。
    // “你”和“好”各占 3 个 UTF-8 字节，ASCII 字符 a 占 1 个字节，总共 7 字节。
    let unicode: &str = "你好a";
    let byte_count: usize = unicode.len(); // len() 对 str/String 返回字节数。
    // chars() 按 Unicode 标量值依次访问，count() 统计个数；这里只认识这一用法。
    // 迭代器机制留到第 9 课；一个人眼中的字符可能由多个 Unicode 标量组成。
    let scalar_count: usize = unicode.chars().count();
    println!("UTF-8：字节数 = {byte_count}, Unicode 标量数 = {scalar_count}");

    // 切片范围按“字节偏移”指定：包含 0、不包含 6，且两端必须位于字符边界。
    // 范围还必须满足“起点 <= 终点 <= 字节长度”；超出范围也会 panic。
    // 因为已知这个固定文本的编码，0..6 恰好选中“你好”，结果仍是借用的 &str。
    // 不能把这里的 6 当成任意中文文本通用的截取规则。
    let greeting: &str = &unicode[0..6];
    println!("合法切片：{greeting}");

    // 三、Vec：长度可变、元素类型相同的连续集合，适合保存一串同类数据。
    // Vec<i32> 中的 <i32> 表示元素类型；泛型的定义和原理留到第 8 课。
    // vec! 是构造 Vec 的宏，方括号里的值会成为初始元素。
    // 也可以用 Vec::new() 创建空集合，再由类型注解或后续 push 确定元素类型。
    let mut values: Vec<i32> = vec![10, 20, 30];
    values.push(40); // 追加一个元素，长度从 3 变为 4。
    values[1] = 25; // 索引从 0 开始：这里修改第二个元素，而非第一个。
    println!("Vec：values = {values:?}, len = {}", values.len());
    // {:?} 输出调试表示，所以集合显示为 [10, 25, 30, 40]。

    // values[index] 在越界时会 panic；索引可能不存在时，get 更适合查询。
    // get 返回 Option<&i32>：存在时借用元素，不存在时返回 None，不因越界而 panic。
    // 这里不取出内部值，只用调试格式观察 Some(25) 与 None。
    println!(
        "Vec 查询：索引 1 = {:?}, 索引 9 = {:?}",
        values.get(1),
        values.get(9)
    );

    // 借用整个 Vec 来遍历，不会把 values 的所有权交给循环。
    // 每轮的 value 是 &i32；*value 读取对应整数，累加到 total。
    let mut total = 0;
    for value in &values {
        total += *value;
    }
    println!("Vec 遍历求和：{total}");

    // 四、HashMap：通过唯一的“键”查找对应的“值”，适合保存名称到数据的映射。
    // 这里键的类型是 &str，值的类型是 i32；键使用固定字面量，借用的数据一直有效。
    // HashMap 也可以拥有 String 键；本课使用字面量，让注意力集中在基本操作上。
    let mut scores: HashMap<&str, i32> = HashMap::new();
    scores.insert("Rust", 90);
    scores.insert("Go", 88);
    scores.insert("Rust", 95); // 同一个键再次插入会替换旧值，不会新增一个相同的键。
    // insert 返回 Option：新键得到 None，替换旧键得到 Some(旧值)；本课暂不使用它。

    // get 同样返回可选的借用；固定按键查询，使输出顺序明确。
    // HashMap 不承诺遍历顺序，因此不要依赖直接打印整个映射得到固定次序。
    println!(
        "HashMap 查询：Rust = {:?}, Go = {:?}",
        scores.get("Rust"),
        scores.get("Go")
    );
    println!(
        "HashMap 状态：键数量 = {}, 包含 Python = {}",
        scores.len(),
        scores.contains_key("Python")
    );

    // 常见编译错误 1：&str 不是拥有可增长存储的 String，没有 push_str 方法。
    // 观察方法：仅取消下面两行各自开头的 //，运行本课命令，观察后恢复注释。
    // let mut borrowed = "Rust";
    // borrowed.push_str("!");
    // 即使绑定加了 mut，也不会把 &str 变成 String。
    // 修复：改用 let mut borrowed = String::from("Rust"); 再追加文本。

    // 常见编译错误 2：字符串不支持用整数直接索引一个“字符”。
    // 观察方法：仅取消下一行的 //，运行本课命令。
    // let first = title[0];
    // UTF-8 字符宽度不固定，整数索引无法统一表达按字节还是按字符取值。
    // 修复：若要遍历 Unicode 标量值，使用 for character in title.chars() { ... }；
    // 若已知合法字节边界并需要一段文本，可以使用本课前面的范围切片。

    // 常见编译错误 3：Vec 的元素类型必须一致，不能混放整数与布尔值。
    // 观察方法：仅取消下一行的 //，运行本课命令。
    // let mixed = vec![1, true];
    // 修复：让所有元素都是整数；若是固定位置的不同类型数据，可改用 (1, true) 元组。

    // 常见运行时错误 1：切片端点落在一个 UTF-8 字符的中间。
    // 观察方法：仅取消下一行的 //，运行本课命令；它能编译，但运行到这里会 panic。
    // println!("错误切片：{}", &unicode[0..1]);
    // 字节 1 位于“你”的编码内部。修复：本例改用 0..3 选中“你”，或 0..6 选中“你好”。
    // 这属于运行时检查，区别于上面 title[0] 被编译器直接拒绝的情况。

    // 常见运行时错误 2：Vec 索引越界；get 会返回 None，而 [] 索引会 panic。
    // 观察方法：同时取消下面两行代码的 //，编译成功后运行到这里会 panic。
    // let missing_index = values.len();
    // println!("错误索引：{}", values[missing_index]);
    // 长度为 4 时合法索引是 0～3；修复：用 get 查询，再按第 7 课处理 Some/None。
    // 每次只启用一个反例，观察后恢复注释，再运行最终编译检查。
}

// 练习（先预测，再修改运行核对）：
// 1. 在追加 ! 之前再执行 title.push('！')，解释两个叹号的类型与最终输出。
// 2. 在 values.push(40) 后再执行 values.push(50)，预测长度、求和与 values.get(4) 的结果。
// 3. 插入 scores.insert("Python", 80)，预测键数量与 contains_key("Python") 的变化。
