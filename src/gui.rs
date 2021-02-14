use std::path::{Path, PathBuf};
use crate::image::{self, Device};
use iced::{
    button, Align, Button, Column, Element, Row, Sandbox, Text,
};
use rfd::FileDialog;

#[derive(Default)]
pub struct Input {
    pub files: Vec<File>,
    input_button: button::State,
    convert_button: button::State,
}

#[derive(Debug, Clone)]
pub enum Message {
    InputPressed,
    ConvertPressed,
    FileMessage(usize, FileMessage),
}

impl Sandbox for Input {
    type Message = Message;

    fn view(&mut self) -> Element<Message> {
        let select_button = Button::new(&mut self.input_button, Text::new("Select files"))
            .on_press(Message::InputPressed);
        let files_list = self.files.iter_mut().enumerate().fold(
            Column::new().spacing(20),
            |column, (i, file)| {
                column.push(
                    file.view()
                        .map(move |message| Message::FileMessage(i, message)),
                )
            },
        );

        Column::new()
            .padding(15)
            .push(
                Row::new()
                    .push(Column::new().push(select_button))
                    .push(Column::new().push(files_list)),
            )
            .push(
                Row::new().push(
                    Button::new(&mut self.convert_button, Text::new("Convert"))
                        .on_press(Message::ConvertPressed),
                ),
            )
            .into()
    }

    fn update(&mut self, message: Message) {
        match message {
            Message::InputPressed => {
                if let Some(files) = FileDialog::new()
                    .add_filter("Image files (.jpg, .png)", &["jpg", "png"])
                    .add_filter(
                        "Archive files (.zip, .rar, .cbz, .cbr)",
                        &["zip", "rar", "cbz", "cbr"],
                    )
                    .set_directory(&"./")
                    .pick_files()
                {
                    self.files = files.iter().map(|f| File::new(f.clone())).collect();
                }
            }
            Message::ConvertPressed => {
                if self.files.is_empty() {
                    println!("Empty");
                } else {
                    println!("Ok");
                    for file in self.files.iter() {
                        let path = Path::new(&file.path);
                        image::process_image(path, &Device::KoboForma);
                    }
                }
            }
            Message::FileMessage(i, _) => {
                self.files.remove(i);
            }
        }
    }

    fn new() -> Self {
        Self::default()
    }

    fn title(&self) -> String {
        String::from("mangastrip")
    }

    fn background_color(&self) -> iced::Color {
        iced::Color::from_rgb8(46, 46, 46)
    }

    fn scale_factor(&self) -> f64 {
        1.0
    }
}

pub struct File {
    path: PathBuf,
    pub delete_button: button::State,
}

#[derive(Debug, Clone)]
pub enum FileMessage {
    Delete,
}

#[derive(Debug, Clone)]
pub struct FileState {}

impl File {
    fn new(path: PathBuf) -> Self {
        File {
            path,
            delete_button: button::State::new(),
        }
    }

    fn view(&mut self) -> Element<FileMessage> {
        let text = Text::new(self.path.to_str().unwrap());
        let delete_button = Button::new(&mut self.delete_button, Text::new("Delete"))
            .on_press(FileMessage::Delete)
            .padding(10)
            .style(style::Button::Destructive);

        Row::new()
            .spacing(20)
            .align_items(Align::Center)
            .push(text)
            .push(delete_button)
            .into()
    }
}

mod style {
    use iced::{button, Background, Color, Vector};

    pub enum Button {
        Destructive,
    }

    impl button::StyleSheet for Button {
        fn active(&self) -> button::Style {
            match self {
                Button::Destructive => button::Style {
                    background: Some(Background::Color(Color::from_rgb(0.8, 0.2, 0.2))),
                    border_radius: 5.0,
                    text_color: Color::WHITE,
                    shadow_offset: Vector::new(1.0, 1.0),
                    ..button::Style::default()
                },
            }
        }

        fn hovered(&self) -> button::Style {
            let active = self.active();

            button::Style {
                text_color: match self {
                    _ => active.text_color,
                },
                shadow_offset: active.shadow_offset + Vector::new(0.0, 1.0),
                ..active
            }
        }
    }
}
