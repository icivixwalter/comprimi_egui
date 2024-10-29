/*
***************************************************************************************
@nuova@procedura_(parto dal Layout, poi scrivo i widget, poi creo lo stato di MyApp)
@studio@comprimi@cartelle_( CON UNA CREAZIONE DELLA @GUI che eplora le directory)

start "apri HelpGui" call "c:\CASA\CDM\LeTorri\RUST\comprimi_egui\pdf\HelpGUI.pdf"

LE OPERAZIONI PRELIMINARI:
        devo creare il layout partendo da questo ordine:
            1 sidepanel right
            2 button panel
            3 sidepanel left
            4 CENTRAL PANEL
                attenzione il central panel deve essere creato sempre per ultimo come
                previsto da egui
    @01_LAYOUT_egui::SidePanel::right = per prima cosa creo il panello sinistro
    @02_LAYOUT_egui::SidePanel::right = per prima cosa creo il panello sinistro


***************************************************************************************
*/
use std::sync::Arc;
use eframe::{App, Frame};
use egui::{Context, Style, Theme, ThemePreference, Visuals, Widget};
use egui::WidgetType::TextEdit;

// #[derive(Default)]
pub struct MyApp {
    //variabili central panel
    checkbox_tutti: bool,
    cartelle_selezionate: Vec<PathSelezionabile>, // Stato delle checkbox (se selezionate o meno)
    path_base: String,
    // pannello destro
    path_recenti: Vec<PathSelezionabile>,
    // pannello inferiore
    path_file_inclusi: String,
    // pannello sinistro
    elenco_inclusi: Vec<String>,
    radio_file_recenti: Vec<bool>,

    // pannello superiore
    //colori tema
    theme_preference: ThemePreference,
    system_theme: Option<Theme>,
    fallback_theme: Theme,
    dark_style: Arc<Style>,
    light_style: Arc<Style>,
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
            theme_preference: ThemePreference::Light,
            system_theme: Some(Theme::Dark),
            fallback_theme: Theme::Dark,
            dark_style: Arc::new(Theme::Dark.default_style()),
            light_style: Arc::new(Theme::Light.default_style()),
        }
    }
}

#[derive(Clone)]
pub struct PathSelezionabile {
    selezionato: bool,
    path: String,
}

impl MyApp {
    fn theme (& self) -> Theme {
        //scelta tema
        match self.theme_preference {
            ThemePreference::Dark => Theme::Dark,
            ThemePreference::Light => Theme::Light,
            ThemePreference::System => self.system_theme.unwrap_or(self.fallback_theme),
        }
    }
}


impl App for MyApp {

    fn update(&mut self, ctx: &Context, frame: &mut Frame) {
        let theme = self.theme();

        //CAMBIA IL COLORE DELLA FORM
        ctx.set_style(match theme {
            Theme::Dark => self.dark_style.clone(),
            Theme::Light => self.light_style.clone(),
        });

        egui::TopBottomPanel::top("top_panel").show(ctx, |ui| {
            self.theme_preference.radio_buttons(ui);
        });


        egui::SidePanel::left("pannello_percorsi_inclusi").show(ctx, |ui| {
            ui.heading("PERCORSI INCLUSI");
            for percorso_incluso in self.elenco_inclusi.iter() {
                ui.label(percorso_incluso);
            }
        });

        /*@01_LAYOUT_egui::SidePanel::right = per prima cosa creo il panello sinistro
                IMPOSTO LA LABE ELENCO RECENTI per i due radio button + un button
        */
        //--------------------------------------------------------------------------------------//
        egui::SidePanel::right("pannello_recenti").show(ctx, |ui| {
            ui.heading("RECENTI");

            //creo 2 radio button
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
            //creo
            if ui.button("Seleziona").clicked() {
                //TODO
            }
        });
        //--------------------------------------------------------------------------------------//


        //in basso pannello
        //--------------------------------------------------------------------------------------//
        egui::TopBottomPanel::bottom("pannello_inclusioni").show(ctx, |ui| {
            ui.vertical_centered(|ui| {
                ui.heading("INCLUSIONI");
            });

            ui.label("File path esclusioni:");
            let text_edit = egui::TextEdit::singleline(&mut self.path_file_inclusi).interactive(false);
            ui.add(text_edit);

            ui.button("Scegli file");
        });
        //--------------------------------------------------------------------------------------//


        egui::CentralPanel::default().show(ctx, |ui| {
            ui.heading("COMPRESSIONI PATH");

            //Creare una input text disabilitata  + pulsante scegli cartella (INSERITE IN UNA RIGA ORIZZONTALE)
            ui.horizontal(|ui| {
                let text_edit = egui::TextEdit::singleline(&mut self.path_base).desired_width(250.0).hint_text("Scegli la cartella da lavorare").interactive(false);
                ui.add(text_edit);
                ui.button("Scegli cartella");
            });



            // TODO: crea un for per le check box e che riportano il nome dell cartelle nelle label
            let bar = egui::ProgressBar::new(0.6).rounding(0.0);
            ui.add(bar);
        });
    }
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
