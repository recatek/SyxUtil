use std::array;

use eframe::egui;

fn main() {
    let options = eframe::NativeOptions {
        viewport: egui::ViewportBuilder::default()
            .with_inner_size([512.0, 730.0])
            .with_title("SyxUtil")
            .with_resizable(true),
        ..Default::default()
    };

    let _ = eframe::run_native(
        "My egui App",
        options,
        Box::new(|_| Ok(Box::<MyApp>::default())),
    );
}

const DATA: [(&str, usize, u32); 22] = [
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

struct MyApp {
    source_36: String,
    source_35: String,

    parsed_35: u64,
    parsed_36: u64,

    data: [String; DATA.len()],
}

impl Default for MyApp {
    fn default() -> Self {
        Self {
            source_36: "0".to_owned(),
            source_35: "0".to_owned(),

            parsed_35: 0,
            parsed_36: 0,

            data: array::from_fn(|_| "00".to_owned()),
        }
    }
}
impl eframe::App for MyApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        egui::CentralPanel::default().show(ctx, |ui| {
            ui.heading("Decimal Input");

            ui.horizontal(|ui| {
                ui.label("35 = ");
                ui.text_edit_singleline(&mut self.source_36);
            });

            ui.horizontal(|ui| {
                ui.label("36 = ");
                ui.text_edit_singleline(&mut self.source_35);
            });

            if ui.button("Read").clicked() {
                self.parsed_35 = self.source_35.parse::<u64>().unwrap_or(0);
                self.parsed_36 = self.source_36.parse::<u64>().unwrap_or(0);

                for (i, (_, source, shift)) in DATA.iter().enumerate() {
                    let value = match *source {
                        35 => read_prospect(self.parsed_35, *shift),
                        36 => read_prospect(self.parsed_36, *shift),
                        _ => panic!("invalid source"),
                    };

                    self.data[i] = format!("{:02}", value);
                }
            }

            ui.heading("Read Binary");

            ui.label(format!("35 = {:064b}", self.parsed_35));
            ui.label(format!("36 = {:064b}", self.parsed_36));

            ui.heading("Prospect Flags");

            for (i, (name, source, shift)) in DATA.iter().enumerate() {
                ui.horizontal(|ui| {
                    ui.text_edit_singleline(&mut self.data[i]);
                    ui.label(
                        egui::RichText::new(*name).color(match is_ok(&self.data[i]) {
                            true => egui::Color32::GRAY,
                            false => egui::Color32::RED,
                        }),
                    );
                });

                let value = self.data[i].parse::<u64>().unwrap_or(0);

                match *source {
                    35 => write_prospect(&mut self.parsed_35, *shift, value),
                    36 => write_prospect(&mut self.parsed_36, *shift, value),
                    _ => panic!("invalid source"),
                };
            }

            ui.heading("Decimal Output");

            ui.label(format!("35 = {}", self.parsed_35));
            ui.label(format!("36 = {}", self.parsed_36));
        });
    }
}

fn is_ok(value: &str) -> bool {
    if value.chars().all(|c| c == '0' || c == '1') == false {
        return false;
    }

    if value.len() > 2 {
        return false;
    }

    true
}

fn read_prospect(value: u64, shift: u32) -> u64 {
    value >> shift & 0b11
}

fn write_prospect(value: &mut u64, shift: u32, prospect: u64) {
    let mask = !(0b11 << shift);
    *value = (*value & mask) | (prospect << shift)
}
