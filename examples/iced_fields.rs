// iced 表单：多种输入控件
//
// 学习目标：
//   - text_input 文本输入（on_input 实时回调 / on_submit 回车触发）
//   - checkbox 复选框（on_toggle 切换回调）
//   - slider 滑块（范围 + 值 + 回调）
//   - container 容器居中
//   - Length::Fill 撑满可用宽度
//
// 每个 input 控件的模式相同：
//   创建控件 → 绑定回调 → 回调产生 Message → update 修改状态 → view 重建
//
// 运行：cargo run --example iced_fields

use iced::widget::{button, checkbox, column, container, row, slider, text, text_input};
use iced::{Alignment, Element, Length, Sandbox, Settings};

struct FieldsApp {
    name: String,
    agree: bool,
    volume: f32,
}

#[derive(Debug, Clone)]
enum Message {
    NameChanged(String),
    AgreeToggled(bool),
    VolumeChanged(f32),
    Submit,
}

impl Sandbox for FieldsApp {
    type Message = Message;

    fn new() -> Self {
        Self {
            name: String::new(),
            agree: false,
            volume: 50.0,
        }
    }

    fn title(&self) -> String {
        "表单控件".into()
    }

    fn update(&mut self, message: Message) {
        match message {
            Message::NameChanged(s) => self.name = s,
            Message::AgreeToggled(b) => self.agree = b,
            Message::VolumeChanged(v) => self.volume = v,
            Message::Submit => {
                println!(
                    "提交: name={}, agree={}, volume={:.0}",
                    self.name, self.agree, self.volume
                );
            }
        }
    }

    fn view(&self) -> Element<'_, Message> {
        let name_input = text_input("请输入姓名...", &self.name)
            .on_input(Message::NameChanged)
            .on_submit(Message::Submit)
            .padding(8);

        let agree_check = checkbox("我同意用户协议", self.agree)
            .on_toggle(Message::AgreeToggled);

        let volume_slider = column![
            text(format!("音量: {:.0}%", self.volume)),
            slider(0.0..=100.0, self.volume, Message::VolumeChanged),
        ];

        let content = column![
            text("表单控件示例").size(24),
            text("姓名"),
            name_input,
            agree_check,
            volume_slider,
            row![button("提交").on_press(Message::Submit)],
        ]
        .spacing(15)
        .padding(20)
        .align_items(Alignment::Center);

        container(content)
            .width(Length::Fill)
            .center_x()
            .into()
    }
}

fn main() -> iced::Result {
    FieldsApp::run(Settings::default())
}
