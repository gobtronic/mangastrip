pub mod app {
    use super::file::{File, FileMessage};
    use crate::image::{self, Device};
    use iced::{button, Button, Column, Element, Row, Sandbox, Text};
    use rfd::FileDialog;
    use std::path::Path;

    #[derive(Default)]
    pub struct App {
        pub files: Vec<File>,
        input_button: button::State,
        convert_button: button::State,
    }

    #[derive(Debug, Clone)]
    pub enum AppMessage {
        InputPressed,
        ConvertPressed,
        FileMessage(usize, FileMessage),
    }

    impl Sandbox for App {
        type Message = AppMessage;

        fn view(&mut self) -> Element<AppMessage> {
            let select_button = Button::new(&mut self.input_button, Text::new("Select files"))
                .on_press(AppMessage::InputPressed);
            let files_list = self.files.iter_mut().enumerate().fold(
                Column::new().spacing(20),
                |column, (i, file)| {
                    column.push(
                        file.view()
                            .map(move |message| AppMessage::FileMessage(i, message)),
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
                            .on_press(AppMessage::ConvertPressed),
                    ),
                )
                .into()
        }

        fn update(&mut self, message: AppMessage) {
            match message {
                AppMessage::InputPressed => {
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
                AppMessage::ConvertPressed => {
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
                AppMessage::FileMessage(i, _) => {
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
}

mod file {
    use std::path::PathBuf;
    use iced::{button, Align, Button, Element, Row, Text};
    use super::style;

    pub struct File {
        pub path: PathBuf,
        pub delete_button: button::State,
    }

    #[derive(Debug, Clone)]
    pub enum FileMessage {
        Delete,
    }

    #[derive(Debug, Clone)]
    pub struct FileState {}

    impl File {
        pub fn new(path: PathBuf) -> Self {
            File {
                path,
                delete_button: button::State::new(),
            }
        }

        pub fn view(&mut self) -> Element<FileMessage> {
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
