use std::sync::OnceLock;

use mac_oui::Oui;

// OnceLock lets us load the DB once and cache the results for future calls
pub fn oui_db() -> &'static Result<Oui, String> {
    static OUI: OnceLock<Result<Oui, String>> = OnceLock::new();

    OUI.get_or_init(Oui::default)
}
