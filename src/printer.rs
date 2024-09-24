use anyhow::{Context, Result};
use glib_macros::clone;
use gtk4::{prelude::ObjectExt, PrintSettings};
use std::path::PathBuf;
use url::Url;
use webkit6::{PrintOperation, WebView};

use crate::utils;

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

        let (s, r) = utils::runtime_oneshot();
        let failed_signal = print_op.connect_failed(clone!(
            #[strong]
            s,
            move |_, err| {
                let err = Err(err.clone());
                let err = err.context("Printing operation failed");
                s.send(err).unwrap();
            }
        ));

        let finished_signal = print_op.connect_finished(clone!(
            #[strong]
            s,
            move |_| {
                s.send(Ok(())).unwrap();
            }
        ));

        print_op.print();
        let res = r.await;

        print_op.disconnect(failed_signal);
        print_op.disconnect(finished_signal);

        res?
    }
}
