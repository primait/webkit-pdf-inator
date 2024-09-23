use anyhow::Result;
use futures::channel::oneshot;
use gtk4::PrintSettings;
use std::cell::Cell;
use std::path::PathBuf;
use url::Url;
use webkit6::{PrintOperation, WebView};

pub struct PrintConfig {
    output_file: PathBuf,
}
impl PrintConfig {
    pub fn new(output_file: PathBuf) -> Self {
        Self { output_file }
    }

    pub async fn print(self, webview: &WebView) -> Result<()> {
        let file = std::path::absolute(self.output_file)?;
        let output_uri = Url::from_file_path(file).unwrap();
        let print_op = PrintOperation::new(webview);

        let settings = PrintSettings::new();
        settings.set_printer("Print to File");

        settings.set(gtk4::PRINT_SETTINGS_OUTPUT_URI, Some(output_uri.as_str()));
        print_op.set_print_settings(&settings);

        let (s, r) = oneshot::channel();
        let s: Cell<Option<oneshot::Sender<()>>> = Cell::new(Some(s));
        print_op.connect_finished(move |_| {
            if let Some(s) = s.take() {
                s.send(()).unwrap();
            } else {
                tracing::warn!(
                    "print operation connect_finished called multiple times. This shouldn't happen"
                );
            };
        });

        print_op.print();
        Ok(r.await?)
    }
}
