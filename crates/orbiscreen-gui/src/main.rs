// Orbiscreen - Linux Desktop GUI (GPL-3.0-or-later) https://github.com/shadow-x78/orbiscreen

mod app;
mod daemon_client;
mod i18n;
mod pages;
mod qr;
mod theme;
mod tray;

use adw::prelude::*;
use gtk::glib::ExitCode;

fn cleanup_legacy_desktop_file() {
    if let Some(home) = std::env::var_os("HOME") {
        let legacy_path =
            std::path::PathBuf::from(home).join(".local/share/applications/orbiscreen.desktop");
        if legacy_path.exists() {
            if let Ok(content) = std::fs::read_to_string(&legacy_path) {
                if content.contains("Exec=orbiscreen start") {
                    let _ = std::fs::remove_file(&legacy_path);
                }
            }
        }
    }
}

fn main() -> ExitCode {
    cleanup_legacy_desktop_file();

    tracing_subscriber::fmt()
        .with_env_filter(
            tracing_subscriber::EnvFilter::try_from_default_env()
                .unwrap_or_else(|_| "orbiscreen_gui=info".into()),
        )
        .init();

    let runtime = Box::leak(Box::new(
        tokio::runtime::Builder::new_multi_thread()
            .worker_threads(2)
            .enable_all()
            .build()
            .expect("failed to spawn the tokio runtime"),
    ));

    let application = adw::Application::builder()
        .application_id(app::APP_ID)
        .build();

    let handle = runtime.handle().clone();
    application.connect_activate(move |app| {
        if let Some(window) = app.active_window() {
            window.present();
        } else {
            app::Ui::launch(app, handle.clone());
        }
    });

    application.run()
}
