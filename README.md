# Rust 与密码学库学习

这是一个按学习领域组织的 Cargo 项目。当前在 `rust_basics/` 中学习 Rust 基础，已完成前八个主题；每课一个独立示例，只使用标准库，没有第三方依赖。后续学习 arkworks 等密码学库时，再增加对应的主题目录。

建议每次只学一课：先读中文注释，再预测输出、运行验证，最后尝试反例和小练习。暂时不用一次读完所有文件，也不需要提前掌握后续主题。

## 项目结构

```text
rust-arkworks/
├── Cargo.toml                        # 包信息与课程入口；依赖列表为空
├── Cargo.lock                        # Cargo 自动维护的依赖锁定文件
├── README.md                         # 学习路线与运行方法
├── src/
│   └── main.rs                       # 默认入口：打印前八课的运行命令
└── rust_basics/                       # Rust 基础课程
    ├── 01_variables_and_types.rs      # 第 1 课：变量与类型
    ├── 02_control_flow.rs             # 第 2 课：控制流
    ├── 03_functions.rs                # 第 3 课：函数
    ├── 04_ownership_and_borrowing.rs   # 第 4 课：所有权与借用
    ├── 05_strings_and_collections.rs  # 第 5 课：字符串与集合
    ├── 06_struct_enum_match.rs        # 第 6 课：struct/enum/match
    ├── 07_option_and_result.rs        # 第 7 课：Option/Result
    └── 08_generics_and_traits.rs      # 第 8 课：泛型与 trait
```

每个课程文件都有自己的 `main` 函数，可以单独编译、运行。课程文件按学习领域放在 `rust_basics/` 中，由根目录 `Cargo.toml` 的 `[[example]]` 显式指定目标名称和源文件路径，因此仍使用 `cargo run --example <名称>` 运行。无需在默认入口中导入每一课，也无需为每一课创建 Cargo 项目。

例如，第一课对应的配置为：

```toml
[[example]]
name = "01_variables_and_types"
path = "rust_basics/01_variables_and_types.rs"
```

后续按学习领域扩展目录：

- `rust_basics/`：Rust 语法与语言特性，继续添加第 9、10 课。
- `arkworks/`：学习 arkworks 时再创建，存放有限域、曲线等相关示例。
- 其他密码学库：使用各自的主题目录，在学习相应库时添加依赖与示例。

新增课程时，先在对应目录创建 `.rs` 文件，再在根目录 `Cargo.toml` 添加相应的 `[[example]]`。各 `[[example]]` 的 `name` 必须互不重复；不同领域可以使用名称前缀，例如 `arkworks_fields`。

## 学习顺序

| 顺序 | 主题 | 本阶段内容或后续范围 | 状态 |
| --- | --- | --- | --- |
| 1 | [变量与类型](rust_basics/01_variables_and_types.rs) | `let`、`mut`、遮蔽、作用域、常量、标量类型、元组和数组 | 已实现 |
| 2 | [控制流](rust_basics/02_control_flow.rs) | `if/else`、`if` 表达式、`loop`、`while`、`for`、`break`、`continue` | 已实现 |
| 3 | [函数](rust_basics/03_functions.rs) | 参数与类型、返回值、语句与表达式、提前返回、单元类型 `()` | 已实现 |
| 4 | [所有权与借用](rust_basics/04_ownership_and_borrowing.rs) | 移动、复制与克隆、共享与可变借用、解引用、作用域与资源释放 | 已实现 |
| 5 | [字符串与集合](rust_basics/05_strings_and_collections.rs) | `String`、`&str`、UTF-8、`Vec`、`HashMap` | 已实现 |
| 6 | [struct/enum/match](rust_basics/06_struct_enum_match.rs) | 结构体与方法、枚举变体、穷尽匹配、解构与 `if let` | 已实现 |
| 7 | [Option/Result](rust_basics/07_option_and_result.rs) | 缺值与失败、显式匹配、默认值、错误返回与 `?` 传播 | 已实现 |
| 8 | [泛型与 trait](rust_basics/08_generics_and_traits.rs) | 类型参数、泛型结构体与 `impl`、trait 实现、默认方法与约束 | 已实现 |
| 9 | 迭代器与闭包 | 捕获环境、迭代适配器、惰性求值与消费 | 后续逐步添加 |
| 10 | 模块与测试 | 模块组织、可见性、单元测试与集成测试 | 后续逐步添加 |

当前提供第 1～8 课，第 9、10 课后续逐步添加。每个示例都包含正常代码、预期输出、被注释的错误写法及 3 个练习。

第 1～3 课建立语法基础；第 4、5 课学习数据的拥有、借用与组织；第 6～8 课学习自定义类型、缺值与错误处理，以及代码复用。建议按这三个阶段复习，遇到不熟悉的语法再回到对应课程。

完成前三课后，先通过第 4 课分清“谁拥有值、谁在借用值”，再学习第 5 课中的字符串和集合。第 5 课可以分两次阅读：先学字符串与 UTF-8，再学 `Vec` 和 `HashMap`。其中 `Some(...)`、`None` 只用于观察查询结果，完整的 `Option` 处理见第 7 课；类型参数写法与迭代器细节分别在第 8、9 课展开。

第 6 课先学结构体与方法，再学枚举和模式匹配；第 7 课用这些匹配方式处理第 5 课见过的 `Some`、`None`，再学习 `Result` 和 `?`。建议将“没有值”和“操作失败”分开理解，并观察 `?` 在成功与失败路径上的不同执行行为。每课仍可分成两次阅读，不必一次掌握全部写法。

第 8 课先用类型参数复用函数和结构体，再用 trait 描述不同类型共同支持的行为。重点分清“类型可以变化”和“操作必须得到约束保证”，并结合第 4 课判断参数是在移动还是借用。可先读泛型部分，再读 trait 与约束部分。

## 运行方法

在当前项目根目录执行命令。需要已安装支持 Rust 2024 edition 的 Rust 工具链；可先用 `rustc --version` 和 `cargo --version` 确认命令可用。

显示学习导航：

```sh
cargo run
```

按顺序运行每一课：

```sh
cargo run --example 01_variables_and_types
cargo run --example 02_control_flow
cargo run --example 03_functions
cargo run --example 04_ownership_and_borrowing
cargo run --example 05_strings_and_collections
cargo run --example 06_struct_enum_match
cargo run --example 07_option_and_result
cargo run --example 08_generics_and_traits
```

`--example` 后面是文件名去掉 `.rs` 的部分。每条命令只运行对应的一课。若只想看到示例输出，可以加 `--quiet`，例如：

```sh
cargo run --quiet --example 01_variables_and_types
```

每个示例文件开头都有完整的预期输出。普通 `cargo run` 还会显示 Cargo 的编译与启动提示，这些提示不属于示例输出。修改练习代码后，实际输出可能随之改变。

## 每课怎么学

1. 从文件顶部开始，先看知识点、运行命令与预期输出。
2. 顺着 `main` 的执行顺序阅读，关注每个变量的类型、作用域和输出时机。
3. 先预测结果，再运行本课，对照输出解释每一行。
4. 按错误示例的说明，每次只取消一组反例所需代码的注释：编译错误反例可用编译检查观察，panic 反例必须实际运行。
5. 观察错误后恢复注释，再完成文件末尾的少量练习。

反例默认全部被注释，因此正常状态下项目可以编译。不要一次取消所有反例的注释，否则多个诊断混在一起会增加理解难度。带分号与不带分号、可变赋值与变量遮蔽等细节，建议每次只修改一处进行比较。

三种结果要分开观察：

| 结果 | 会发生什么 | 在本项目中如何观察 |
| --- | --- | --- |
| 编译错误 | 类型或借用等规则不满足，程序不能构建 | 启用一组编译反例后执行 `cargo check --example <名称> --offline --locked` |
| 运行时 panic | 可以通过编译，运行到错误位置时该次示例运行失败 | 启用一组 panic 反例后执行 `cargo run --example <名称>` |
| `None` 或 `Err(...)` | 正常返回的枚举值，调用者决定如何继续处理 | 第 7 课用 `match` 分支输出结果，程序继续运行 |

阅读编译错误时，先看错误位置和第一条主要诊断，再对照“期望类型/实际类型”或“借用创建/最后使用”的位置。`cargo check` 不执行程序，因此不能据此断定运行时一定不会出错。

打印使用的 `println!` 是宏，先按“输出一行文字”理解即可；`{name}` 会插入变量值，`{}` 会插入后续参数的值，`{:?}` 用于观察数组、集合或查询结果的调试表示。本阶段暂不展开宏、格式化 trait 或迭代器实现细节。

## 主题 1～8 复习检查

完成练习后，尝试用自己的话解释下列问题；答不清楚时回到对应示例修改一处代码再观察。

| 主题 | 应能解释的问题 |
| --- | --- |
| 1 | `mut` 赋值与重新 `let` 遮蔽有什么区别，哪一种可以改变绑定的类型？ |
| 2 | `break`、`continue` 和循环条件分别会让执行流程走到哪里？ |
| 3 | 为什么在函数或块的末尾多写一个分号，返回值可能变成 `()`？ |
| 4 | 哪些操作转移所有权，哪些只借用；引用最后一次使用为什么影响后续修改？ |
| 5 | UTF-8 字节数、Unicode 标量数与人眼看到的字符数为何可能不同？ |
| 6 | `match` 为什么需要穷尽所有变体，`if let` 又适用于什么场景？ |
| 7 | `None`、`Err` 与 panic 有什么区别，`?` 会从哪个函数提前返回？ |
| 8 | 泛型函数为什么不能随意操作 `T`，trait 约束如何提供所需能力？ |

## 格式化与编译检查

格式化默认入口与全部示例：

```sh
cargo fmt --all
```

检查格式是否符合要求，并编译检查所有目标：

```sh
cargo fmt --all -- --check
cargo check --all-targets --offline --locked
```

`cargo check` 检查类型等编译问题，但不会执行程序；因此还应运行上面的八个示例，对照预期输出。`--all-targets` 会覆盖默认入口和独立示例，`--offline` 禁止访问网络，`--locked` 要求使用现有锁定文件而不更新它。项目没有第三方依赖，无需下载密码学库。

编译错误反例应在编译阶段失败，panic 反例则应在运行阶段失败；它们都不是正常示例的预期结果。观察后恢复注释，再完成格式检查、编译检查和实际运行。正式的模块与测试写法留到第 10 课介绍。
