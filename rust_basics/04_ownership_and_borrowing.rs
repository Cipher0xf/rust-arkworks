// 第 4 课：所有权与借用。
// 运行命令：cargo run --example 04_ownership_and_borrowing
// 前置知识：变量、作用域、函数参数与返回值。
// 学习目标：区分移动、复制与克隆，判断一次读取或修改是否只需要借用。
//
// 预期输出（不含 Cargo 自己的编译和运行提示）：
// 复制：original = 10, copied = 10
// 移动后的所有者：Rust
// 独立副本：原值 = Rust!, 副本 = Rust
// 函数取得所有权：交给函数
// 共享借用：Rust! / Rust!
// 借用函数读取：Rust!
// 借用后仍拥有：Rust!
// 可变借用：Rust! 学习中
// 解引用修改：21
// 先读后改：开始
// 借用结束后：开始，继续
// 内层资源：临时文本
// 离开内层作用域
//
// String 是拥有字符串内容的类型；String::from("文字") 创建一个拥有内容的新值。
// String 值负责管理自己的文本存储；移动会把这个值的所有权交给新绑定。
// 本课只使用创建和追加操作，UTF-8、切片与集合留在第 5 课详细介绍。

// 参数写 String，表示函数接收这个值的所有权，而非仅临时查看它。
fn consume_text(text: String) {
    println!("函数取得所有权：{text}");
    // text 在函数结束时离开作用域，其拥有的字符串存储会被自动释放。
    // 调用者不能再使用已经移动进来的原绑定，避免同一资源被重复释放。
}

// &str 是借来的字符串视图；这里不接收字符串的所有权，也不修改内容。
// 传入 &String 时，Rust 可以把它转换成 &str 供读取；第 5 课会展开这种接口。
fn show_text(text: &str) {
    println!("借用函数读取：{text}");
}

// &mut String 表示通过独占的可变借用修改调用者拥有的字符串。
// push_str 追加文字；方法调用会自动处理引用，不必写成 (*text).push_str(...)。
fn append_note(text: &mut String) {
    text.push_str(" 学习中");
}

fn increment(value: &mut i32) {
    // value 是引用；*value 才是它指向的整数位置。
    // 这里修改调用者的值，不是让局部引用改指向其他位置。
    *value += 1;
}

fn main() {
    // 一、i32 的赋值会复制数值，两个绑定都能继续使用。
    // 这由类型是否支持 Copy 决定，并不是“所有赋值都复制”或“可变值才移动”。
    // Copy 是一种 trait，第 8 课再解释；目前记住 i32 属于这种可复制的类型。
    let original: i32 = 10;
    let copied = original;
    println!("复制：original = {original}, copied = {copied}");

    // 二、String 不支持 Copy。赋值会移动所有权，源绑定随后不能再被使用。
    // 移动不会为字符串内容创建独立副本，也不是把原绑定设为空字符串。
    let source = String::from("Rust");
    let mut owner = source;
    println!("移动后的所有者：{owner}");

    // 对本课的 String，clone 显式创建内容独立的副本，需要复制文本并有存储开销。
    // clone 的具体行为由类型决定；不能把所有类型的 clone 都理解成复制整份底层数据。
    // 需要独立数据时才克隆；如果只需要读或临时修改，通常可以借用。
    let cloned = owner.clone();
    owner.push('!'); // push 追加一个 char；修改 owner 不会影响 cloned。
    println!("独立副本：原值 = {owner}, 副本 = {cloned}");

    let message = String::from("交给函数");
    consume_text(message); // 与 String 赋值一样，按值传参会移动所有权。
    // message 已移动。println! 对上面的 owner、cloned 做格式化时则只是借用，
    // 所以打印它们并不会让对应绑定失效。

    // 三、&owner 创建共享引用，引用的类型写作 &String（一般写成 &T）。
    // 通过它可以读取本课的 String，但不能直接修改字符串内容。
    // 可以同时存在多个共享引用；r1、r2 的类型都是 &String。
    let r1 = &owner;
    let r2 = &owner;
    println!("共享借用：{r1} / {r2}");
    show_text(&owner); // &owner 表示借用；调用结束后所有者仍是 owner。
    println!("借用后仍拥有：{owner}");

    // 四、&mut owner 创建独占的可变引用，其类型是 &mut String（一般写成 &mut T）。
    // 这里要从 owner 借出可变引用，所以拥有这个 String 的绑定需要声明为 mut。
    // 对同一数据，可变借用有效期间不能再有其他有效的共享借用或可变借用。
    // r1、r2 后面没有再使用，因此它们的借用不再妨碍这里的修改。
    append_note(&mut owner);
    println!("可变借用：{owner}");

    let mut number = 20;
    increment(&mut number);
    println!("解引用修改：{number}");

    // 五、借用的有效范围通常到最后一次使用为止，不一定延续到右花括号。
    let mut progress = String::from("开始");
    let view = &progress;
    println!("先读后改：{view}"); // view 的最后一次使用。
    progress.push_str("，继续"); // 共享借用已经结束，所以这里可以修改。
    println!("借用结束后：{progress}");

    // 六、拥有资源的局部值离开作用域时，Rust 会自动执行清理。
    {
        let temporary = String::from("临时文本");
        println!("内层资源：{temporary}");
    } // temporary 在此离开作用域，字符串拥有的存储在此释放。
    println!("离开内层作用域");
    // 借用结束不等于资源释放：引用不拥有字符串，清理由所有者负责。
    // main 结束时，owner、cloned、progress 等仍拥有资源的局部值也会被清理。

    // 常见编译错误 1：使用已经移动的绑定。
    // 操作：同时取消下面三行代码的 //，运行本课；观察后恢复注释。
    // let first = String::from("移动");
    // let second = first;
    // println!("{first} / {second}");
    // 修复：只使用 second；确实需要独立副本时，改成 let second = first.clone()。

    // 常见编译错误 2：同一数据存在两个仍然有效的可变借用。
    // 操作：同时取消下面四行代码的 //。
    // let mut text = String::from("独占");
    // let first = &mut text;
    // let second = &mut text;
    // println!("{first} / {second}");
    // 最后一行还使用 first，所以创建 second 时，第一个借用仍然有效。
    // 修复：先使用完 first，再创建并使用 second，不让两次借用重叠。

    // 常见编译错误 3：共享借用仍有效时，又创建可变借用。
    // 操作：同时取消下面四行代码的 //。
    // let mut text = String::from("借用");
    // let shared = &text;
    // let exclusive = &mut text;
    // println!("{shared} / {exclusive}");
    // 修复：先完成 shared 的最后一次使用，再创建 exclusive。
    // 关键是“之后是否还要使用引用”，不能仅靠看引用变量是否还在作用域内判断。

    // 常见编译错误 4：通过可变借用修改一个没有声明为 mut 的绑定。
    // 操作：同时取消下面三行代码的 //。
    // let fixed = String::from("固定");
    // let editable = &mut fixed;
    // editable.push_str("!");
    // 修复：最初写 let mut fixed = ...；给引用起名并不会绕过所有者的可变性要求。

    // 常见编译错误 5：让引用指向已经离开作用域的数据。
    // 操作：同时取消下面六行代码的 //；编译器会阻止悬垂引用。
    // let dangling;
    // {
    //     let temporary = String::from("短命");
    //     dangling = &temporary;
    // }
    // println!("{dangling}");
    // 修复：把字符串所有者放到外层，或把 temporary 的所有权移动到外层变量。
}

// 练习（每次只改一处，先预测结果，再编译或运行确认）：
// 1. 将 copied 声明为 mut，保留原输出，再在其后赋值为 99 并打印两个变量；
//    解释为什么 original 仍为 10，而 copied 变为 99。
// 2. 将 cloned 声明为 mut，在原来的“独立副本”输出后给它追加 '?' 并重新打印两份文本，
//    确认 owner 的内容不会随之变化。
// 3. 在 progress.push_str(...) 后再次打印 view，观察为何原本合法的借用变成错误；
//    再把对 view 的全部读取放到修改前，恢复程序的可编译状态。
