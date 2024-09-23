use clap::Parser;
use tracing_subscriber::{fmt, prelude::*, EnvFilter};
use webkit6::glib;
use webkit_pdf_inator::args::Args;

fn main() -> glib::ExitCode {
    tracing_subscriber::registry()
        .with(fmt::layer())
        .with(EnvFilter::from_default_env())
        .init();

    let args = Args::parse();
    webkit_pdf_inator::print(args)
}
