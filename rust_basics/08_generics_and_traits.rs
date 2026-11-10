// 第 8 课：泛型与 trait。
// 运行命令：cargo run --example 08_generics_and_traits
// 前置知识：函数、所有权与借用、struct/impl，以及 Option<T>、Result<T, E> 的写法。
//
// 预期输出（不含 Cargo 自己的编译和运行提示）：
// 泛型函数：8 / Rust
// 整数配对：3 / 5
// 文本配对：泛型 / trait
// 约束保证比较：true / false
// 共同接口：学习：泛型
// 共同接口：笔记：trait
// 调用后仍可读取：泛型 / trait
//
// 学习目标：用类型参数复用数据结构与函数，用 trait 约束说明所需的行为。
// 建议先读泛型函数与 Pair，再读 Summary；本课不展开动态分派或高级类型写法。

// 一、<T> 声明一个类型参数，T 是约定俗成的名字，也可以使用其他清晰的名称。
// 参数与返回值使用同一个 T，表示“一次调用中的输入和输出具有相同类型”。
// 这个函数只转交值，不需要知道 T 是整数、String 还是其他具体类型。
fn identity<T>(value: T) -> T {
    // 按值接收并返回：传入 String 时会移动所有权，再把所有权交给调用者。
    // 泛型不会自动克隆数据；是否复制仍由第 4 课讲过的 Copy 等规则决定。
    value
}

// 二、泛型结构体：两个字段共享一个 T，因此每个 Pair 实例的两个元素类型相同。
// Pair<i32> 与 Pair<String> 是不同的具体类型，但复用同一份结构定义。
struct Pair<T> {
    first: T,
    second: T,
}

// impl<T> 先声明这段实现所使用的类型参数，再把方法实现给 Pair<T>。
// 这里没有额外的 trait 约束，所以这些方法可以用于不同元素类型的 Pair。
impl<T> Pair<T> {
    fn new(first: T, second: T) -> Self {
        Self { first, second }
    }

    // 返回 &T 借用字段，避免为了读取 String 等数据就复制它或移走它。
    // 返回的引用借自 self，不能比拥有这个 Pair 的值活得更久。
    fn first(&self) -> &T {
        &self.first
    }

    fn second(&self) -> &T {
        &self.second
    }
}

// 三、trait 定义一组行为约定，让不同类型都能提供调用者需要的方法。
// 目前可以将 Summary 理解成“能提供标题和摘要”的共同接口。
trait Summary {
    // 只有签名、以分号结束的是必需方法，具体类型的实现必须提供方法体。
    // &self 借用接收者；返回 &str 借用标题内容，不为读取标题创建独立副本。
    fn title(&self) -> &str;

    // 有方法体的是默认方法；实现者可以直接使用，也可以提供自己的版本。
    // 默认方法可以调用同一 trait 中的其他方法，包括上面的必需方法。
    fn summary(&self) -> String {
        // format! 与 println! 都支持占位符；format! 创建 String 作为结果，不输出。
        format!("学习：{}", self.title())
    }
}

struct Lesson {
    title: String,
}

// impl Summary for Lesson 表示 Lesson 承诺满足 Summary 的方法约定。
impl Summary for Lesson {
    fn title(&self) -> &str {
        self.title.as_str()
    }
    // 不写 summary，调用时就使用 trait 给出的默认版本。
}

struct Note {
    title: String,
}

impl Summary for Note {
    fn title(&self) -> &str {
        self.title.as_str()
    }

    // 为 Note 提供专用摘要；它仍然满足相同的 Summary 接口。
    fn summary(&self) -> String {
        format!("笔记：{}", self.title())
    }
}

// 四、where T: Summary 是 trait 约束：只接收实现 Summary 的具体类型。
// 同样可以写成 fn print_summary<T: Summary>(item: &T)，含义相同。
// where 把约束放到单独位置，约束较多时更容易阅读。
fn print_summary<T>(item: &T)
where
    T: Summary,
{
    // 编译器依据 Summary 约束，确认任何允许传入的 T 都具有 summary 方法。
    // 函数体只能使用约束保证的操作，不能凭空假定 T 支持加法、比较或某个字段。
    println!("共同接口：{}", item.summary());
}

// 标准库的 PartialEq 描述相等比较能力；约束保证函数体可以使用 ==。
// 接收 &T 只借用两个输入，不要求 T 支持 Copy，也不创建副本。
fn same<T: PartialEq>(left: &T, right: &T) -> bool {
    left == right
}

fn main() {
    // 五、类型推断：由实参决定本次调用的 T；这两次分别是 i32 与 String。
    // 也可以写 identity::<i32>(8) 显式指定类型，和第 7 课 parse::<u32>() 类似。
    let number = identity(8);
    let text = identity(String::from("Rust"));
    println!("泛型函数：{number} / {text}");

    let numbers = Pair::new(3, 5); // 推断为 Pair<i32>，无需每次都写类型注解。
    println!("整数配对：{} / {}", numbers.first(), numbers.second());

    let words: Pair<String> = Pair::new(String::from("泛型"), String::from("trait"));
    println!("文本配对：{} / {}", words.first(), words.second());
    // Pair 的读取方法只需要借用，所以无需为了这些方法添加 T: Copy 或 T: Clone。
    // 需要独立副本时才考虑 Clone 约束；需要按复制规则取值时再考虑 Copy 约束。
    println!(
        "约束保证比较：{} / {}",
        same(&3, &3),
        same(words.first(), words.second())
    ); // 两次调用的 T 分别是 i32 与 String；类型不同仍复用同一比较逻辑。

    let lesson = Lesson {
        title: String::from("泛型"),
    };
    let note = Note {
        title: String::from("trait"),
    };
    print_summary(&lesson); // 使用默认摘要方法。
    print_summary(&note); // 使用 Note 自己的摘要方法。
    // 类型参数在编译期确定；这两次调用分别使用满足约束的 Lesson 和 Note。
    // &T 只是借用，所以调用后仍可以使用原来的所有者。
    println!("调用后仍可读取：{} / {}", lesson.title, note.title);

    // 常见编译错误 1：同一个 T 不能在一次构造中同时代表整数与 bool。
    // 观察方法：只取消下一行开头的 //，运行本课命令，观察后恢复注释。
    // let mixed = Pair::new(1, true);
    // 修复：两个元素使用相同类型；需要混合类型时可使用已学过的元组 (1, true)。

    // 常见编译错误 2：传入的类型没有实现函数要求的 trait。
    // 观察方法：只取消下一行开头的 //，运行本课命令。
    // print_summary(&42);
    // 修复：传入已实现 Summary 的 Lesson 或 Note，或为自己的类型实现此 trait。
    // 单纯具有同名字段或方法也不等于已实现 Summary，必须有对应的 impl。
}

// 常见编译错误 3：impl 声明了 trait 实现，却遗漏必需方法。
// 观察方法：同时取消下面两行代码的 //；即使不创建实例，也会检查实现是否完整。
// struct Incomplete {}
// impl Summary for Incomplete {}
// 修复：在 impl 中补上 fn title(&self) -> &str { "待补标题" }。
// summary 已有默认方法体，可以不重复实现。

// 常见编译错误 4：无约束的类型参数没有保证支持相等比较。
// 观察方法：同时取消下面三行代码的 //；无需调用该函数，也会检查它的函数体。
// fn same_without_bound<T>(left: T, right: T) -> bool {
//     left == right
// }
// 修复：将 <T> 改为 <T: PartialEq>，可对照上面可以编译运行的 same 函数。
// 即使你计划只传整数，也必须在泛型定义处写明函数体需要的能力。
// 每次只启用一个反例，观察后恢复注释，再执行最终检查。

// 练习（先预测，再修改、运行核对）：
// 1. 用 identity 传入 char，并创建一个 Pair<char>；解释哪些地方靠类型推断确定 T。
// 2. 删除 Note 中完整的 summary 方法，保留 title，预测它会改用什么默认输出。
// 3. 为 Lesson 覆写 summary，加入“课程”前缀，确认 print_summary 函数本身无需修改。
