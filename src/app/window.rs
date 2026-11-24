//! Main application window: a sidebar of panels and a stack of pages.

use std::rc::Rc;

use glib::clone;
use gtk4::prelude::*;
use libadwaita as adw;
use libadwaita::prelude::*;


pub struct MainWindow {
    inner: adw::ApplicationWindow,
}

impl MainWindow {
    pub fn new(app: &adw::Application) -> Self {
        let panels = Rc::new(crate::panels::all());

        // ----- sidebar -----------------------------------------------------
        let list = gtk4::ListBox::builder()
            .selection_mode(gtk4::SelectionMode::Single)
            .build();
        list.add_css_class("navigation-sidebar");

        for panel in panels.iter() {
            let row = adw::ActionRow::builder().title(panel.title).build();
            if let Some(icon) = panel.icon {
                row.add_prefix(&gtk4::Image::from_icon_name(icon));
            }
            let list_row = gtk4::ListBoxRow::builder().child(&row).build();
            list_row.set_widget_name(panel.id);
            list.append(&list_row);
        }

        let sidebar_scroll = gtk4::ScrolledWindow::builder()
            .child(&list)
            .vexpand(true)
            .build();

        let sidebar_header = adw::HeaderBar::new();
        sidebar_header.set_title_widget(Some(&adw::WindowTitle::new("TweaksPlus", "")));
        sidebar_header.pack_end(&gtk4::MenuButton::builder()
            .icon_name("open-menu-symbolic")
            .menu_model(&app.menubar().unwrap_or_else(|| gio::Menu::new().upcast()))
            .tooltip_text("Main menu")
            .build());

        let sidebar_toolbar = adw::ToolbarView::new();
        sidebar_toolbar.add_top_bar(&sidebar_header);
        sidebar_toolbar.set_content(Some(&sidebar_scroll));
        let sidebar_page = adw::NavigationPage::builder()
            .title("TweaksPlus")
            .child(&sidebar_toolbar)
            .build();

        // ----- content -----------------------------------------------------
        let stack = gtk4::Stack::builder()
            .transition_type(gtk4::StackTransitionType::Crossfade)
            .build();
        for panel in panels.iter() {
            stack.add_named(&panel.page, Some(panel.id));
        }

        let title = adw::WindowTitle::new("", "");
        let search_entry = gtk4::SearchEntry::builder()
            .placeholder_text("Search Tweaks…")
            .width_chars(24)
            .build();
        let search_button = gtk4::ToggleButton::builder()
            .icon_name("edit-find-symbolic")
            .tooltip_text("Search")
            .valign(gtk4::Align::Center)
            .build();
        let search_bar = gtk4::SearchBar::builder()
            .child(&search_entry)
            .show_close_button(true)
            .build();
        search_button
            .bind_property("active", &search_bar, "search-mode-enabled")
            .flags(glib::BindingFlags::BIDIRECTIONAL)
            .build();

        let content_header = adw::HeaderBar::new();
        content_header.set_title_widget(Some(&title));
        content_header.pack_start(&search_button);

        let content_toolbar = adw::ToolbarView::new();
        content_toolbar.add_top_bar(&content_header);
        content_toolbar.add_bottom_bar(&search_bar);
        content_toolbar.set_content(Some(&stack));
        let content_page = adw::NavigationPage::builder()
            .title("Tweaks")
            .child(&content_toolbar)
            .build();

        let split_view = adw::NavigationSplitView::builder()
            .sidebar(&sidebar_page)
            .content(&content_page)
            .build();

        let window = adw::ApplicationWindow::builder()
            .application(app)
            .title("TweaksPlus")
            .default_width(1000)
            .default_height(700)
            .content(&split_view)
            .build();

        // ----- behaviour ---------------------------------------------------
        let titles: Rc<Vec<(String, String)>> = Rc::new(
            panels
                .iter()
                .map(|panel| {
                    let mut words = vec![panel.title.to_lowercase(), panel.description.to_lowercase()];
                    words.extend(panel.keywords.iter().map(|word| word.to_lowercase()));
                    (panel.id.to_string(), words.join(" "))
                })
                .collect(),
        );

        list.set_filter_func(clone!(#[weak] search_entry, #[strong] titles, #[upgrade_or] false, move |row| {
            let query = search_entry.text().trim().to_lowercase();
            if query.is_empty() {
                return true;
            }
            titles
                .iter()
                .find(|(id, _)| id == &row.widget_name().as_str())
                .map(|(_, words)| words.contains(&query))
                .unwrap_or(false)
        }));

        search_entry.connect_search_changed(clone!(#[weak] list, move |_| {
            list.invalidate_filter();
        }));

        list.connect_row_selected(clone!(
            #[weak] stack,
            #[weak] title,
            #[weak] split_view,
            #[strong] panels,
            move |_, row| {
                let Some(row) = row else { return };
                let id = row.widget_name();
                stack.set_visible_child_name(&id);
                if let Some(panel) = panels.iter().find(|panel| panel.id == id.as_str()) {
                    title.set_title(panel.title);
                    title.set_subtitle(panel.description);
                }
                split_view.set_show_content(true);
            }
        ));

        if let Some(first) = list.row_at_index(0) {
            list.select_row(Some(&first));
        }

        // Ctrl+F focuses search, like GNOME Tweaks.
        if let (Some(trigger), Some(action)) = (
            gtk4::ShortcutTrigger::parse_string("<primary>f"),
            Some(gtk4::CallbackAction::new(clone!(
                #[weak] search_button,
                #[weak] search_entry,
                #[upgrade_or] glib::Propagation::Stop,
                move |_, _| {
                    search_button.set_active(true);
                    search_entry.grab_focus();
                    glib::Propagation::Stop
                }
            ))),
        ) {
            let controller = gtk4::ShortcutController::new();
            controller.add_shortcut(
                gtk4::Shortcut::builder()
                    .trigger(&trigger)
                    .action(&action)
                    .build(),
            );
            window.add_controller(controller);
        }

        Self { inner: window }
    }

    pub fn present(&self) {
        self.inner.present();
    }
}
