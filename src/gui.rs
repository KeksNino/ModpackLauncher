use iced::widget::{button, column};
use quick_xml::reader::Reader;

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
    fn update(&mut self, _message: Message) -> () {
        match _message {
            Message::DoSomething => (fetch_versions()),
        }
    }

    fn view(&self) -> iced::Element<Message> {
        column![button("Add Instance").on_press(Message::DoSomething),].into()
    }
}

fn fetch_versions() {
    let xml = reqwest::get(
        "https://maven.minecraftforge.net/net/minecraftforge/forge/maven-metadata.xml",
    );
    let mut reader = Reader::from_str(xml);
    reader.config_mut().trim_text(true);
    println!("{reader:?}");
}
