#![feature(track_caller)]
use seed::prelude::*;
use seed::*;

use seed_hooks::*;

use seed_style::px;
use seed_style::*;

mod global_styles;
mod theme;

use theme::*;

//
//  Model, Msg, Update, init(), and start()
//  ---------------------------------------

pub struct Model {}

// In aps that make use of conditional rendering on breakpoints we We just need one Msg
// in order to handle a WindowResized event.
#[derive(Clone)]
pub enum Msg {
    WindowResized,
}

fn update(_msg: Msg, _model: &mut Model, _orders: &mut impl Orders<Msg>) {}

fn init(_url: Url, _orders: &mut impl Orders<Msg>) -> Model {
    // CSS style reset on app launch.
    global_styles::global_init();
    Model {}
}

#[wasm_bindgen(start)]
pub fn start() {
    App::start("app", init, update, view);
}

//  View Entry Here, Sets up theme access, two themes are allowed access
//
//  The first is the app defined theme, the second provides access to seed style presets.
//  The content block also activates themed global_styles.
//  ---------------
#[topo::nested] // Needed for Seed Hooks
pub fn view(model: &Model) -> Node<Msg> {
    // ensure our app is themed, include style presets theme.
    use_themes(
        || vec![theme::theme(), seed_style::style_presets()],
        || {
            global_styles::use_themed_styles();
            themed_view(model)
        },
    )
}

//
// Content from themed_view and below is fully themeable:
//
pub fn themed_view(_model: &Model) -> Node<Msg> {
    div![
        h1![
            s().font_size(px(32))
                .background_color(Color::MutedPrimary)
                .border_left_width(px(8))
                .border_left_style_solid()
                .border_color(Color::Primary)
                .custom_style(MyStyles::HeaderText)
                .padding(px(24))
                .border_radius(px(3))
                .margin_x(px(24))
                .margin_y(px(12)),
            "Welcome to Seed Style"
        ],
        div![
            s().column_count(CssColumnCount::Number(3))
                .custom_style(MyStyles::BodyText)
                .column_width(px(250))
                .padding_x(px(24))
                .padding_y(px(12)),
            (0..8).map(|_| p![
                        s().p(px(3)).my(px(12)),
                        s().hover()
                            .background_color(Color::MutedSecondary)
                            .radius(px(4)),
                        r#"
                Lorem ipsum dolor sit amet, consectetur adipiscing elit, sed do eiusmod tempor incididunt ut labore et dolore magna aliqua. 
                Ut enim ad minim veniam, quis nostrud exercitation ullamco laboris nisi ut aliquip ex ea commodo consequat. Duis aute irure dolor 
                in reprehenderit in voluptate velit esse cillum dolore eu fugiat nulla pariatur. Excepteur sint occaecat cupidatat non proident, 
                sunt in culpa qui officia deserunt mollit anim id est laborum"#
                ]
            ),
        ]
    ]
}
