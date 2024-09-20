use clap::Parser;
use gtk4::{prelude::*, Application, ApplicationWindow, PrintSettings};
use url::Url;
use webkit6::{glib, prelude::*, LoadEvent, PrintOperation, WebView};
use webkit_pdf_inator::args::Args;

fn main() -> glib::ExitCode {
    let args = Args::parse();
    let output_path = std::path::absolute(&args.output_file).unwrap();
    let output_uri = Url::from_file_path(output_path).unwrap();

    let app = Application::new(Some("com.helloprima.webkit-pdf-inator"), Default::default());
    app.connect_activate(move |app| {
        let webview = WebView::new();
        webview.load_uri(&args.input);

        let window = ApplicationWindow::new(app);
        window.set_child(Some(&webview));

        let app = app.clone();
        let output_uri = output_uri.clone();
        webview.connect_load_changed(move |webview, event| {
            let app = app.clone();
            if event != LoadEvent::Finished {
                return;
            }

            println!("Loaded");
            let print_op = PrintOperation::new(webview);

            let settings = PrintSettings::new();
            settings.set_printer("Print to File");

            settings.set(gtk4::PRINT_SETTINGS_OUTPUT_URI, Some(output_uri.as_str()));
            print_op.set_print_settings(&settings);

            print_op.connect_finished(move |_| {
                println!("Done");
                app.quit();
            });

            print_op.print();
        });
    });

    // Use run_with_args here, since we rely on clap to do our arg parsing
    app.run_with_args::<&str>(&[])
}
