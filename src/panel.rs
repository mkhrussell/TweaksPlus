//! Panel descriptor shared by the sidebar and the settings stack.

use libadwaita as adw;

/// One page in the TweaksPlus sidebar.
pub struct Panel {
    pub id: &'static str,
    pub title: &'static str,
    pub description: &'static str,
    pub icon: Option<&'static str>,
    /// Extra words that should find this panel in the search box.
    pub keywords: &'static [&'static str],
    pub page: adw::PreferencesPage,
}
