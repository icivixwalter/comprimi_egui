/*
        GESTIIONE DEL FILE DIALOGO
        CARTELLE
        FILE

*/

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