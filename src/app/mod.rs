use egui::{FontData, FontDefinitions, FontFamily, FontId, TextStyle};

#[derive(Default)]
pub struct App {
    name: String,
}

impl App {
    pub fn cn_fonts() -> FontDefinitions {
        let mut fonts = FontDefinitions::default();

        fonts.font_data.insert(
            "wqy".to_owned(),
            FontData::from_static(include_bytes!("../../assets/WenQuanWeiMiHei.ttf")),
        );

        fonts
            .families
            .get_mut(&FontFamily::Proportional)
            .unwrap()
            .insert(0, "wqy".to_owned());

        fonts
            .families
            .get_mut(&FontFamily::Monospace)
            .unwrap()
            .insert(0, "wqy".to_owned());

        fonts
    }

    pub fn new(cc: &eframe::CreationContext<'_>) -> Self {
        cc.egui_ctx.set_fonts(Self::cn_fonts());

        let mut app: App = Default::default();

        app.name = "World".to_owned();

        app
    }
}

impl eframe::App for App {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        egui::CentralPanel::default().show(ctx, |ui| {
            let mut style = (*ctx.style()).clone();
            style.text_styles = [
                (
                    TextStyle::Heading,
                    FontId::new(32.0, FontFamily::Proportional),
                ),
                (TextStyle::Body, FontId::new(24.0, FontFamily::Proportional)),
                (
                    TextStyle::Monospace,
                    FontId::new(24.0, FontFamily::Proportional),
                ),
                (
                    TextStyle::Button,
                    FontId::new(24.0, FontFamily::Proportional),
                ),
                (
                    TextStyle::Small,
                    FontId::new(16.0, FontFamily::Proportional),
                ),
            ]
            .into();
            ctx.set_style(style);

            ui.label(format!("你好 {}!", self.name));
            if ui.button("点击世界").clicked() {
                self.name = "世界".to_owned()
            }
        });
    }
}
