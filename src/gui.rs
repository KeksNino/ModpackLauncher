use iced::widget::{button, column};

pub fn run() -> iced::Result {
    iced::run("Mod Launcher", ModLauncher::update, ModLauncher::view)
}

#[derive(Debug, Clone)]
enum Message {
    DoSomething,
}

#[derive(Default)]
struct ModLauncher;

impl ModLauncher {
    fn update(&mut self, _message: Message) {}

    fn view(&self) -> iced::Element<Message> {
        column![button("Add Instance").on_press(Message::DoSomething),].into()
    }
}
