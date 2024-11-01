mod gui;
mod file_dialog;

use crate::gui::MyApp;
use eframe::egui;


// Funzione principale che avvia l'applicazione
fn main() -> eframe::Result {
    let options = eframe::NativeOptions {
        //@finestra@base@windosw_(crea la finestra base di larghezza e altezza predefiniti)
        viewport: egui::ViewportBuilder::default().with_inner_size([620.0, 320.0]), //larghezza+ altezza
        ..Default::default()
    };
    // attiva il ciclo di rendering e agni iterazione del ciclo chiama il metodo App::update()
    // nel quale è presente il codice della grafica.
    eframe::run_native(
        "My egui App",
        options,
        Box::new(|_cc| { // creo MyApp sull'heap grazie a Box(smart pointer)
            println!("Ho creato la MyApp con un puntatore sull'heap che puo essere modificato");
            return Ok(Box::<MyApp>::default()); // crea + restituisce il puntatore alla Myapp sull'heap
        }),
    )
}
