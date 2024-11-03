/*
        GESTIIONE DEL FILE DIALOGO
        CARTELLE
        FILE

*/
use std::fs;
use std::path::Path;
use anyhow::Error;
use rfd::FileDialog;



//@funzione@scegli@cartella_(per ora restituisco solo le cartelle)
pub fn scegli_cartella_pfn()-> Option<String> {
    FileDialog::new()
        .pick_folder()
        .map(|p| p.display().to_string())
}



//@funzione@scegli@FILE_(per ora restituisco solo I FILE)
pub fn scegli_file_pfn()-> Option<String> {
    FileDialog::new()
        .pick_file()
        .map(|p| p.display().to_string())
}


pub fn leggi_sottocartelle(directory_path: &str) -> Result<Vec<String>, anyhow::Error> {
    let path = Path::new(directory_path);

    let mut my_vettore_s: Vec<String> = vec![];
    // Verifica se il percorso è una directory
    if path.is_dir() {
        // Itera attraverso il contenuto della directory
        for entry in fs::read_dir(path)? {
            let entry = entry?;
            let entry_path = entry.path();

            // Stampa solo se l'elemento è una cartella
            if entry_path.is_dir() {
                my_vettore_s.push(entry_path.display().to_string());
                // println!("{}", entry_path.display());
            }
        }
    } else {
        eprintln!("Il percorso specificato non è una directory.");
        return Err(Error::msg("Impossibile leggere la cartella"));
    }
    return Ok(my_vettore_s);
}