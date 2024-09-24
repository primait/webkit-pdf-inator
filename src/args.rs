use clap::Parser;
use std::path::PathBuf;
use url::Url;

#[derive(Clone, Parser)]
pub struct Args {
    #[arg(
        name = "file",
        short,
        long,
        required_unless_present = "url",
        conflicts_with = "url",
        help = "file to convert"
    )]
    /// Local file to convert.
    /// Mutually exclusive with `--url`
    pub input_file: Option<PathBuf>,

    #[arg(name = "url", short, long, help = "url to convert")]
    /// Url to convert.
    /// This option is slightly broken, and fails to print if a website ever redirects.
    /// Mutually exclusive with `--file`
    ///
    /// Security note: no validation is performed on these urls, they can be used to convert local
    /// files, with the `file://` protocol, perform SSRF or use whatever protocols gio happens to
    /// support. You are responsible for sanitizing them to prevent these issues
    pub input_url: Option<Url>,

    #[arg(default_value = "output.pdf")]
    pub output_file: PathBuf,
}
