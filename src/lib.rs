pub mod args;
mod printer;
mod webview;
use args::Args;
use glib_macros::clone;
use gtk4::{prelude::*, Application};
use url::Url;
use webkit6::glib;

pub fn print(args: Args) -> glib::ExitCode {
    let app = Application::new(Some("com.helloprima.webkit-pdf-inator"), Default::default());
    app.connect_activate(clone!(
        #[strong]
        args,
        move |app| {
            let uri = Url::from_file_path(&args.input).unwrap().to_string();
            glib::spawn_future_local(clone!(
                #[strong]
                args,
                #[weak]
                app,
                async move {
                    let webview_cfg = webview::WebviewConfig { uri };
                    let webview = webview_cfg.run(&app).await.unwrap();

                    printer::PrintConfig::new(args.output_file.clone())
                        .print(&webview)
                        .await
                        .unwrap();
                }
            ));
        }
    ));

    // Use run_with_args here, since we rely on clap to do our arg parsing
    app.run_with_args::<&str>(&[])
}

#[cfg(test)]
mod test {
    #[test]
    fn test_add() {
        assert_eq!(1 + 2, 3);
    }
}
