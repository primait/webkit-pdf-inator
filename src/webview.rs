use anyhow::Result;
use futures::channel::oneshot::{self, Sender};
use gtk4::{prelude::*, ApplicationWindow};
use std::cell::Cell;
use webkit6::{prelude::*, LoadEvent, WebView};

pub struct WebviewConfig {
    pub uri: String,
}

impl WebviewConfig {
    pub async fn run(self, window: &ApplicationWindow) -> Result<WebView> {
        let webview = WebView::new();
        webview.load_uri(&self.uri);

        window.set_child(Some(&webview));
        let (s, r) = oneshot::channel();

        let s: Cell<Option<Sender<()>>> = Cell::new(Some(s));
        let handle = webview.connect_load_changed(move |_webview, event| {
            if event != LoadEvent::Finished {
                return;
            }

            // Confirm that load finished.
            // If this gets called multiple times
            // (which it shouldn't, since we always disconnect the handle immiedately)
            // ignore
            if let Some(s) = s.take() {
                s.send(()).unwrap();
            } else {
                tracing::warn!("connect_load_changed called multiple times. This shouldn't happen");
            };
        });

        r.await?;
        webview.disconnect(handle);
        Ok(webview)
    }
}
