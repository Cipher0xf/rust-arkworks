// Cargo 默认把 src/main.rs 当作主程序，cargo run 会从这里的 main 函数开始执行。
// 此处只展示学习导航。每一课都有独立的 main，放在 rust_basics/ 中，方便逐课阅读、修改。
// println! 后面的 ! 表示它是宏；目前只需把它当作“输出一行文字”的工具。
fn main() {
    println!("Rust 基础学习（rust_basics/）：当前已提供前八个主题，建议每次只学习一课。");
    println!("1. 变量与类型：cargo run --example 01_variables_and_types");
    println!("2. 控制流：cargo run --example 02_control_flow");
    println!("3. 函数：cargo run --example 03_functions");
    println!("4. 所有权与借用：cargo run --example 04_ownership_and_borrowing");
    println!("5. 字符串与集合：cargo run --example 05_strings_and_collections");
    println!("6. struct/enum/match：cargo run --example 06_struct_enum_match");
    println!("7. Option/Result：cargo run --example 07_option_and_result");
    println!("8. 泛型与 trait：cargo run --example 08_generics_and_traits");
    println!("请按顺序阅读 rust_basics/ 中的中文注释，并对照预期输出完成练习。");
}
