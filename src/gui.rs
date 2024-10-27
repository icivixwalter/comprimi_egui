use eframe::{App, Frame};
use egui::Context;
use egui::WidgetType::TextEdit;

// #[derive(Default)]
pub struct MyApp {
    checkbox_tutti: bool,
    cartelle_selezionate: Vec<PathSelezionabile>, // Stato delle checkbox (se selezionate o meno)
    path_base: String,
    path_recenti: Vec<PathSelezionabile>,
    path_file_inclusi: String,
    elenco_inclusi: Vec<String>,
    radio_file_recenti: Vec<bool>,
}

impl Default for MyApp {
    fn default() -> Self {
        MyApp {
            checkbox_tutti: false,
            cartelle_selezionate: vec![], // Stato delle checkbox (se selezionate o meno)
            path_base: "".to_string(),
            path_recenti: vec![PathSelezionabile::new("recent1", true), PathSelezionabile::new("recenti2", false)],
            path_file_inclusi: "".to_string(),
            elenco_inclusi: vec!["prova".to_string(), "prova2".to_string()],
            radio_file_recenti: vec![false],
        }
    }
}

#[derive(Clone)]
pub struct PathSelezionabile {
    selezionato: bool,
    path: String,
}

impl PathSelezionabile {
    fn new(path: &str, selezionato: bool) -> PathSelezionabile {
        PathSelezionabile {
            path: path.to_string(),
            selezionato,
        }
    }

    fn is_selezionato(&self) -> bool {
        self.selezionato
    }

    fn get_path(&self) -> String {
        self.path.clone()
    }
}

impl App for MyApp {
    fn update(&mut self, ctx: &Context, frame: &mut Frame) {
        egui::SidePanel::left("pannello_percorsi_inclusi").show(ctx, |ui| {
            ui.heading("PERCORSI INCLUSI");
            for percorso_incluso in self.elenco_inclusi.iter() {
                ui.label(percorso_incluso);
            }
        });
        //ELENCO RECENTI
        egui::SidePanel::right("pannello_recenti").show(ctx, |ui| {
            ui.heading("RECENTI");


            let mut i = 0;
            for path in self.path_recenti.clone().iter() {
                let response = ui.radio(path.is_selezionato(), path.get_path());
                if response.clicked() {
                    let mut j = 0;
                    for p in self.path_recenti.iter_mut() {
                        p.selezionato = (i == j);
                        j += 1;
                    }
                }
                i += 1;
            }


            if ui.button("Seleziona").clicked() {
                //TODO
            }
        });

        //in basso pannello
        egui::TopBottomPanel::bottom("pannello_inclusioni").show(ctx, |ui| {
            ui.vertical_centered(|ui| {
                ui.heading("INCLUSIONI");
            });

            ui.label("File path esclusioni:");
            let text_edit = egui::TextEdit::singleline(&mut self.path_file_inclusi).interactive(false);
            ui.add(text_edit);

            ui.button("Scegli file");
        });
        egui::CentralPanel::default().show(ctx, |ui| {
            ui.heading("COMPRESSIONI PATH");

            let bar = egui::ProgressBar::new(0.6).rounding(0.0);
            ui.add(bar);
        });
    }
}