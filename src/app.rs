/// We derive Deserialize/Serialize so we can persist app state on shutdown.
#[derive(serde::Deserialize, serde::Serialize)]
#[serde(default)] // if we add new fields, give them default values when deserializing old state

pub struct HtreeApp {
    // this how you opt-out of serialization of a member
    // #[serde(skip)]
    // value: f32,
    max_level: usize,
}

impl Default for HtreeApp {
    fn default() -> Self {
        Self { max_level: 1 }
    }
}
const MAX_LEVEL: usize = 16;

impl HtreeApp {
    /// Called once before the first frame.
    pub fn new(_cc: &eframe::CreationContext<'_>) -> Self {
        // This is also where you can customize the look and feel of egui using
        // `cc.egui_ctx.set_visuals` and `cc.egui_ctx.set_fonts`.

        // Load previous app state (if any).
        // Note that you must enable the `persistence` feature for this to work.
        // if let Some(storage) = cc.storage {
        //     return eframe::get_value(storage, eframe::APP_KEY).unwrap_or_default();
        // }

        Default::default()
    }
}

impl eframe::App for HtreeApp {
    /// Called by the frame work to save state before shutdown.
    fn save(&mut self, _storage: &mut dyn eframe::Storage) {
        // eframe::set_value(storage, eframe::APP_KEY, self);
    }

    /// Called each time the UI needs repainting, which may be many times per second.
    /// Put your widgets into a `SidePanel`, `TopPanel`, `CentralPanel`, `Window` or `Area`.
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        // Examples of how to create different panels and windows.
        // Pick whichever suits you.
        // Tip: a good default choice is to just keep the `CentralPanel`.
        // For inspiration and more examples, go to https://emilk.github.io/egui
        ctx.set_pixels_per_point(1.0);
        let mut offset = 0.0;

        egui::TopBottomPanel::top("top_panel").show(ctx, |ui| {
            // The top panel is often a good place for a menu bar:
            egui::menu::bar(ui, |ui| {
                ui.add(
                    egui::widgets::Slider::new(&mut self.max_level, 1..=MAX_LEVEL)
                        .prefix("Levels:"),
                );
            });
            offset = 22.0;
        });

        egui::CentralPanel::default().show(ctx, |ui| {
            // dbg!(ctx.available_rect());
            let r = ui.available_rect_before_wrap();
            // dbg!(&r);
            let center = egui::pos2(r.width() / 2.0, r.height() / 2.0 + offset);
            let l = r.width().min(r.height()) / 2.0_f32.sqrt();
            let painter = ui.painter();
            self.draw_horizontal(painter, &center, l, 0);
        });
    }
}
impl HtreeApp {
    fn draw_horizontal(
        &self,
        painter: &egui::Painter,
        pos: &egui::Pos2,
        length: f32,
        level: usize,
    ) {
        if level == self.max_level {
            return;
        }
        let x1 = pos.x - length / 2.0;
        let x2 = pos.x + length / 2.0;
        let thickness = ((MAX_LEVEL - level) as f32 / 4.0).max(1.0);
        let pen = egui::Stroke::new(thickness, egui::Color32::RED);
        painter.hline(x1..=pos.x, pos.y, pen);
        let pen = egui::Stroke::new(thickness, egui::Color32::GREEN);
        painter.hline(pos.x..=x2, pos.y, pen);
        self.draw_vertical(
            &painter,
            &egui::Pos2::new(x1, pos.y),
            length / 2.0_f32.sqrt(),
            level + 1,
        );
        self.draw_vertical(
            &painter,
            &egui::Pos2::new(x2, pos.y),
            length / 2.0_f32.sqrt(),
            level + 1,
        );
        painter.circle(
            *pos,
            thickness,
            egui::Color32::BLACK,
            egui::Stroke::new(1.0, egui::Color32::TRANSPARENT),
        );
        painter.circle(
            *pos,
            thickness / 2.0,
            egui::Color32::YELLOW,
            egui::Stroke::new(1.0, egui::Color32::TRANSPARENT),
        );
    }

    fn draw_vertical(&self, painter: &egui::Painter, pos: &egui::Pos2, length: f32, level: usize) {
        if level == self.max_level {
            return;
        }
        let y1 = pos.y - length / 2.0;
        let y2 = pos.y + length / 2.0;
        let thickness = ((MAX_LEVEL - level) as f32 / 4.0).max(1.0);
        let pen = egui::Stroke::new(thickness, egui::Color32::RED);
        painter.vline(pos.x, y1..=pos.y, pen);
        let pen = egui::Stroke::new(thickness, egui::Color32::GREEN);
        painter.vline(pos.x, pos.y..=y2, pen);
        self.draw_horizontal(
            &painter,
            &egui::Pos2::new(pos.x, y1),
            length / 2.0_f32.sqrt(),
            level + 1,
        );
        self.draw_horizontal(
            &painter,
            &egui::Pos2::new(pos.x, y2),
            length / 2.0_f32.sqrt(),
            level + 1,
        );
        painter.circle(
            *pos,
            thickness,
            egui::Color32::BLACK,
            egui::Stroke::new(1.0, egui::Color32::TRANSPARENT),
        );
        painter.circle(
            *pos,
            thickness / 2.0,
            egui::Color32::YELLOW,
            egui::Stroke::new(1.0, egui::Color32::TRANSPARENT),
        );
    }
}
