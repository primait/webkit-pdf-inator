use anyhow::{Context, Result};
use glib_macros::clone;
use gtk4::{prelude::*, ApplicationWindow};
use webkit6::{prelude::*, LoadEvent, WebView};

use crate::utils::runtime_oneshot;

pub struct WebviewConfig {
    pub uri: String,
}

impl WebviewConfig {
    pub async fn run(self, window: &ApplicationWindow) -> Result<WebView> {
        let settings = webkit6::Settings::builder()
            // TODO: make these customizeable
            .allow_file_access_from_file_urls(false)
            .allow_universal_access_from_file_urls(false)
            .enable_javascript(false)
            .build();
        let webview = WebView::builder().settings(&settings).build();
        webview.load_uri(&self.uri);

        window.set_child(Some(&webview));
        let (s, r) = runtime_oneshot::<Result<()>>();
        let handle = webview.connect_load_changed(clone!(
            #[strong]
            s,
            move |_webview, event| {
                if event != LoadEvent::Finished {
                    return;
                }

                // Confirm that load finished.
                // If this gets called multiple times
                // (which it shouldn't, since we always disconnect the handle immiedately)
                // ignore
                s.send(Ok(())).ok();
            }
        ));

        webview.connect_load_failed(clone!(
            #[strong]
            s,
            move |_, _, url, err| {
                let err = Err(err.clone()).context(format!("While loading {url}"));
                s.send(err).unwrap();
                false
            }
        ));

        r.await??;
        webview.disconnect(handle);
        Ok(webview)
    }
}
