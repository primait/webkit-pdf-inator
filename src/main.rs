use anyhow::Result;
use clap::Parser;
use tracing_subscriber::{fmt, prelude::*, EnvFilter};
use webkit_pdf_inator::args::Args;

fn main() -> Result<()> {
    glib::log_set_default_handler(glib::rust_log_handler);

    tracing_subscriber::registry()
        .with(fmt::layer())
        .with(EnvFilter::from_default_env())
        .init();

    let args = Args::parse();
    webkit_pdf_inator::print(args)
}
