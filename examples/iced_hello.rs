// iced 入门：最简 Hello World
//
// 学习目标：
//   - Sandbox trait 的四个核心方法（new / title / update / view）
//   - text 组件
//   - Element 作为 view 的返回类型
//
// 核心概念：
//   iced 应用 = 状态(struct) + 消息(enum) + 更新逻辑(update) + 视图(view)
//   Sandbox 是最简单的应用 trait，适合纯同步场景。
//
// 运行：cargo run --example iced_hello

use iced::widget::{column, text};
use iced::{Element, Sandbox, Settings};

// 应用状态：Hello 无状态，但必须是一个 struct
struct Hello;

// 消息枚举：定义所有可能的用户交互
// 当前无交互，留空即可
#[derive(Debug, Clone)]
enum Message {}

impl Sandbox for Hello {
    type Message = Message;

    // 初始化状态
    fn new() -> Self {
        Hello
    }

    // 窗口标题
    fn title(&self) -> String {
        "Hello, Iced!".into()
    }

    // 处理消息：当前无消息可处理
    fn update(&mut self, _message: Message) {}

    // 构建视图：返回 Element 树
    // column! 宏纵向排列子组件
    fn view(&self) -> Element<'_, Message> {
        column![
            text("Hello, Iced!").size(32),
            text("欢迎来到 Rust GUI 的世界").size(16),
        ]
        .spacing(10)
        .padding(20)
        .into()
    }
}

fn main() -> iced::Result {
    Hello::run(Settings::default())
}
