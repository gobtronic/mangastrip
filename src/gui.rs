pub mod app {
    use super::{
        file::{File, FileMessage},
        style,
    };
    use crate::image::{self, Device};
    use iced::{
        button, scrollable, Button, Column, Element, Length, Row, Sandbox, Scrollable, Text,
    };
    use rfd::FileDialog;
    use std::path::Path;

    #[derive(Default)]
    pub struct App {
        pub files: Vec<File>,
        input_button: button::State,
        convert_button: button::State,
        files_scrollable: scrollable::State,
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
                .on_press(AppMessage::InputPressed)
                .style(style::Button::Normal);
            let convert_button = Button::new(&mut self.convert_button, Text::new("Convert"))
                .on_press(AppMessage::ConvertPressed)
                .style(style::Button::Normal);
            let files_list = self.files.iter_mut().enumerate().fold(
                Scrollable::new(&mut self.files_scrollable).spacing(10),
                |column, (i, file)| {
                    column.push(
                        file.view()
                            .map(move |message| AppMessage::FileMessage(i, message)),
                    )
                },
            );

            Column::new()
                .padding(15)
                .spacing(15)
                .push(
                    Row::new()
                        .width(Length::Fill)
                        .height(Length::Fill)
                        .push(Column::new().push(files_list)),
                )
                .push(
                    Row::new()
                        .width(Length::Fill)
                        .spacing(10)
                        .push(Column::new().width(Length::Fill).push(select_button))
                        .push(convert_button),
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
    use super::style;
    use iced::{button, Align, Button, Color, Element, Length, Row, Space, Text};
    use std::path::PathBuf;

    pub struct File {
        pub path: PathBuf,
        pub delete_button: button::State,
    }

    #[derive(Debug, Clone)]
    pub enum FileMessage {
        Delete,
    }

    impl File {
        pub fn new(path: PathBuf) -> Self {
            File {
                path,
                delete_button: button::State::new(),
            }
        }

        pub fn view(&mut self) -> Element<FileMessage> {
            let text = Text::new(self.path.to_str().unwrap())
                .color(Color::WHITE)
                .width(Length::Fill);
            let delete_button = Button::new(&mut self.delete_button, Text::new("✕"))
                .on_press(FileMessage::Delete)
                //.padding(10)
                .style(style::Button::Destructive);

            Row::new()
                .spacing(20)
                .align_items(Align::Center)
                .push(text)
                .push(delete_button)
                .push(Space::with_width(Length::Units(1)))
                .into()
        }
    }
}

mod style {
    use iced::{button, Background, Color, Vector};

    pub enum Button {
        Normal,
        Destructive,
    }

    impl button::StyleSheet for Button {
        fn active(&self) -> button::Style {
            let default = button::Style::default();

            match self {
                Button::Normal => button::Style {
                    background: Some(Background::Color(Color::from_rgb(0.75, 0.75, 0.75))),
                    border_radius: 5.0,
                    shadow_offset: Vector::new(1.0, 1.0),
                    ..default
                },
                Button::Destructive => button::Style {
                    background: Some(Background::Color(Color::from_rgb(0.8, 0.2, 0.2))),
                    border_radius: 5.0,
                    text_color: Color::WHITE,
                    shadow_offset: Vector::new(1.0, 1.0),
                    ..default
                },
            }
        }

        fn pressed(&self) -> button::Style {
            let active = self.active();

            match self {
                Button::Normal => button::Style {
                    background: Some(Background::Color(Color::from_rgb(0.65, 0.65, 0.65))),
                    ..active
                },
                Button::Destructive => button::Style {
                    background: Some(Background::Color(Color::from_rgb(0.6, 0.15, 0.15))),
                    ..active
                },
            }
        }

        fn hovered(&self) -> button::Style {
            let active = self.active();

            match self {
                Button::Normal => button::Style {
                    background: Some(Background::Color(Color::from_rgb(0.8, 0.8, 0.8))),
                    shadow_offset: active.shadow_offset + Vector::new(0.0, 1.0),
                    ..active
                },
                Button::Destructive => button::Style {
                    background: Some(Background::Color(Color::from_rgb(0.85, 0.25, 0.25))),
                    shadow_offset: active.shadow_offset + Vector::new(0.0, 1.0),
                    ..active
                },
            }
        }
    }
}
