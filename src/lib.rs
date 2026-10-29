{%- capture project_name_pascal -%}{%- assign parts = crate_name | split: "_" -%}{%- for p in parts -%}{{ p | capitalize }}{%- endfor -%}{%- endcapture -%}
{% if style == "xaml" -%}
//! {{description}}
//!
//! A FerroUI application whose interface is written in XAML and compiled to Rust source by
//! the build script of the crate (`build.rs`): `App.xaml` (the application, with its theme)
//! and `MainWindow.xaml` (a window with "Hello, World!" in its middle), each of which
//! populates the class it names.

// The compiled markup of a document names the types of this crate by the name of the crate.
extern crate self as {{crate_name}};

mod app;
mod main_window;
mod register_types;

// One module per compiled document and `compiled_markup` (the loader table of the compiled
// markup of the crate and `register()`), which the build script of the crate generates.
ferroui_markup_xaml::include_compiled_xaml!();

pub use app::App;
pub use main_window::MainWindow;
pub use register_types::{register_types, ASSEMBLY};

#[cfg(test)]
mod tests {
    use crate::register_types::ASSEMBLY;

    /// What the crate states about itself for markup.
    #[test]
    fn assembly_states_the_crate() {
        assert_eq!(ASSEMBLY.crate_name, "{{crate_name}}");
        assert_eq!(ASSEMBLY.name, "{{project_name_pascal}}");
    }
}
{% elsif style == "rsx" -%}
//! {{description}}
//!
//! A FerroUI application written with the `rsx!` macro and `#[component]` of `ferroui-view`
//! (docs/declarative-ui.md): the text and a button that counts its clicks, the count a
//! signal (section 6) that the text of the button follows.

use ferroui_base::layout::{HorizontalAlignment, VerticalAlignment};
use ferroui_view::prelude::*;

/// The content of the window.
#[component]
pub fn App() -> Element {
    let clicks = signal(0);
    rsx! {
        StackPanel {
            horizontal_alignment: HorizontalAlignment::Center,
            vertical_alignment: VerticalAlignment::Center,
            spacing: 12.0,
            TextBlock { text: "Hello, World!", font_size: 28.0 }
            Button {
                horizontal_alignment: HorizontalAlignment::Center,
                on_click: move |_| clicks.update(|c| *c += 1),
                TextBlock { text: move || clicked_text(clicks.get()) }
            }
        }
    }
}

/// The label of the button after `clicks` clicks.
fn clicked_text(clicks: i32) -> String {
    match clicks {
        1 => "Clicked 1 time".to_string(),
        _ => format!("Clicked {clicks} times"),
    }
}

#[cfg(test)]
mod tests {
    use super::clicked_text;

    #[test]
    fn one_click_reads_singular() {
        assert_eq!(clicked_text(1), "Clicked 1 time");
    }

    #[test]
    fn any_other_count_reads_plural() {
        assert_eq!(clicked_text(0), "Clicked 0 times");
        assert_eq!(clicked_text(42), "Clicked 42 times");
    }
}
{% else -%}
//! {{description}}
//!
//! A FerroUI application written with the builders of `ferroui-view`
//! (docs/declarative-ui.md, section 4): the text and a button that counts its clicks, the
//! count a signal (section 6) that the text of the button follows.

use ferroui_base::layout::{HorizontalAlignment, VerticalAlignment};
use ferroui_view::prelude::*;

/// The content of the window.
pub fn app() -> Element {
    let clicks = signal(0);
    stack_panel()
        .horizontal_alignment(HorizontalAlignment::Center)
        .vertical_alignment(VerticalAlignment::Center)
        .spacing(12.0)
        .child(text_block().text("Hello, World!").font_size(28.0))
        .child(
            button()
                .horizontal_alignment(HorizontalAlignment::Center)
                .on_click(move |_| clicks.update(|c| *c += 1))
                .child(text_block().text(move || clicked_text(clicks.get()))),
        )
        .into()
}

/// The label of the button after `clicks` clicks.
fn clicked_text(clicks: i32) -> String {
    match clicks {
        1 => "Clicked 1 time".to_string(),
        _ => format!("Clicked {clicks} times"),
    }
}

#[cfg(test)]
mod tests {
    use super::clicked_text;

    #[test]
    fn one_click_reads_singular() {
        assert_eq!(clicked_text(1), "Clicked 1 time");
    }

    #[test]
    fn any_other_count_reads_plural() {
        assert_eq!(clicked_text(0), "Clicked 0 times");
        assert_eq!(clicked_text(42), "Clicked 42 times");
    }
}
{% endif -%}
