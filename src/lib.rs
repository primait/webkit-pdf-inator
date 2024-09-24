pub mod args;
mod printer;
mod utils;
mod webview;
use anyhow::{bail, Context, Result};
use args::Args;
use glib_macros::clone;
use gtk4::{glib::ExitCode, prelude::*, Application, ApplicationWindow};
use url::Url;
use utils::runtime_oneshot;
use webkit6::glib;

async fn do_print(args: Args, window: ApplicationWindow) -> Result<()> {
    let uri = match (args.input_url, args.input_file) {
        (Some(url), None) => url,
        (None, Some(file)) => {
            let file = std::path::absolute(file).context("Failed to resolve path to input file")?;
            // Unwrapping here is fine since we resolved the path to be absolute
            Url::from_file_path(file).unwrap()
        }
        // The argument parsing logic should prevent this from happening
        _ => unreachable!(),
    };
    let webview_cfg = webview::WebviewConfig {
        uri: uri.to_string(),
    };
    let webview = webview_cfg.run(&window).await?;

    printer::PrintConfig::new(args.output_file.clone(), args.orientation)
        .print(&webview)
        .await?;

    Ok(())
}

pub fn print(args: Args) -> Result<()> {
    let app = Application::new(Some("com.helloprima.webkit-pdf-inator"), Default::default());
    let (s, mut r) = runtime_oneshot();

    app.connect_activate(clone!(
        #[strong]
        args,
        #[strong]
        s,
        move |app| {
            let window = ApplicationWindow::new(app);
            glib::spawn_future_local(clone!(
                #[strong]
                args,
                #[weak]
                app,
                #[weak]
                window,
                #[strong]
                s,
                async move {
                    let res = do_print(args, window).await;
                    s.send(res).unwrap();

                    app.quit()
                }
            ));
        }
    ));

    // Use run_with_args here, since we rely on clap to do our arg parsing
    let exit_code = app.run_with_args::<&str>(&[]);
    let res = r.try_recv().ok().flatten();
    match (exit_code, res) {
        (ExitCode::SUCCESS, Some(res)) => res,
        (_, None) => bail!("Printing operation didn't return result"),
        (_, _) => bail!("GTK app returned an exit code indicating failure"),
    }
}

#[cfg(test)]
mod test {
    #[test]
    fn test_add() {
        assert_eq!(1 + 2, 3);
    }
}
