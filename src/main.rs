use std::array;

use eframe::egui;

fn main() {
    let options = eframe::NativeOptions {
        viewport: egui::ViewportBuilder::default()
            .with_inner_size([512.0, 125.0])
            .with_resizable(false)
            .with_title("SyxUtil"),
        ..Default::default()
    };

    let _ = eframe::run_native(
        "My egui App",
        options,
        Box::new(|_| Ok(Box::<MyApp>::default())),
    );
}

// Name, Source Long Index, Scroll/Shift Amount
const DATA_V70: [(&str, usize, u32); 22] = [
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
    scratch: String,

    raw_input: String,

    parsed: Vec<i64>,
    data: [String; DATA_V70.len()],
}

impl Default for MyApp {
    fn default() -> Self {
        Self {
            scratch: "Current Settlement Name and Index".to_owned(),
            raw_input: "".to_owned(),
            parsed: vec![],
            data: array::from_fn(|_| "00".to_owned()),
        }
    }
}
impl eframe::App for MyApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        egui::CentralPanel::default().show(ctx, |ui| {
            ui.heading("Scratch Space");
            ui.text_edit_singleline(&mut self.scratch);

            ui.heading("Region Input");

            ui.text_edit_singleline(&mut self.raw_input);

            if ui.button("Read").clicked() {
                self.parsed = self
                    .raw_input
                    .trim_matches(['[', ']'].as_ref())
                    .split(',')
                    .map(|s| s.trim().parse::<i64>().unwrap())
                    .collect();

                println!("{}", self.parsed[35]);
                println!("{}", self.parsed[36]);

                for (i, (_, index, shift)) in DATA_V70.iter().enumerate() {
                    let value = read_prospect(self.parsed[*index], *shift);
                    self.data[i] = format!("{:02b}", value);
                }

                ctx.send_viewport_cmd(egui::ViewportCommand::InnerSize((512.0, 760.0).into()));
            }

            if self.parsed.len() > 0 {
                ui.heading("Read Binary");

                ui.label(format!("35 = {:064b}", self.parsed[35]));
                ui.label(format!("36 = {:064b}", self.parsed[36]));

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

                ui.horizontal(|ui| {
                    if ui.button("Copy").clicked() {
                        ctx.copy_text(self.parsed[35].to_string());
                    }
                    ui.label(format!("35 = {}", self.parsed[35]));
                });

                ui.horizontal(|ui| {
                    if ui.button("Copy").clicked() {
                        ctx.copy_text(self.parsed[36].to_string());
                    }
                    ui.label(format!("36 = {}", self.parsed[36]));
                });
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
