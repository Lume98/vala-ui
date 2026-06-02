// iced 待办事项：综合实战
//
// 学习目标：
//   - Vec 状态管理（增 / 删 / 改）
//   - scrollable 滚动容器
//   - Copy enum 驱动的过滤逻辑
//   - 动态构建列表 UI（Column::with_children）
//   - 条件渲染不同视图
//
// 这是 Sandbox 的综合应用，涵盖：
//   复合状态 → 集合操作 → 过滤映射 → 动态 UI 生成
//
// 运行：cargo run --example iced_todos

use iced::widget::{button, column, container, row, scrollable, text, text_input, Column};
use iced::{Alignment, Element, Length, Sandbox, Settings};

#[derive(Debug, Clone)]
struct Todo {
    text: String,
    done: bool,
}

#[derive(Debug, Clone, Copy)]
enum Filter {
    All,
    Active,
    Completed,
}

#[derive(Debug, Clone)]
enum Message {
    InputChanged(String),
    Add,
    Toggle(usize),
    Remove(usize),
    FilterChanged(Filter),
}

struct TodosApp {
    input: String,
    todos: Vec<Todo>,
    filter: Filter,
}

impl Sandbox for TodosApp {
    type Message = Message;

    fn new() -> Self {
        Self {
            input: String::new(),
            todos: Vec::new(),
            filter: Filter::All,
        }
    }

    fn title(&self) -> String {
        "待办事项".into()
    }

    fn update(&mut self, message: Message) {
        match message {
            Message::InputChanged(s) => self.input = s,
            Message::Add => {
                let text = self.input.trim().to_string();
                if !text.is_empty() {
                    self.todos.push(Todo { text, done: false });
                    self.input.clear();
                }
            }
            Message::Toggle(i) => {
                if let Some(todo) = self.todos.get_mut(i) {
                    todo.done = !todo.done;
                }
            }
            Message::Remove(i) => {
                if i < self.todos.len() {
                    self.todos.remove(i);
                }
            }
            Message::FilterChanged(f) => self.filter = f,
        }
    }

    fn view(&self) -> Element<'_, Message> {
        // 输入行
        let input_row = row![
            text_input("添加待办事项...", &self.input)
                .on_input(Message::InputChanged)
                .on_submit(Message::Add)
                .width(Length::Fill),
            button("添加").on_press(Message::Add),
        ]
        .spacing(10)
        .align_items(Alignment::Center);

        // 过滤按钮行
        let filter_row = row![
            button("全部").on_press(Message::FilterChanged(Filter::All)),
            button("进行中").on_press(Message::FilterChanged(Filter::Active)),
            button("已完成").on_press(Message::FilterChanged(Filter::Completed)),
        ]
        .spacing(5);

        // 按过滤器筛选
        let filtered: Vec<(usize, &Todo)> = self
            .todos
            .iter()
            .enumerate()
            .filter(|(_, todo)| match self.filter {
                Filter::All => true,
                Filter::Active => !todo.done,
                Filter::Completed => todo.done,
            })
            .collect();

        // 动态构建列表
        let todo_list: Element<_> = if filtered.is_empty() {
            text("暂无待办事项").into()
        } else {
            let items: Vec<Element<_>> = filtered
                .into_iter()
                .map(|(i, todo)| {
                    let mark = if todo.done { "✓ " } else { "○ " };
                    row![
                        button(text(format!("{}{}", mark, todo.text)))
                            .on_press(Message::Toggle(i))
                            .width(Length::Fill),
                        button("×").on_press(Message::Remove(i)),
                    ]
                    .spacing(8)
                    .into()
                })
                .collect();

            scrollable(Column::with_children(items).spacing(6)).into()
        };

        let content = column![
            text("待办事项").size(28),
            input_row,
            filter_row,
            text(format!("共 {} 项", self.todos.len())).size(12),
            todo_list,
        ]
        .spacing(15)
        .padding(20);

        container(content)
            .width(Length::Fill)
            .center_x()
            .into()
    }
}

fn main() -> iced::Result {
    TodosApp::run(Settings::default())
}
