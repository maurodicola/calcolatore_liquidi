// Atelier Liquidi per il computer: la stessa pagina del sito, più l'archivio delle ricette in un file .json
// (di base Documenti\Atelier Liquidi\atelier-liquidi-ricette.json), letto e scritto senza chiedere permessi.
// La cartella dell'archivio si può cambiare; la scelta resta nel file impostazioni.json dell'app.
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use std::{fs, path::PathBuf};
use tauri::{AppHandle, Manager};
use tauri_plugin_dialog::DialogExt;

const FOLDER: &str = "Atelier Liquidi";
const FILE: &str = "atelier-liquidi-ricette.json";
const SETTINGS: &str = "impostazioni.json";

fn settings_path(app: &AppHandle) -> Result<PathBuf, String> {
    let dir = app
        .path()
        .app_config_dir()
        .map_err(|e| format!("cartella delle impostazioni non trovata: {e}"))?;
    fs::create_dir_all(&dir).map_err(|e| format!("impossibile creare {}: {e}", dir.display()))?;
    Ok(dir.join(SETTINGS))
}

fn default_folder(app: &AppHandle) -> Result<PathBuf, String> {
    Ok(app
        .path()
        .document_dir()
        .map_err(|e| format!("cartella Documenti non trovata: {e}"))?
        .join(FOLDER))
}

/// Cartella scelta dall'utente, oppure quella predefinita in Documenti.
fn archive_folder(app: &AppHandle) -> Result<PathBuf, String> {
    if let Ok(text) = fs::read_to_string(settings_path(app)?) {
        if let Ok(value) = serde_json::from_str::<serde_json::Value>(&text) {
            if let Some(folder) = value.get("cartellaArchivio").and_then(|v| v.as_str()) {
                return Ok(PathBuf::from(folder));
            }
        }
    }
    default_folder(app)
}

fn archive_path(app: &AppHandle) -> Result<PathBuf, String> {
    let dir = archive_folder(app)?;
    fs::create_dir_all(&dir).map_err(|e| format!("impossibile creare {}: {e}", dir.display()))?;
    Ok(dir.join(FILE))
}

/// Percorso completo dell'archivio, da mostrare nella pagina.
#[tauri::command]
fn archive_location(app: AppHandle) -> Result<String, String> {
    Ok(archive_path(&app)?.display().to_string())
}

/// Contenuto dell'archivio, oppure nulla se il file non esiste ancora.
#[tauri::command]
fn load_archive(app: AppHandle) -> Result<Option<String>, String> {
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
fn save_archive(app: AppHandle, text: String) -> Result<(), String> {
    let path = archive_path(&app)?;
    let tmp = path.with_extension("json.tmp");
    fs::write(&tmp, text.as_bytes()).map_err(|e| format!("impossibile scrivere {}: {e}", tmp.display()))?;
    fs::rename(&tmp, &path).map_err(|e| format!("impossibile aggiornare {}: {e}", path.display()))
}

/// «Cambia destinazione»: l'utente sceglie una cartella; l'archivio sarà atelier-liquidi-ricette.json lì dentro.
/// Restituisce il nuovo percorso, oppure nulla se la scelta viene annullata.
#[tauri::command]
async fn choose_archive_folder(app: AppHandle) -> Result<Option<String>, String> {
    let start = archive_folder(&app)?;
    let picked = app
        .dialog()
        .file()
        .set_title("Scegli la cartella per l'archivio delle ricette")
        .set_directory(if start.exists() { start } else { default_folder(&app)? })
        .blocking_pick_folder();
    let Some(picked) = picked else { return Ok(None) };
    let folder = picked.into_path().map_err(|e| format!("cartella non valida: {e}"))?;
    let settings = serde_json::json!({ "cartellaArchivio": folder.display().to_string() });
    fs::write(settings_path(&app)?, settings.to_string()).map_err(|e| format!("impossibile salvare la scelta: {e}"))?;
    Ok(Some(archive_path(&app)?.display().to_string()))
}

/// «Esporta JSON» ed «Esporta CSV»: finestra di Windows per scegliere dove salvare il file.
/// Restituisce il percorso del file salvato, oppure nulla se l'utente annulla.
#[tauri::command]
async fn export_file(app: AppHandle, name: String, text: String) -> Result<Option<String>, String> {
    let ext = name.rsplit('.').next().unwrap_or("").to_lowercase();
    let mut dialog = app.dialog().file().set_title("Esporta le ricette").set_file_name(&name);
    if ext == "csv" {
        dialog = dialog.add_filter("Foglio di calcolo CSV", &["csv"]);
    } else if ext == "json" {
        dialog = dialog.add_filter("Ricette Atelier Liquidi", &["json"]);
    }
    if let Ok(dir) = app.path().document_dir() {
        dialog = dialog.set_directory(dir);
    }
    let Some(target) = dialog.blocking_save_file() else { return Ok(None) };
    let path = target.into_path().map_err(|e| format!("percorso non valido: {e}"))?;
    fs::write(&path, text.as_bytes()).map_err(|e| format!("impossibile scrivere {}: {e}", path.display()))?;
    Ok(Some(path.display().to_string()))
}

fn main() {
    tauri::Builder::default()
        .plugin(tauri_plugin_dialog::init())
        .invoke_handler(tauri::generate_handler![
            archive_location,
            load_archive,
            save_archive,
            choose_archive_folder,
            export_file
        ])
        .run(tauri::generate_context!())
        .expect("errore all'avvio di Atelier Liquidi");
}
