use std::{fs, io};
use std::path::{Path, PathBuf};
use std::process::Command;
use anyhow::bail;
use crate::gui::MyApp;

impl MyApp {
   pub fn comprimi_selezionati(&self) -> Vec<String> {
      let mut messaggi=Vec::new();
      for  cartella in self.cartelle_selezionate.iter() {
         if cartella.selezionato{
            messaggi.push(self.comprimi_cartella(&*cartella.path));
         }
      }
      messaggi
   }

   pub fn comprimi_tutti(&self) -> Vec<String> {
      let mut messaggi=Vec::new();
      for cartella in self.cartelle_selezionate.iter() {
         messaggi.push(self.comprimi_cartella(&*cartella.path));
      }
      return messaggi;
   }

   fn comprimi_cartella(&self, path: &str) -> String{
      comprimi_path(&*self.path_base, path, "").unwrap_or("".into())
   }

}

fn crea_cartella_se_non_esiste(path: &str) -> std::io::Result<String> {
   let mut aa_salvataggi = PathBuf::from(path);
   aa_salvataggi.push(Path::new("AA_SALVATAGGI"));
   if !fs::exists(aa_salvataggi.as_path()).unwrap_or(false) {
      if let Err(e)= fs::create_dir(aa_salvataggi.as_path()) {
         return Err(e);
      }
   }
   println!("Creata cartella {:?}", aa_salvataggi.as_path());
   Ok(aa_salvataggi.display().to_string())
}

// ******************
pub fn comprimi_path(base_path: &str, output_zip: &str, file_list_path: &str) -> Result<String, anyhow::Error>{

   let aa_salvataggi = crea_cartella_se_non_esiste(base_path)?;

   let path = PathBuf::from(&output_zip);
   let nome_zip = path.file_name().unwrap().to_str().unwrap_or("").to_owned();


   // Definire i percorsi di base
   let cartella_sorgente = format!("{output_zip}\\{nome_zip}");
   let nome_cartella_zip = format!("{aa_salvataggi}\\{nome_zip}");
   let file_list_path = format!("{aa_salvataggi}{file_list_path}");

   // Controllare se il file esiste
   if !Path::new(&file_list_path).exists() {
      bail!("File di elenco non trovato: {}", file_list_path);
   }

   // Leggere i file dal file di elenco
   // let files = match read_file_list(&file_list_path) {
   //    Ok(f) => f,
   //    Err(e) => {
   //       eprintln!("Errore nella lettura del file di elenco: {}", e);
   //       return;
   //    }
   // };

   // Eseguire il comando 7-Zip
   match execute_7zip(&nome_cartella_zip, &cartella_sorgente,None) {
      Err(err) => {
         bail!("Errore nella creazione dell'archivio: {}", err)
      }
      Ok(msg) => Ok(msg),
   }
}
// ******************


// Funzione per eseguire 7-Zip con la lista di file
fn execute_7zip(nome_zip: &str, cartella_sorgente: &str, files: Option<&[String]>) -> io::Result<String> {

   /*UNICA ESPRESSIONE IF-ELSE
      che prende il risultato di uno dei due rami o If o else e lo salva nella
      variabile status, che contiene il risultato (lo status dell'esecuzione del comando)
      E quindi al posto di visualizzarlo su un terminale dos lavora il comando 7z in memoria
      background.
   */
   //IF
   let status = if let Some(files) = files {
      Command::new(r"C:\Program Files\7-Zip\7z.exe")
         .arg("u") // Aggiornare o creare l'archivio
         .arg("-tzip") // Specificare il formato ZIP
         .arg("-r") // Ricorsivo
         .arg(nome_zip) // Output ZIP file
         .args(files) // Lista dei file da aggiungere
         .status()?
   //ELSE
   } else {
      Command::new(r"C:\Program Files\7-Zip\7z.exe")
         .arg("u") // Aggiornare o creare l'archivio
         .arg(nome_zip) // Output ZIP file
         .arg(format!("{}/", cartella_sorgente))
         .arg("-tzip") // Specificare il formato ZIP
         .arg("-r") // Ricorsivo
         .status()?
   };


   if status.success() {
      Ok(format!("Archivio creato nella cartella: {}", nome_zip))
   } else {
      Err(io::Error::new(io::ErrorKind::Other, "Errore durante l'esecuzione di 7-Zip"))
   }
}
