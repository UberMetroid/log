use yew::prelude::*;
use gloo_storage::{LocalStorage, Storage};

#[derive(Clone, PartialEq)]
pub struct LocaleContext {
    pub current: String,
    pub on_change: Callback<String>,
}

impl LocaleContext {
    pub fn t(&self, key: &str) -> String {
        translate(&self.current, key)
    }
}

pub fn detect_browser_locale() -> String {
    if let Some(window) = web_sys::window() {
        let navigator = window.navigator();
        if let Some(lang) = navigator.language() {
            if lang.starts_with("es") {
                return "es".to_string();
            }
        }
    }
    "en".to_string()
}

pub fn get_saved_locale() -> String {
    LocalStorage::get("rustpad_locale").unwrap_or_else(|_| detect_browser_locale())
}

pub fn set_saved_locale(locale: &str) {
    let _ = LocalStorage::set("rustpad_locale", locale);
}

pub fn translate(lang: &str, key: &str) -> String {
    let l = if lang.starts_with("es") { "es" } else { "en" };
    match (l, key) {
        // App header / general
        ("en", "search_btn") => "Search (Ctrl+K)".into(),
        ("es", "search_btn") => "Buscar (Ctrl+K)".into(),
        ("en", "new_pad") => "New Pad".into(),
        ("es", "new_pad") => "Nuevo Bloc".into(),
        ("en", "rename") => "Rename".into(),
        ("es", "rename") => "Renombrar".into(),
        ("en", "delete") => "Delete".into(),
        ("es", "delete") => "Eliminar".into(),
        ("en", "shortcuts") => "Shortcuts (?)".into(),
        ("es", "shortcuts") => "Atajos (?)".into(),
        ("en", "settings") => "Settings".into(),
        ("es", "settings") => "Ajustes".into(),
        ("en", "logout") => "Logout".into(),
        ("es", "logout") => "Cerrar sesión".into(),
        ("en", "online") => "online".into(),
        ("es", "online") => "en línea".into(),
        
        // Editor
        ("en", "saving") => "Saving...".into(),
        ("es", "saving") => "Guardando...".into(),
        ("en", "saved") => "Saved".into(),
        ("es", "saved") => "Guardado".into(),
        ("en", "copied") => "Copied!".into(),
        ("es", "copied") => "¡Copiado!".into(),
        ("en", "copy") => "Copy".into(),
        ("es", "copy") => "Copiar".into(),
        ("en", "export") => "Export".into(),
        ("es", "export") => "Exportar".into(),
        ("en", "unsaved_changes") => "Unsaved changes".into(),
        ("es", "unsaved_changes") => "Cambios no guardados".into(),
        ("en", "offline") => "Offline".into(),
        ("es", "offline") => "Sin conexión".into(),
        ("en", "placeholder") => "Start typing your markdown here...".into(),
        ("es", "placeholder") => "Empieza a escribir tu markdown aquí...".into(),
        
        // Settings modal
        ("en", "settings_title") => "Settings".into(),
        ("es", "settings_title") => "Ajustes".into(),
        ("en", "settings_preview") => "Default Preview Mode:".into(),
        ("es", "settings_preview") => "Modo de Vista Previa:".into(),
        ("en", "settings_save_interval") => "Status Message Keep Time (ms):".into(),
        ("es", "settings_save_interval") => "Tiempo de mensaje de estado (ms):".into(),
        ("en", "settings_disable_print") => "Disable auto-expand in Print:".into(),
        ("es", "settings_disable_print") => "Desactivar auto-expansión al imprimir:".into(),
        ("en", "settings_lang") => "App Language:".into(),
        ("es", "settings_lang") => "Idioma de la aplicación:".into(),
        ("en", "settings_save") => "Save Settings".into(),
        ("es", "settings_save") => "Guardar Ajustes".into(),
        
        // Search modal
        ("en", "search_title") => "Fuzzy Search Notepads".into(),
        ("es", "search_title") => "Búsqueda de blocs de notas".into(),
        ("en", "search_placeholder") => "Type title or content to search...".into(),
        ("es", "search_placeholder") => "Escribe título o contenido para buscar...".into(),
        ("en", "search_no_results") => "No matching notepads found.".into(),
        ("es", "search_no_results") => "No se encontraron blocs de notas.".into(),
        
        // Login
        ("en", "login_title") => "RustPad".into(),
        ("es", "login_title") => "RustPad".into(),
        ("en", "login_locked") => "Locked out. Try again in 15 minutes.".into(),
        ("es", "login_locked") => "Bloqueado. Vuelve a intentarlo en 15 minutos.".into(),
        ("en", "login_prompt") => "Enter authentication PIN to access your notes".into(),
        ("es", "login_prompt") => "Introduce el PIN de autenticación para acceder".into(),
        ("en", "login_btn") => "Unlock".into(),
        ("es", "login_btn") => "Desbloquear".into(),
        
        // Shortcuts keys
        ("en", "sc_search") => "Search Notepads".into(),
        ("es", "sc_search") => "Buscar Blocs".into(),
        ("en", "sc_save") => "Manual Save".into(),
        ("es", "sc_save") => "Guardado Manual".into(),
        ("en", "sc_preview") => "Toggle Preview Mode".into(),
        ("es", "sc_preview") => "Cambiar Vista Previa".into(),
        ("en", "sc_new") => "New Notepad".into(),
        ("es", "sc_new") => "Nuevo Bloc".into(),
        ("en", "sc_help") => "Shortcuts Help".into(),
        ("es", "sc_help") => "Ayuda de Atajos".into(),
        ("en", "close") => "Close".into(),
        ("es", "close") => "Cerrar".into(),

        // Rename modal
        ("en", "rename_title") => "Rename Notepad".into(),
        ("es", "rename_title") => "Renombrar Bloc de Notas".into(),
        ("en", "rename_confirm") => "Rename".into(),
        ("es", "rename_confirm") => "Renombrar".into(),
        ("en", "cancel") => "Cancel".into(),
        ("es", "cancel") => "Cancelar".into(),
        ("en", "reset") => "Reset".into(),
        ("es", "reset") => "Restablecer".into(),
        
        // Delete modal
        ("en", "delete_title") => "Delete Notepad".into(),
        ("es", "delete_title") => "Eliminar Bloc de Notas".into(),
        ("en", "delete_msg") => "Are you sure you want to delete this notepad? This action cannot be undone.".into(),
        ("es", "delete_msg") => "¿Estás seguro de que quieres eliminar este bloc? Esta acción no se puede deshacer.".into(),
        ("en", "delete_confirm") => "Delete".into(),
        ("es", "delete_confirm") => "Eliminar".into(),
        
        // Shortcuts modal
        ("en", "shortcuts_title") => "Keyboard Shortcuts".into(),
        ("es", "shortcuts_title") => "Atajos de Teclado".into(),
        
        // Preview modes
        ("en", "prev_editor") => "Editor".into(),
        ("es", "prev_editor") => "Editor".into(),
        ("en", "prev_split") => "Split".into(),
        ("es", "prev_split") => "Dividido".into(),
        ("en", "prev_preview") => "Preview".into(),
        ("es", "prev_preview") => "Vista Previa".into(),
        
        // Toolbar tooltips
        ("en", "tb_bold") => "Bold".into(),
        ("es", "tb_bold") => "Negrita".into(),
        ("en", "tb_italic") => "Italic".into(),
        ("es", "tb_italic") => "Cursiva".into(),
        ("en", "tb_heading") => "Heading".into(),
        ("es", "tb_heading") => "Encabezado".into(),
        ("en", "tb_link") => "Link".into(),
        ("es", "tb_link") => "Enlace".into(),
        ("en", "tb_code") => "Code Block".into(),
        ("es", "tb_code") => "Bloque de código".into(),
        ("en", "tb_list") => "List".into(),
        ("es", "tb_list") => "Lista".into(),

        // Default or unmapped
        (_, k) => k.to_string(),
    }
}
