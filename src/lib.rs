use egui::Color32;

#[derive(Default)]
pub struct StyleEditor {
    initial_style: Option<egui::Style>,
    edited_style: Option<egui::Style>,
    example_text: String,
    example_num: u8,
}

impl StyleEditor {
    pub fn show_window(&mut self, ctx: &egui::Context, custom_colors: &mut [&mut egui::Color32]) {
        egui::Window::new("Style editor")
            .min_width(800.0)
            .show(ctx, |ui| self.show_ui(ui, custom_colors));
    }
    pub fn show_ui(&mut self, ui: &mut egui::Ui, custom_colors: &mut [&mut egui::Color32]) {
        if self.initial_style.is_none() {
            self.initial_style = Some((*ui.global_style()).clone());
        }
        if self.edited_style.is_none() {
            self.edited_style = Some((*ui.global_style()).clone());
        }
        let Some(style) = &mut self.edited_style else {
            return;
        };
        ui.horizontal(|ui| {
            if ui.button("Reset").clicked() {
                *style = self.initial_style.as_ref().unwrap().clone();
            }
            #[cfg(feature = "postcard-serde")]
            {
                ui.separator();
                seri_ui(ui, style);
            }
        });
        ui.separator();
        egui::ScrollArea::vertical().show(ui, |ui| {
            ui.horizontal_wrapped(|ui| {
                visuals_ui(
                    &mut self.example_text,
                    &mut self.example_num,
                    ui,
                    &mut style.visuals,
                );
            });
            ui.separator();
            for (i, color) in custom_colors.iter_mut().enumerate() {
                color_ui(ui, color, &format!("Custom {i}"));
            }
        });
        // Apply the style
        ui.global_style_mut(|egui_style| {
            *egui_style = style.clone();
        });
    }
    pub fn apply(&self, ctx: &egui::Context) {
        if let Some(style) = &self.edited_style {
            ctx.global_style_mut(|egui_style| {
                *egui_style = style.clone();
            });
        }
    }
}

fn visuals_ui(
    example_text: &mut String,
    example_num: &mut u8,
    ui: &mut egui::Ui,
    vis: &mut egui::Visuals,
) {
    ui.heading("Misc visuals");
    ui.end_row();
    color_ui(ui, &mut vis.panel_fill, "panel_fill");
    ui.end_row();
    color_ui(ui, &mut vis.window_fill, "window_fill");
    ui.end_row();
    stroke_ui(ui, &mut vis.window_stroke, "window_stroke");
    ui.end_row();
    color_ui(ui, &mut vis.extreme_bg_color, "extreme_bg_color");
    ui.separator();
    ui.add(egui::TextEdit::singleline(example_text).hint_text("Example text edit"));
    ui.end_row();
    color_ui(ui, &mut vis.hyperlink_color, "hyperlink color");
    ui.separator();
    let _ = ui.link("Example link");
    ui.end_row();
    color_ui(ui, &mut vis.selection.bg_fill, "selection.bg_fill");
    stroke_ui(ui, &mut vis.selection.stroke, "selection.stroke");
    ui.separator();
    let _ = ui.selectable_label(true, "Example selectable label");
    ui.end_row();
    ui.heading("Widgets");
    ui.end_row();
    widgets_ui(ui, &mut vis.widgets, example_num);
}

#[cfg(feature = "postcard-serde")]
fn seri_ui<T: serde::Serialize + serde::de::DeserializeOwned>(ui: &mut egui::Ui, val: &mut T) {
    use base64::{Engine, prelude::BASE64_STANDARD};
    if ui.button("📎").on_hover_text("Copy to clipboard").clicked()
        && let Ok(pc) = postcard::to_allocvec(val)
    {
        let out = BASE64_STANDARD.encode(&pc);
        ui.copy_text(out);
    }
    if ui
        .button("📋")
        .on_hover_text("Paste from clipboard")
        .clicked()
        && let Ok(mut cb) = arboard::Clipboard::new()
        && let Ok(text) = cb.get_text()
        && let Ok(bytes) = BASE64_STANDARD.decode(text)
        && let Ok(new) = postcard::from_bytes::<T>(&bytes)
    {
        *val = new;
    }
}

fn widgets_ui(ui: &mut egui::Ui, widgets: &mut egui::style::Widgets, example_num: &mut u8) {
    widget_visuals_ui_top(ui, &mut widgets.noninteractive, "noninteractive");
    ui.group(|ui| {
        ui.label("Example label in a group");
    });
    ui.end_row();
    widget_visuals_ui_top(ui, &mut widgets.inactive, "inactive");
    widget_visuals_ui_top(ui, &mut widgets.hovered, "hovered");
    widget_visuals_ui_top(ui, &mut widgets.active, "active");
    let _ = ui.button("Example button");
    let _ = ui.add(egui::Slider::new(example_num, 0..=255));
    ui.end_row();
    widget_visuals_ui_top(ui, &mut widgets.open, "open");
    ui.end_row();
    ui.menu_button("Example menu button", |ui| {
        ui.menu_button("Open me", |ui| {
            let _ = ui.button("Example nested button");
        });
    });
    ui.small("Active window title uses weak_bg_fill, non-active uses panel bg fill (text uses noninteractive stroke)");
}

fn widget_visuals_ui_top(ui: &mut egui::Ui, w_vis: &mut egui::style::WidgetVisuals, label: &str) {
    ui.strong(label);
    ui.end_row();
    widget_visuals_ui_inner(ui, w_vis);
    ui.end_row();
}

fn widget_visuals_ui_inner(ui: &mut egui::Ui, vis: &mut egui::style::WidgetVisuals) {
    color_ui(ui, &mut vis.bg_fill, "bg_fill");
    color_ui(ui, &mut vis.weak_bg_fill, "weak_bg_fill");
    stroke_ui(ui, &mut vis.bg_stroke, "bg_stroke");
    ui.end_row();
    stroke_ui(ui, &mut vis.fg_stroke, "fg_stroke");
}

fn color_ui(ui: &mut egui::Ui, color: &mut Color32, label: &str) {
    ui.label(label);
    color_ui_inner(color, ui);
}

fn color_ui_inner(color: &mut Color32, ui: &mut egui::Ui) {
    ui.color_edit_button_srgba(color);
    if ui
        .button("📎")
        .on_hover_text("Copy as hex color to clipboard")
        .clicked()
    {
        ui.copy_text(color.to_hex());
    }
    if ui
        .button("📋")
        .on_hover_text("from hex color on clipboard")
        .clicked()
        && let Ok(mut cb) = arboard::Clipboard::new()
        && let Ok(text) = cb.get_text()
        && let Ok(parsed_color) = egui::Color32::from_hex(&text)
    {
        *color = parsed_color;
    }
}

fn stroke_ui(ui: &mut egui::Ui, stroke: &mut egui::Stroke, label: &str) {
    ui.label(label);
    ui.add(egui::DragValue::new(&mut stroke.width));
    color_ui_inner(&mut stroke.color, ui);
}
