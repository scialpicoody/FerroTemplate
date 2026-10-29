{%- capture project_name_pascal -%}{%- assign parts = project_name | split: "-" -%}{%- for p in parts -%}{{ p | capitalize }}{%- endfor -%}{%- endcapture -%}
{%- capture theme_type -%}{% if theme == "winui" %}WinUITheme{% elsif theme == "fluent" %}FluentTheme{% elsif theme == "simple" %}SimpleTheme{% elsif theme == "material" %}MaterialTheme{% else %}MacOSTheme{% endif %}{%- endcapture -%}
//! The class of the document `App.xaml`: the application, whose main window is the window
//! of `MainWindow.xaml`.

use crate::main_window::MainWindow;
use ferroui_base::{
    ferro_class, ferro_class_info, ferro_impl_classes, instantiate, FerroObjectImpl, Ref,
};
use ferroui_controls::{Application, ApplicationImpl, ApplicationImplExt, NewApplication};

#[repr(C)]
pub struct App {
    base: Application,
}

ferro_class!(App: Application);
ferro_class_info!(App { new: App::new });
ferro_impl_classes!(App: FerroObjectImpl);

impl NewApplication for App {
    fn new_application() -> Ref<Self> {
        Self::new()
    }
}

impl ApplicationImpl for App {
    fn initialize(this: &Self) {
        crate::register_types();
{% if theme == "material" or theme == "macos" %}
        // The theme is added in code: the theme crate does not declare it as a markup element.
        this.styles().add(ferroui_themes_{{theme}}::{{theme_type}}::new().as_style());
{% endif %}
        this.initialize_component();
    }

    fn on_framework_initialization_completed(this: &Self) {
        let lifetime = this.application_lifetime();
        if let Some(desktop) = lifetime
            .as_ref()
            .and_then(|l| l.as_classic_desktop_style_application_lifetime())
        {
            desktop.set_main_window(Some(MainWindow::new().upcast()));
        }
        Self::parent_on_framework_initialization_completed(this);
    }
}

impl App {
    pub fn construct() -> Self {
        Self {
            base: Application::construct(),
        }
    }

    pub fn new() -> Ref<Self> {
        instantiate(Self::construct())
    }

    /// `InitializeComponent()`: populates the application from its document.
    fn initialize_component(&self) {
        if let Err(error) = crate::compiled_app::populate(None, &self.to_ref()) {
            panic!("/App.xaml: {error}");
        }
    }
}
