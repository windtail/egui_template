#![windows_subsystem = "windows"]

use anyhow::anyhow;
use clap::Parser;
use egui::ViewportBuilder;
use std::sync::Arc;
use xxx_ui::App;

#[derive(Parser, Debug)]
#[clap(author, version, about, long_about = "xxx ui")]
struct Args {
    /// some option help
    #[clap(short, long)]
    some_option: Option<String>,
}

fn main() -> anyhow::Result<()> {
    let args = Args::parse();

    if let Some(some_option) = args.some_option {
        println!("some option is {}", some_option);
    } else {
        // Log to stdout (if you run with `RUST_LOG=debug`).
        tracing_subscriber::fmt::init();

        let icon = Arc::new(eframe::icon_data::from_png_bytes(include_bytes!(
            "../assets/icon.png"
        ))?);

        let native_options = eframe::NativeOptions {
            viewport: ViewportBuilder {
                icon: Some(icon),
                title: Some("xx应用".to_owned()),
                ..Default::default()
            },
            ..Default::default()
        };
        let res = eframe::run_native(
            "xxx_name",
            native_options,
            Box::new(|cc| Box::new(App::new(cc))),
        );

        if let Err(err) = res {
            return Err(anyhow!("fail to start: {:?}", err));
        }
    }

    Ok(())
}
