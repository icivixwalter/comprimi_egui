use std::sync::Arc;
use eframe::{App, Frame};
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
    @01_LAYOUT_PANNELLO_SINISTRO:egui::SidePanel::left = per prima cosa creo il panello sinistro
    @02_LAYOUT_PANNELLO_DESTRO:egui::SidePanel::right = per prima cosa creo il panello destro
    @03_LAYOUT_PANNELLO_BASSO::egui::TopBottomPanel::bottom = TOP PANEL è il pannello basso
        Pannello basso + una Label + una casella di testo


***************************************************************************************

***************************************************************************************
*/
use egui::{Context, Style, Theme, ThemePreference, Visuals, Widget};
use egui::UiKind::ScrollArea;
use egui::WidgetType::TextEdit;
use crate::file_dialog::scegli_cartella;

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
            //dove il check box tutti ???
            checkbox_tutti: false,

            //creo il vettore delle cartelle
            cartelle_selezionate: vec![
                PathSelezionabile::new("path_2008", false),
                PathSelezionabile::new("path_2009", false),
                PathSelezionabile::new("path_2010", false),
            ], // Stato delle checkbox (se selezionate o meno)

            path_base: "".to_string(),
            path_recenti: vec![PathSelezionabile::new("recent1", true), PathSelezionabile::new("recenti2", false)],
            path_file_inclusi: "".to_string(),

            //i due vettori elenco inclusi + radio impostati a false
            elenco_inclusi: vec!["prova".to_string(), "prova2".to_string()],
            radio_file_recenti: vec![false],

            //theme = 1) colore default Light + 2) colore Dark 3) system  = non funzionante.
            theme_preference: ThemePreference::Light,
            system_theme: Some(Theme::Dark),
            fallback_theme: Theme::Dark,
            dark_style: Arc::new(Theme::Dark.default_style()),
            light_style: Arc::new(Theme::Light.default_style()),
        }
    }
}


impl MyApp {
    fn theme(&self) -> Theme {
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

        //CAMBIA IL COLORE DELLA FORM BASE e da scegliere
        ctx.set_style(match theme {
            Theme::Dark => self.dark_style.clone(),
            Theme::Light => self.light_style.clone(),
        });
        //PANNELLO SUPERIORE
        egui::TopBottomPanel::top("top_panel").show(ctx, |ui| {
            self.theme_preference.radio_buttons(ui);
        });


        //@01_LAYOUT_PANNELLO_SINISTRO:egui::SidePanel::left = per prima cosa creo il panello sinistro
        egui::SidePanel::left("pannello_percorsi_inclusi").show(ctx, |ui| {
            ui.heading("PERCORSI INCLUSI");
            for percorso_incluso in self.elenco_inclusi.iter() {
                ui.label(percorso_incluso);
            }
        });


        //@02_LAYOUT_PANNELLO_DESTRO:egui::SidePanel::right = per prima cosa creo il panello destro
        // --------------------------------------------------------------------------------------//
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


        //@03_LAYOUT_PANNELLO_BASSOegui::TopBottomPanel::bottom = TOP PANEL è il pannello basso
        //------------------------------------------------------------------------------------------------//
        egui::TopBottomPanel::bottom("pannello_inclusioni").show(ctx, |ui| {



            // LA BARRA DI PROGRESSIONE
            //....................................................................................//
            let cartelle_totali= self.cartelle_selezionate.len();  //prende la lunghezza del vettore
            let cartelle_completate= 0;
            let bar = egui::ProgressBar::new((cartelle_completate / cartelle_totali) as f32).rounding(0.0).show_percentage().text(format!("{cartelle_completate}/{cartelle_totali}"));
            ui.add(bar);
            //....................................................................................//

            ui.vertical_centered(|ui| {
                ui.heading("INCLUSIONI");
            });
            //Pannello basso + una Label + una casella di testo
            ui.label("File path esclusioni:");
            let text_edit = egui::TextEdit::singleline(&mut self.path_file_inclusi).interactive(false);
            ui.add(text_edit);

            ui.button("Scegli file");
        });
        //------------------------------------------------------------------------------------------------//

        //PANNELLO CENTRALE per ultimo
        //------------------------------------------------------------------------------------------------//
        egui::CentralPanel::default().show(ctx, |ui| {
            ui.heading("COMPRESSIONI PATH");

            //Creare una input text disabilitata  + pulsante scegli
            // cartella (INSERITE IN UNA RIGA ORIZZONTALE)
            ui.horizontal(|ui| {
                let text_edit = egui::TextEdit::singleline(&mut self.path_base).clip_text(false).hint_text("Scegli la cartella da lavorare").interactive(false);
                ui.add(text_edit);


                if ui.button("Scegli cartella").clicked(){
                    if let Some(cartella) = scegli_cartella() {
                        self.path_base = cartella;
                    }

                }
            });

            //CREO LE CHECK BOX
            //....................................................................................//
            //scrolla le pagine
            egui::scroll_area::ScrollArea::vertical().show(ui, |ui| {

                //ciclo per costruire i checkbox cartelle + evento selezionato checkbox
                let mut i = 0;
                for mut my_bool in self.cartelle_selezionate.iter_mut().map(|ps| ps.selezionato) {
                    //assegno alla variabile il nome della cartella costruita +1
                    let my_cartella = format!("{}", 2008 + i);


                    ui.horizontal(|ui| {
                        //assegno alla checkbox il valore bool + il nome costruito
                        let checkbox = ui.checkbox(&mut my_bool, &my_cartella);

                        //evento click checkbox
                        if checkbox.clicked() {
                            //stampo il nome ed il valore della check box cliccata.
                            println!("Checkbox con indice {i} clicked, nome = {}", &my_cartella);
                        }

                        ui.label("2008 prova");
                        //aggiugno spazio

                    });

                    i += 1;
                } //for mut my_bool
                //....................................................................................//
            });

            //tutto lo spazio disponibile in altezza  -10
            //ui.add_space(ui.available_height() - 60.0);

            //3 button
            //....................................................................................//
            ui.add_space(10.0);

            ui.horizontal(|ui| {
                ui.button("Comprimi Selezionati");
                ui.button("Comprimi tutto");
                ui.button("Esci");
            });
            //....................................................................................//

            // TODO: crea un for per le check box e che riportano il nome dell cartelle nelle label
        }); //egui::CentralPanel
        //------------------------------------------------------------------------------------------------//

    }
}


//CREA LA PATH SELEZIONABILE
//------------------------------------------------------------------------------------------------//
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

//------------------------------------------------------------------------------------------------//
