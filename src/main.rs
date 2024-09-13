use gtk::gio::prelude::*;
use gtk::{glib, prelude::*};
use gtk4::{self as gtk, ApplicationWindow, PrintSettings};
use webkit::{prelude::*, WebView};
use webkit6::{self as webkit, LoadEvent, PrintOperation};

fn main() -> glib::ExitCode {
    let app = gtk::Application::new(Some("org.gnome.webkit6-rs.example"), Default::default());
    app.connect_activate(move |app| {
        let window = ApplicationWindow::new(app);
        let webview = WebView::new();
        window.set_child(Some(&webview));

        webview.load_uri("file:/home/mae/webkit-test/asdf.html");
        let app = app.clone();

        webview.connect_load_changed(move |webview, event| {
            let app = app.clone();
            if event != LoadEvent::Finished {
                return;
            }
            println!("loaded");

            let settings = PrintSettings::new();

            settings.set_printer("Print to File");

            let print_op = PrintOperation::new(webview);
            print_op.connect_finished(move |_| {
                println!("DONE");
                app.quit();
            });
            print_op.set_print_settings(&settings);

            print_op.print();
        });
        window.present();
    });
    app.run()
}
