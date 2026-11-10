// 第 6 课：struct、enum 与 match。
// 运行命令：cargo run --example 06_struct_enum_match
// 前置知识：函数、String、所有权以及 &T / &mut T 借用。
//
// 预期输出（不含 Cargo 自己的编译和运行提示）：
// 结构体：struct，计划 15 分钟
// 方法调用：enum 与 match，计划 30 分钟，较长课程 = true
// 状态：尚未开始
// 状态：学习中（30 分钟）
// 状态：已完成，备注：已完成练习
// match 表达式：较长课
// if let 读取备注：已完成练习
//
// 学习目标：用结构体组合相关数据，用枚举表示互斥状态，用模式匹配处理状态。
// 本课围绕一份学习计划展开，不提前引入泛型定义或 trait 实现。

// 一、struct 定义一种具名类型，将“同时存在”的相关字段组合在一起。
// 与元组相比，title、minutes 这样的字段名让数据的含义更容易理解。
// 字段分别有自己的类型；这里让 Lesson 拥有 String，避免依赖外部文本的借用。
struct Lesson {
    title: String,
    minutes: u32,
}

// impl Lesson 把与 Lesson 相关的函数集中在一起，本身不会执行这些函数。
impl Lesson {
    // 在 impl 中定义、不接收 self 的函数是关联函数，使用 Lesson::new(...) 调用。
    // new 是常见的命名约定，并不是 Rust 的特殊关键字。
    // Self 在此表示正在实现的类型 Lesson，可以减少重复书写类型名。
    fn new(title: &str, minutes: u32) -> Self {
        Self {
            title: String::from(title),
            minutes, // 字段名与参数同名时，minutes 是 minutes: minutes 的简写。
        }
    }

    // 带 self 参数的关联函数是“方法”，通常用 实例.方法名(...) 调用。
    // &self 是 self: &Self 的简写：借用实例读取数据，不取得它的所有权。
    // 因为没有移动整个实例，调用后仍可以继续使用 lesson。
    fn is_long(&self) -> bool {
        self.minutes >= 30
    }

    // &mut self 是 self: &mut Self 的简写，要求独占的可变借用。
    // 因此调用者的实例绑定需要声明为 mut，才能通过此方法修改字段。
    fn add_minutes(&mut self, extra: u32) {
        // 本例数值很小；u32 有范围上限，不表示任意大的分钟数。
        self.minutes += extra;
    }
}

// 二、enum 定义“几种情况中的一种”：每个 StudyState 值只处于一个变体。
// 变体既可以不携带数据，也可以携带不同类型、不同形状的数据。
// 将状态与对应的数据绑在一起，每种状态只携带它需要的数据。
// Finished 必须携带一个 String；文本是否允许为空仍需另外设计规则。
enum StudyState {
    NotStarted,                // 无数据变体，构造时写 StudyState::NotStarted。
    Learning { minutes: u32 }, // 具名字段变体，使用花括号指定字段。
    Finished(String),          // 元组形式变体，使用圆括号提供一条备注。
}

// 三、match 根据模式选择分支。借用状态，只读取其中的数据，不消费整个状态值。
fn show_state(state: &StudyState) {
    // match 要穷尽所有可能的情况，新增枚举变体时，编译器能提醒遗漏的处理。
    // “模式 => 要执行的表达式”是一条分支；分支按顺序尝试，执行首个匹配项。
    match state {
        StudyState::NotStarted => println!("状态：尚未开始"),
        StudyState::Learning { minutes } => {
            // Learning { minutes } 按字段解构，给内部的分钟数起一个局部名字。
            // 匹配的是 &StudyState，Rust 会自动按引用绑定字段；minutes 是 &u32。
            println!("状态：学习中（{minutes} 分钟）");
        }
        StudyState::Finished(note) => {
            // note 是 &String，只借用备注，所以不会把 String 从状态中移动出来。
            println!("状态：已完成，备注：{note}");
        }
    }
}

fn main() {
    // 四、结构体字面量：按字段名创建实例，字段的顺序不必与定义一致。
    // 创建实例时必须提供全部字段，字段值也必须满足定义中的类型。
    let first = Lesson {
        title: String::from("struct"),
        minutes: 15,
    };
    // 用点号读取字段；println! 格式化 title 时只是借用，不会移动它。
    println!("结构体：{}，计划 {} 分钟", first.title, first.minutes);

    let mut lesson = Lesson::new("enum 与 match", 20);
    // Lesson::new 使用 ::，因为它不依赖某个现成的实例。
    // lesson.add_minutes 使用 .，调用时 Rust 会自动借用 lesson 作为 &mut self。
    lesson.add_minutes(10);
    println!(
        "方法调用：{}，计划 {} 分钟，较长课程 = {}",
        lesson.title,
        lesson.minutes,
        lesson.is_long()
    );

    let pending = StudyState::NotStarted;
    let active = StudyState::Learning { minutes: 30 };
    let finished = StudyState::Finished(String::from("已完成练习"));
    show_state(&pending);
    show_state(&active);
    show_state(&finished);

    // match 也是表达式：各分支的结果需要能统一成同一个类型。
    // 这里都返回 &str，所以结果可以赋给 category；规则与第 2 课的 if 表达式相似。
    // 模式可以匹配字面量或范围；1..=29 表示包含两端的整数范围。
    let category = match lesson.minutes {
        0 => "未计时",
        1..=29 => "短课",
        _ => "较长课", // _ 匹配剩余所有情况，不绑定一个供后面使用的变量。
    };
    // _ 应放在较具体的模式后面，否则后续模式就没有机会被匹配。
    println!("match 表达式：{category}");

    // 五、if let 只关心某一种模式，不要求把其他变体全部列出来。
    // 这里继续借用 finished，成功匹配时进入块，其他变体会直接跳过这个块。
    if let StudyState::Finished(note) = &finished {
        println!("if let 读取备注：{note}");
    }
    // 需要为每一种状态安排处理时优先用 match；只关心一种情况时 if let 更简洁。

    // 常见编译错误 1：创建结构体时缺少字段。
    // 观察方法：只取消下一行开头的 //，运行本课命令，观察后恢复注释。
    // let incomplete = Lesson { title: String::from("字段不全") };
    // 修复：补齐 minutes: 15 等合法值；结构体字段不会自动获得默认值。

    // 常见编译错误 2：match 没有覆盖枚举的所有变体。
    // 观察方法：同时取消下面四行代码的 //，运行本课命令。
    // let state = StudyState::NotStarted;
    // match state {
    //     StudyState::NotStarted => {}
    // }
    // 即使当前构造了 NotStarted，match 仍须覆盖 StudyState 类型的所有可能变体。
    // 修复：补齐 Learning、Finished 分支；若确实无需处理其他变体，可添加 _ => {}。
    // 每次只启用一个反例，观察后恢复注释，再继续练习或运行最终检查。
}

// 常见编译错误 3：&self 只能共享读取，不能通过它直接修改普通字段。
// 观察方法：同时取消下面五行代码的 //，运行本课命令；无需调用该错误方法。
// impl Lesson {
//     fn cannot_edit(&self) {
//         self.minutes += 1;
//     }
// }
// 修复：需要修改此实例时改用 &mut self，并确保调用者使用可变绑定。

// 练习（每次只改一处，先预测，再编译或运行确认）：
// 1. 把 new 的初始分钟数改为 10，保持增加 10 分钟，预测 is_long 与 category 的变化。
// 2. 给 StudyState 新增 Paused 变体，先观察 show_state 的编译错误，再补齐分支并调用。
// 3. 把 if let 的输入由 &finished 改成 &pending，解释为什么最后一行输出消失。
