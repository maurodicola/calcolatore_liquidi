// Atelier Liquidi per il computer: la stessa pagina del sito, più l'archivio delle ricette
// in Documenti\Atelier Liquidi\atelier-liquidi-ricette.json, letto e scritto senza chiedere permessi.
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use std::{fs, path::PathBuf};
use tauri::Manager;

const FOLDER: &str = "Atelier Liquidi";
const FILE: &str = "atelier-liquidi-ricette.json";

fn archive_path(app: &tauri::AppHandle) -> Result<PathBuf, String> {
    let dir = app
        .path()
        .document_dir()
        .map_err(|e| format!("cartella Documenti non trovata: {e}"))?
        .join(FOLDER);
    fs::create_dir_all(&dir).map_err(|e| format!("impossibile creare {}: {e}", dir.display()))?;
    Ok(dir.join(FILE))
}

/// Percorso completo dell'archivio, da mostrare nella pagina.
#[tauri::command]
fn archive_location(app: tauri::AppHandle) -> Result<String, String> {
    Ok(archive_path(&app)?.display().to_string())
}

/// Contenuto dell'archivio, oppure nulla se il file non esiste ancora.
#[tauri::command]
fn load_archive(app: tauri::AppHandle) -> Result<Option<String>, String> {
    let path = archive_path(&app)?;
    if !path.exists() {
        return Ok(None);
    }
    fs::read_to_string(&path)
        .map(Some)
        .map_err(|e| format!("impossibile leggere {}: {e}", path.display()))
}

/// Scrive l'archivio passando da un file temporaneo, così un'interruzione non lascia il file a metà.
#[tauri::command]
fn save_archive(app: tauri::AppHandle, text: String) -> Result<(), String> {
    let path = archive_path(&app)?;
    let tmp = path.with_extension("json.tmp");
    fs::write(&tmp, text.as_bytes()).map_err(|e| format!("impossibile scrivere {}: {e}", tmp.display()))?;
    fs::rename(&tmp, &path).map_err(|e| format!("impossibile aggiornare {}: {e}", path.display()))
}

fn main() {
    tauri::Builder::default()
        .invoke_handler(tauri::generate_handler![archive_location, load_archive, save_archive])
        .run(tauri::generate_context!())
        .expect("errore all'avvio di Atelier Liquidi");
}
