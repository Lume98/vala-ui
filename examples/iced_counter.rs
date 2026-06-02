// iced 计数器：按钮交互与状态管理
//
// 学习目标：
//   - Message enum 定义用户交互类型
//   - struct 持有可变状态
//   - button 组件 + on_press 绑定消息
//   - row / column 布局
//   - Alignment 对齐
//
// 数据流：view() 产生 Message → update() 修改状态 → view() 重建 UI
// 这是 iced 的核心循环，类似 Elm Architecture (TEA)。
//
// 运行：cargo run --example iced_counter

use iced::widget::{button, column, row, text};
use iced::{Alignment, Element, Sandbox, Settings};

// 应用状态：持有一个计数器
struct Counter {
    value: i32,
}

// 消息：定义三种用户操作
#[derive(Debug, Clone)]
enum Message {
    Increment,
    Decrement,
    Reset,
}

impl Sandbox for Counter {
    type Message = Message;

    fn new() -> Self {
        Self { value: 0 }
    }

    fn title(&self) -> String {
        "计数器".into()
    }

    // update 根据消息类型修改状态
    fn update(&mut self, message: Message) {
        match message {
            Message::Increment => self.value += 1,
            Message::Decrement => self.value -= 1,
            Message::Reset => self.value = 0,
        }
    }

    fn view(&self) -> Element<'_, Message> {
        column![
            text("计数器").size(28),
            // text 接受 impl ToString，i32 直接转为字符串
            text(self.value.to_string()).size(50),
            // row! 横向排列按钮
            row![
                button("−").on_press(Message::Decrement),
                button("重置").on_press(Message::Reset),
                button("+").on_press(Message::Increment),
            ]
            .spacing(10),
        ]
        .spacing(20)
        .padding(20)
        .align_items(Alignment::Center)
        .into()
    }
}

fn main() -> iced::Result {
    Counter::run(Settings::default())
}
