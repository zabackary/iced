use iced::widget::{self, markdown, row, scrollable, text_editor};
use iced::{Element, Fill, Font, Task, Theme};

pub fn main() -> iced::Result {
    iced::application("Markdown - Iced", Markdown::update, Markdown::view)
        .theme(Markdown::theme)
        .run_with(Markdown::new)
}

struct Markdown {
    content: text_editor::Content,
    items: Vec<markdown::Item>,
    theme: Theme,
}

#[derive(Debug, Clone)]
enum Message {
    ContentChanged,
    LinkClicked(markdown::Url),
}

impl Markdown {
    fn new() -> (Self, Task<Message>) {
        const INITIAL_CONTENT: &str = include_str!("../overview.md");

        let theme = Theme::TokyoNight;

        (
            Self {
                content: text_editor::Content::with_text(INITIAL_CONTENT),
                items: markdown::parse(INITIAL_CONTENT, theme.palette())
                    .collect(),
                theme,
            },
            widget::focus_next(),
        )
    }

    fn update(&mut self, message: Message) {
        match message {
            Message::ContentChanged => {
                self.items =
                    markdown::parse(&self.content.text(), self.theme.palette())
                        .collect();
            }
            Message::LinkClicked(link) => {
                let _ = open::that_in_background(link.to_string());
            }
        }
    }

    fn view(&self) -> Element<Message> {
        let editor = text_editor(&self.content)
            .placeholder("Type your Markdown here...")
            .on_change(Message::ContentChanged)
            .height(Fill)
            .padding(10)
            .font(Font::MONOSPACE);

        let preview = markdown(&self.items, markdown::Settings::default())
            .map(Message::LinkClicked);

        row![editor, scrollable(preview).spacing(10).height(Fill)]
            .spacing(10)
            .padding(10)
            .into()
    }

    fn theme(&self) -> Theme {
        self.theme.clone()
    }
}
