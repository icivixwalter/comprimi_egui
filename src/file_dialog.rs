use rfd::FileDialog;



//@funzione@scegli@cartella_(per ora restituisco solo le cartelle)
pub fn scegli_cartella()-> Option<String> {
    FileDialog::new()
        .pick_folder()
        .map(|p| p.display().to_string())
}