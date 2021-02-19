#[allow(dead_code)]
#[derive(PartialEq)]
pub enum Type {
    Default,
    Warning,
    Error,
    Success,
}

impl Type {
    fn color(&self) -> term::color::Color {
        match self {
            Type::Default => term::color::WHITE,
            Type::Warning => term::color::YELLOW,
            Type::Error => term::color::RED,
            Type::Success => term::color::GREEN,
        }
    }
}

pub fn println(text: &str, c_type: Type) {
    println!();
    print(text, c_type);
}

pub fn print(text: &str, c_type: Type) {
    let mut t = term::stdout().unwrap();
    if c_type != Type::Default {
        t.fg(c_type.color()).unwrap();
    }
    let _ = writeln!(t, "{}", text);
    t.reset().unwrap();
}
