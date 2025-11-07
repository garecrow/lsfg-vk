use adw;
use gtk;
use gtk::glib;

pub mod dropdown;
pub mod entry;
pub mod number;
pub mod slider;
pub mod switch;

glib::wrapper! {
    pub struct PrefDropdown(ObjectSubclass<dropdown::PrefDropdown>)
        @extends
            adw::ComboRow, adw::ActionRow, adw::PreferencesRow, gtk::ListBoxRow, gtk::Widget,
        @implements
            gtk::Accessible, gtk::Actionable, gtk::Buildable, gtk::ConstraintTarget;
}

glib::wrapper! {
    pub struct PrefSwitch(ObjectSubclass<switch::PrefSwitch>)
        @extends
            adw::SwitchRow, adw::ActionRow, adw::PreferencesRow, gtk::ListBoxRow, gtk::Widget,
        @implements
            gtk::Accessible, gtk::Actionable, gtk::Buildable, gtk::ConstraintTarget;
}

glib::wrapper! {
    pub struct PrefNumber(ObjectSubclass<number::PrefNumber>)
        @extends
            adw::SpinRow, adw::ActionRow, adw::PreferencesRow, gtk::ListBoxRow, gtk::Widget,
        @implements
            gtk::Accessible, gtk::Actionable, gtk::Buildable, gtk::ConstraintTarget, gtk::Editable;
}

glib::wrapper! {
    pub struct PrefSlider(ObjectSubclass<slider::PrefSlider>)
        @extends
            adw::ActionRow, adw::PreferencesRow, gtk::ListBoxRow, gtk::Widget,
        @implements
            gtk::Accessible, gtk::Actionable, gtk::Buildable, gtk::ConstraintTarget;
}

glib::wrapper! {
    pub struct PrefEntry(ObjectSubclass<entry::PrefEntry>)
        @extends
            adw::EntryRow, adw::PreferencesRow, gtk::ListBoxRow, gtk::Widget,
        @implements
            gtk::Accessible, gtk::Actionable, gtk::Buildable, gtk::ConstraintTarget, gtk::Editable;
}

impl PrefDropdown {
    pub fn new() -> Self {
        glib::Object::new()
    }
}

impl PrefSwitch {
    pub fn new() -> Self {
        glib::Object::new()
    }
}

impl PrefNumber {
    pub fn new() -> Self {
        glib::Object::new()
    }
}

impl PrefSlider {
    pub fn new() -> Self {
        glib::Object::new()
    }
}

impl PrefEntry {
    pub fn new() -> Self {
        glib::Object::new()
    }
}

impl PrefEntry {
    pub fn browse_button(&self) -> gtk::Button {
        self.imp().btn.get()
    }
}
