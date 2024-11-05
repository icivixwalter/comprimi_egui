use std::process::exit;
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

    @04_LAYOUT_PANNELLO_CENTRALE:egui::CentralPanel = IL PANELLE CENTRALE che deve essere creato per ultimo


***************************************************************************************

***************************************************************************************
*/
use egui::{Context, Style, Theme, ThemePreference, Visuals, Widget};
use egui::UiKind::ScrollArea;
use egui::WidgetType::TextEdit;
use crate::file_dialog::{leggi_sottocartelle, scegli_cartella_pfn, scegli_file_pfn};

// #[derive(Default)]
pub struct MyApp {
   //variabili central panel
   checkbox_tutti: bool,
   pub cartelle_selezionate: Vec<PathSelezionabile>, // Stato delle checkbox (se selezionate o meno)
   pub path_base: String,                            //la path selezionata
   // pannello destro
   path_recenti: Vec<PathSelezionabile>,
   // pannello inferiore
   path_file_inclusi: String,
   // pannello sinistro
   elenco_inclusi: Vec<String>,
   radio_file_recenti: Vec<bool>,
   //messaggi
   messaggi:String,

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
         cartelle_selezionate: vec![], // Stato delle checkbox (se selezionate o meno)

         path_base: "".to_string(),
         path_recenti: vec![PathSelezionabile::new("recent1", true), PathSelezionabile::new("recenti2", false)],
         path_file_inclusi: "".to_string(),

         //i due vettori elenco inclusi + radio impostati a false
         elenco_inclusi: vec!["prova".to_string(), "prova2".to_string()],
         radio_file_recenti: vec![false],
         messaggi: "Pronto".to_string(),
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
   //CON LA FUNZIONE UPDATE DI MYAPP = crea le varie strutture
   fn update(&mut self, ctx: &Context, frame: &mut Frame) {

      //CAMBIA IL COLORE DELLA FORM BASE e da scegliere
      // --------------------------------------------------------------------------------------//
      let theme = self.theme();
      ctx.set_style(match theme {
         Theme::Dark => self.dark_style.clone(),
         Theme::Light => self.light_style.clone(),
      });
      // --------------------------------------------------------------------------------------//

      //PANNELLO SUPERIORE
      // ==============================================================================================================//
      egui::TopBottomPanel::top("top_panel").show(ctx, |ui| {
         self.theme_preference.radio_buttons(ui);
      });
      // ==============================================================================================================//


      //@01_LAYOUT_PANNELLO_SINISTRO:egui::SidePanel::left = per prima cosa creo il panello sinistro
      // ==============================================================================================================//
      egui::SidePanel::left("pannello_percorsi_inclusi").show(ctx, |ui| {
         ui.heading("01) PERCORSI INCLUSI");
         for percorso_incluso in self.elenco_inclusi.iter() {
            ui.label(percorso_incluso);
         }
      });
      // ==============================================================================================================//


      //@02_LAYOUT_PANNELLO_DESTRO:egui::SidePanel::right = per prima cosa creo il panello destro
      // ==============================================================================================================//

      egui::SidePanel::right("pannello_recenti").show(ctx, |ui| {
         ui.heading("02) RECENTI");

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
         //creo button seleziona nel pannello dx
         if ui.button("Seleziona").clicked() {
            //TODO
         }
      });
      // ==============================================================================================================//


      //@03_LAYOUT_PANNELLO_BASSO:egui::TopBottomPanel::bottom = TOP PANEL è il pannello basso
      // ==============================================================================================================//

      egui::TopBottomPanel::bottom("pannello_inclusioni").show(ctx, |ui| {


         // LA BARRA DI PROGRESSIONE
         //....................................................................................//
         let cartelle_totali = self.cartelle_selezionate.len();  //prende la lunghezza del vettore
         let cartelle_completate = 0;
         let progress = if cartelle_totali == 0 {
            0.0 as f32
         } else {
            (cartelle_completate / cartelle_totali) as f32
         };

         let bar = egui::ProgressBar::new(progress).rounding(0.0).show_percentage().text(format!("{cartelle_completate}/{cartelle_totali}"));
         ui.add(bar);
         //....................................................................................//


         //LABEL INCLUSIONI
         ui.vertical_centered(|ui| {
            ui.heading("03) INCLUSIONI");
         });
         //CREA IL Pannello basso + una Label + una casella di testo
         //....................................................................................//
         ui.label("File path esclusioni:"); //LABEL

         //METTO IN ORIZZONTALE LA CASELLA DI TESTO + IL BUTTO SCEGLI FILE
         ui.horizontal_centered(|ui| {
            //CASELLA DI TESTO SCEGLI FILE
            //testo assegnato alla casella di testo vien inserito nella variabile text_edit
            let text_edit = egui::TextEdit::singleline(&mut self.path_file_inclusi).clip_text(false).hint_text("inserire il file da lavorare").interactive(false);
            ui.add(text_edit);  //CASELLA DI TESTO SCEGLI FILE


            //BUTTON SCELTA DEL FILE
            //@SCEGLI@FILE = con l'event click button inserisci il file scelto nella casella di testo
            if ui.button("Scegli FILE").clicked() {
               if let Some(file) = scegli_file_pfn() {
                  //qui chiama path dei file da includere
                  self.path_file_inclusi = file;
               }
            }
         });


         //....................................................................................//

      });
      // ==============================================================================================================//


      //@04_LAYOUT_PANNELLO_CENTRALE:egui::CentralPanel = IL PANELLE CENTRALE che deve essere creato per ultimo
      // ==============================================================================================================//
      egui::CentralPanel::default().show(ctx, |ui| {
         ui.heading("04) COMPRESSIONI PATH");

         //Creare una input text disabilitata  + pulsante scegli
         // cartella (INSERITE IN UNA RIGA ORIZZONTALE)
         ui.horizontal(|ui| {

            //Hint_text allarga la casella di testo fino al valore caricato  @creo@cartella.da@caricare_(qui viene creata la casella
            // di testo per caricare le cartelle
            let text_edit = egui::TextEdit::singleline(&mut self.path_base)
               .clip_text(false).hint_text("Scegli la cartella da lavorare").interactive(false);
            ui.add(text_edit);  //aggiugno la casella di testo

            //@SCEGLI@CARTELLA = Evento click del button inserisci nella path il nome della cartella scelta
            if ui.button("Scegli cartella").clicked() {
               if let Some(cartella) = scegli_cartella_pfn() {
                  self.path_base = cartella;

                  //CHIAMO LA FUNZIONE di lettura delle sottocartelle
                  if let Ok(vettore_di_sottocartelle) = leggi_sottocartelle(&self.path_base) {
                     /* conveto il vettore di stringhe sottocartelle in un vettore di PathSelezionabile */
                     self.cartelle_selezionate = vettore_di_sottocartelle.iter()
                        .map(|cartella| PathSelezionabile::new(cartella, false)) //  map =  converte, elemento per elemento
                        .filter(|p| !p.path.contains("AA_SALVATAGGI"))
                        .collect::<Vec<PathSelezionabile>>();
                  }
               }
            }
         });

         //3 button
         //....................................................................................//
         ui.add_space(10.0);

         ui.horizontal(|ui| {
            //CREA IL BUTTON = lo visualizza + ASSEGNA LA STRUCT RESPONSE a let resp1
            let resp1 = ui.button("Comprimi Selezionati");
            if resp1.clicked() {

               let messaggi = self.comprimi_selezionati();
               let mut str = String::new();
               for messaggio in messaggi.iter() {
                  str = str + messaggio + "\n";
               }
               self.messaggi = str;
            }

            let resp2 = ui.button("Comprimi tutto");
            if resp2.clicked() {
               let messaggi =self.comprimi_tutti();
               let mut str = String::new();
               for messaggio in messaggi.iter() {
                  str = str + messaggio + "\n";
               }
               self.messaggi = str;
            }
            let resp3 = ui.button("Esci");
            if resp3.clicked() {
               exit(0);
            }
         });

         ui.label(&self.messaggi);
         //....................................................................................//
         ui.add_space(10.0);

         //CREO LE CHECK BOX
         //....................................................................................//
         //scrolla le pagine
         egui::scroll_area::ScrollArea::vertical().show(ui, |ui| {

            //FOR PER LE CHECKBOX
            //ciclo per costruire i checkbox cartelle + evento selezionato checkbox
            let mut i = 0;
            for cartella in self.cartelle_selezionate.iter_mut() {

               //assegno alla variabile il nome della cartella costruita +1
               let my_cartella = &cartella.path.clone();

               ui.horizontal(|ui| {
                  //assegno alla checkbox il valore bool + il nome costruito
                  let checkbox = ui.checkbox(&mut cartella.selezionato, my_cartella);

                  //evento click checkbox
                  if checkbox.clicked() {
                     //stampo il nome ed il valore della check box cliccata.
                     println!("Checkbox con indice {i} clicked, nome = {}", my_cartella);
                  }


               });

               i += 1;
            } //for mut my_bool
            //....................................................................................//
         });

         //SOSPESO=tutto lo spazio disponibile in altezza  -10
         //ui.add_space(ui.available_height() - 60.0);

         // TODO: crea un for per le check box e che riportano il nome dell cartelle nelle label
      }); //egui::CentralPanel
      // ==============================================================================================================//

   }
}


//******************************//
//* @LE IMPLEMENTAZIONI ESTERNE
//*
//******************************//


//CREA LA PATH SELEZIONABILE
//**********************************************************************************************************************//
#[derive(Clone)]
pub struct PathSelezionabile {
   pub selezionato: bool,
   pub path: String,
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
//**********************************************************************************************************************//
