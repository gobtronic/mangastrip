use clap::Clap;

#[derive(Clap, Debug)]
#[clap(
    version = "1.0",
    author = "gobtronic",
    about = "Manga optimizer made in 🦀 !"
)]

pub struct Opt {
    /// Path to the dir or image file you want to optimize.
    #[clap(short, long)]
    pub input: String,
    /// Optional dir path to where you want to save the optimized file(s).
    /// If no output-dir is specified, file(s) will be saved in the input dir.
    #[clap(short, long = "output-dir")]
    pub output_dir: Option<String>,
    /// The device width you want to optimize for.
    #[clap(short, long)]
    pub width: u32,
    /// The device height you want to optimize for.
    #[clap(short, long)]
    pub height: u32,
}
