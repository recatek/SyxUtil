use std::array;
use std::num::ParseIntError;
use std::sync::Arc;

use eframe::{egui, icon_data};
use native_dialog::{DialogBuilder, MessageLevel};

const ICON: &[u8] = include_bytes!("..\\assets\\icon.png");
const WIDTH: f32 = 512.0;
const HEIGHT_SMALL: f32 = 193.0;
const HEIGHT_LARGE: f32 = 845.0;

// The relevant affected indices for setting prospect flags
const INDICES_V70: &[usize] = &[35, 36];
// Name, Source Long Index, Scroll/Shift Amount
const DATA_V70: &[(&str, usize, u32)] = &[
    ("Woodcutter", 35, 14),
    ("Fishery", 35, 40),
    ("Clay", 35, 42),
    ("Coal", 35, 44),
    ("Gem", 35, 46),
    ("Ore", 36, 16),
    ("Sithilon", 36, 18),
    ("Stone", 36, 20),
    ("Cotton", 36, 22),
    ("Fruit", 36, 24),
    ("Grain", 36, 26),
    ("Herbs", 36, 28),
    ("Mushroom", 36, 30),
    ("Opiates", 36, 32),
    ("Vegetables", 36, 34),
    ("Orchard", 36, 36),
    ("Auroch", 36, 38),
    ("Balticrawler", 36, 40),
    ("Entelodon", 36, 42),
    ("Globdien", 36, 44),
    ("Warbeast", 36, 46),
    ("Onx", 36, 48),
];

fn main() {
    let icon = icon_data::from_png_bytes(ICON).unwrap();

    let options = eframe::NativeOptions {
        viewport: egui::ViewportBuilder::default()
            .with_inner_size([WIDTH, HEIGHT_SMALL])
            .with_resizable(false)
            .with_icon(Arc::new(icon)),
        ..Default::default()
    };

    let _ = eframe::run_native(
        "SyxUtil v70",
        options,
        Box::new(|_| Ok(Box::<AppState>::default())),
    );
}

struct AppState {
    scratch: String,

    raw_input: String,

    parsed: Vec<i64>,
    data: [String; DATA_V70.len()],
}

impl Default for AppState {
    fn default() -> Self {
        Self {
            scratch: "0".to_owned(),
            raw_input: "".to_owned(),
            parsed: vec![],
            data: array::from_fn(|_| "00".to_owned()),
        }
    }
}

impl eframe::App for AppState {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        egui::CentralPanel::default().show(ctx, |ui| {
            ui.heading("Scratch Space");

            ui.label(
                "With the settlement window open, set a breakpoint in the RDProspects.get \n\
                function and record the reg.index value here for reference in the next step:",
            );

            ui.horizontal(|ui| {
                ui.label("N = ");
                ui.text_edit_singleline(&mut self.scratch);
            });

            ui.heading("Region Input");

            ui.label(
                "Set a breakpoint in WORLD.update's for loop and copy the data from\n\
                w.resources.es[3].regionData[N] for index N (e.g. [123456, 98765, ...]):",
            );
            ui.text_edit_singleline(&mut self.raw_input);

            if ui.button("Read").clicked() {
                if let Ok(parsed) = parse_input(&self.raw_input) {
                    self.parsed = parsed;

                    for (i, (_, index, shift)) in DATA_V70.iter().enumerate() {
                        let value = read_prospect(self.parsed[*index], *shift);
                        self.data[i] = format!("{:02b}", value);
                    }

                    ctx.send_viewport_cmd(egui::ViewportCommand::InnerSize(
                        (WIDTH, HEIGHT_LARGE).into(),
                    ));
                } else {
                    self.parsed.clear();

                    let _ = DialogBuilder::message()
                        .set_title("Error")
                        .set_text("Failed to parse input.")
                        .set_level(MessageLevel::Error)
                        .alert()
                        .show();

                    ctx.send_viewport_cmd(egui::ViewportCommand::InnerSize(
                        (WIDTH, HEIGHT_SMALL).into(),
                    ));
                }
            }

            if self.parsed.len() > 0 {
                ui.heading("Read Binary");

                for index in INDICES_V70.iter() {
                    ui.label(format!("{} = {:064b}", index, self.parsed[*index]));
                }

                ui.heading("Prospect Flags");

                for (i, (name, index, shift)) in DATA_V70.iter().enumerate() {
                    ui.horizontal(|ui| {
                        ui.text_edit_singleline(&mut self.data[i]);
                        ui.label(egui::RichText::new(*name).color(get_color(&self.data[i])));
                    });

                    let value = i64::from_str_radix(&self.data[i], 2).unwrap_or(0);
                    write_prospect(&mut self.parsed[*index], *shift, value)
                }

                ui.heading("Output");

                ui.label("Copy these modified values back into the regionData[N] entry:");

                for index in INDICES_V70.iter() {
                    ui.horizontal(|ui| {
                        if ui.button("Copy").clicked() {
                            ctx.copy_text(self.parsed[*index].to_string());
                        }
                        ui.label(format!("{} = {}", index, self.parsed[*index]));
                    });
                }
            }
        });
    }
}

fn get_color(value: &str) -> egui::Color32 {
    if value.chars().all(|c| c == '0' || c == '1') == false || value.len() != 2 {
        return egui::Color32::RED;
    }

    match value {
        "00" => egui::Color32::GRAY,
        "01" => egui::Color32::ORANGE,
        "10" => egui::Color32::DARK_GREEN,
        "11" => egui::Color32::GREEN,
        _ => egui::Color32::RED,
    }
}

fn read_prospect(value: i64, shift: u32) -> i64 {
    value >> shift & 0b11
}

fn write_prospect(value: &mut i64, shift: u32, prospect: i64) {
    let mask = !(0b11 << shift);
    *value = (*value & mask) | ((prospect & 0b11) << shift)
}

fn parse_input(input: &str) -> Result<Vec<i64>, ParseIntError> {
    let mut parsed = Vec::new();

    for value in input.trim_matches(['[', ']'].as_ref()).split(',') {
        parsed.push(value.trim().parse::<i64>()?);
    }

    Ok(parsed)
}
