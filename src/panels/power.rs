//! Power panel: lid and power button actions, screen blanking and dimming.

use crate::{panel::Panel, settings::rows};
use libadwaita as adw;
use libadwaita::prelude::*;

const POWER: &str = "org.gnome.settings-daemon.plugins.power";

const LID_ACTIONS: &[(&str, &str)] = &[
    ("blank", "Blank screen"),
    ("suspend", "Suspend"),
    ("shutdown", "Power off"),
    ("hibernate", "Hibernate"),
    ("interactive", "Ask"),
    ("nothing", "Do nothing"),
    ("logout", "Log out"),
];

const BUTTON_ACTIONS: &[(&str, &str)] = &[
    ("nothing", "Do nothing"),
    ("suspend", "Suspend"),
    ("hibernate", "Hibernate"),
    ("interactive", "Ask"),
];

pub fn build() -> Panel {
    let page = adw::PreferencesPage::builder().title("Power").build();

    // Older GNOME releases (including Ubuntu 26.04) expose dedicated lid-close
    // keys. Newer rolling releases such as CachyOS GNOME 51 removed them and
    // use the equivalent idle-action keys, which share the same enum.
    let native_lid_keys = crate::settings::key_available(POWER, "lid-close-ac-action")
        && crate::settings::key_available(POWER, "lid-close-battery-action");
    let (ac_key, battery_key) = if native_lid_keys {
        ("lid-close-ac-action", "lid-close-battery-action")
    } else {
        ("sleep-inactive-ac-type", "sleep-inactive-battery-type")
    };

    let lid = adw::PreferencesGroup::builder()
        .title(if native_lid_keys {
            "Lid Close Action"
        } else {
            "Sleep Action"
        })
        .build();
    if !native_lid_keys {
        lid.set_description(Some(
            "Lid-close actions are managed by the system in this GNOME release",
        ));
    }
    lid.add(&rows::combo(
        POWER,
        ac_key,
        "When Plugged In",
        None,
        LID_ACTIONS,
    ));
    lid.add(&rows::combo(
        POWER,
        battery_key,
        "On Battery",
        None,
        LID_ACTIONS,
    ));

    let button = adw::PreferencesGroup::builder().build();
    button.add(&rows::combo(
        POWER,
        "power-button-action",
        "Power Button Action",
        None,
        BUTTON_ACTIONS,
    ));

    let screen = adw::PreferencesGroup::builder().title("Screen").build();
    screen.add(&rows::spin_uint(
        "org.gnome.desktop.session",
        "idle-delay",
        "Blank Screen Delay",
        Some("Seconds of inactivity before the screen blanks; 0 disables it"),
        0,
        36000,
        10,
    ));
    screen.add(&rows::switch(
        POWER,
        "idle-dim",
        "Dim Screen When Inactive",
        None,
    ));
    screen.add(&rows::spin_int(
        POWER,
        "idle-brightness",
        "Idle Brightness",
        Some("Brightness while the screen is dimmed"),
        0,
        100,
        1,
    ));
    screen.add(&rows::switch(
        POWER,
        "power-saver-profile-on-low-battery",
        "Enable Power Saver on Low Battery",
        None,
    ));

    page.add(&lid);
    page.add(&button);
    page.add(&screen);

    Panel {
        id: "power",
        title: "Power",
        description: "Lid, power button and screen blanking",
        icon: Some("battery-full-charged-symbolic"),
        keywords: &[
            "power", "battery", "lid", "suspend", "hibernate", "shutdown", "screen", "blank",
            "dim", "idle",
        ],
        page,
    }
}
