pub mod app {
    use super::{
        file::{File, FileMessage},
        style,
    };
    use crate::image::{self, Device};
    use iced::{
        button, scrollable, Button, Column, Container, Element, Length, Row, Sandbox, Scrollable,
        Text,
    };
    use rfd::FileDialog;
    use std::{path::Path, thread};

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
                .style(style::button::Button::Normal);
            let convert_button = Button::new(&mut self.convert_button, Text::new("Convert"))
                .on_press(AppMessage::ConvertPressed)
                .style(style::button::Button::Normal);
            let files_list = self.files.iter_mut().enumerate().fold(
                Scrollable::new(&mut self.files_scrollable)
                    .spacing(5)
                    .padding(10),
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
                    Row::new().width(Length::Fill).height(Length::Fill).push(
                        Container::new(files_list)
                            .style(style::container::Container::Normal)
                            .height(Length::Fill)
                            .width(Length::Fill),
                    ),
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
                        for file in self.files.iter_mut() {
                            file.converting = true;
                            let path = Path::new(&file.path);
                            image::process_image(path, &Device::KoboForma);
                            file.converting = false;
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
    use iced::{button, Button, Color, Element, Length, Row, Text};
    use std::path::PathBuf;

    pub struct File {
        pub path: PathBuf,
        pub converting: bool,
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
                converting: false,
                delete_button: button::State::new(),
            }
        }

        pub fn view(&mut self) -> Element<FileMessage> {
            // TODO: This is ugly
            let filename = self.path.file_name().unwrap().to_str().unwrap();
            let text = Text::new(filename).color(Color::WHITE).width(Length::Fill);
            let delete_button = Button::new(&mut self.delete_button, Text::new(""))
                .on_press(FileMessage::Delete)
                .height(Length::Units(14))
                .width(Length::Units(14))
                .style(style::button::Button::Destructive);

            let row = Row::new().spacing(20).push(text);

            if !self.converting {
                let row = row.push(delete_button);
                return row.into();
            }

            row.into()
        }
    }
}

mod style {
    pub mod button {
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

            fn disabled(&self) -> button::Style {
                let active = self.active();

                match self {
                    Button::Normal => button::Style {
                        background: Some(Background::Color(Color::from_rgba(
                            0.65, 0.65, 0.65, 0.7,
                        ))),
                        ..active
                    },
                    Button::Destructive => button::Style {
                        background: Some(Background::Color(Color::from_rgba(0.6, 0.15, 0.15, 0.7))),
                        ..active
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

    pub mod container {
        use iced::{
            container::{self, Style},
            Background,
        };

        pub enum Container {
            Normal,
        }

        impl container::StyleSheet for Container {
            fn style(&self) -> Style {
                return Style {
                    background: Some(Background::Color(iced::Color::from_rgb8(55, 55, 55))),
                    border_radius: 5.0,
                    ..container::Style::default()
                };
            }
        }
    }
}
