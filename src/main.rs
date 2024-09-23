use clap::Parser;
use webkit6::glib;
use webkit_pdf_inator::args::Args;

fn main() -> glib::ExitCode {
    let args = Args::parse();
    webkit_pdf_inator::print(args)
}
