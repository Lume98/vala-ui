// iced 异步：Application trait 与 Command
//
// 学习目标：
//   - Application trait（vs Sandbox）— 支持 Command 和 Subscription
//   - Command::perform 启动异步任务
//   - 异步函数在线程池上执行，不阻塞 UI 线程
//   - 加载状态 → 完成状态的 UI 切换
//
// Sandbox vs Application：
//   Sandbox   — 同步，update() 无返回值
//   Application — 异步，update() 返回 Command<Message>
//   需要额外指定 type Executor 和 type Flags
//
// 运行：cargo run --example iced_async

use iced::widget::{button, column, container, text};
use iced::{Application, Command, Element, Length, Settings};

struct AsyncApp {
    status: Status,
}

enum Status {
    Idle,
    Loading,
    Done(String),
}

#[derive(Debug, Clone)]
enum Message {
    Fetch,
    DataLoaded(String),
}

impl Application for AsyncApp {
    type Message = Message;
    type Theme = iced::Theme;
    type Executor = iced::executor::Default;
    type Flags = ();

    // Application::new 接收 flags 参数，返回 (Self, Command)
    fn new(_flags: ()) -> (Self, Command<Message>) {
        (
            Self {
                status: Status::Idle,
            },
            Command::none(), // 初始化时无需执行命令
        )
    }

    fn title(&self) -> String {
        "异步示例".into()
    }

    // update 返回 Command<Message>，可启动异步任务
    fn update(&mut self, message: Message) -> Command<Message> {
        match message {
            Message::Fetch => {
                self.status = Status::Loading;
                // Command::perform: 在线程池上执行 async 函数
                // 完成后将结果包装为 DataLoaded 消息发回 update
                Command::perform(fetch_data(), Message::DataLoaded)
            }
            Message::DataLoaded(data) => {
                self.status = Status::Done(data);
                Command::none()
            }
        }
    }

    fn view(&self) -> Element<'_, Message> {
        let content = match &self.status {
            Status::Idle => column![
                text("点击按钮模拟异步数据获取").size(18),
                button("获取数据").on_press(Message::Fetch),
            ],
            Status::Loading => column![text("加载中，请稍候...").size(18),],
            Status::Done(data) => column![
                text("获取成功！").size(18),
                text(data).size(14),
                button("重新获取").on_press(Message::Fetch),
            ],
        }
        .spacing(15)
        .padding(20);

        container(content)
            .width(Length::Fill)
            .height(Length::Fill)
            .center_x()
            .center_y()
            .into()
    }
}

/// 模拟异步数据获取（2 秒延迟）
/// Command::perform 在 iced 的线程池上运行此函数，不会阻塞 UI
async fn fetch_data() -> String {
    std::thread::sleep(std::time::Duration::from_secs(2));
    "数据加载完成！这是从异步任务返回的结果。".into()
}

fn main() -> iced::Result {
    AsyncApp::run(Settings::default())
}
