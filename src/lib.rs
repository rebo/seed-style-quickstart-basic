#![feature(track_caller)]
use seed::prelude::*;
use seed::*;

use seed_hooks::*;
use seed_style::measures::{pc, px};
use seed_style::*;

mod app_theme;
use app_theme::*;

// This app shows how to use most features of a proposed styling system for Seed.
//
// It includes :
//
// Component Style - Define CSS styles for html elements.
// Theming Support -Define themes for your app so common styles can set centrally
// Layout - Declare Application layout ahead of time in a context agnostic way. (Not used in seed quickstart)

// Not all of these need to be used, all features are optional.
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

// Update optionally handles WindowResized. For performance reasons we dont want to
// re-render the app on every window resize, only if the resize takes the window into new breakpoint
// this step could be completely left off and just added in at the end of a design once all breakpoints have been
// defined.
fn update(msg: Msg, _model: &mut Model, orders: &mut impl Orders<Msg>) {
    match msg {
        Msg::WindowResized => {
            // We just need to provide a copy of the theme that is providing the breakpoints.
            // it is passed in a block because we only need it on first assignment.
            seed_style::conditionally_skip_rendering::<app_theme::Breakpoint, _, _, _>(
                || app_theme::theme(),
                orders,
            )
        }
    }
}

// We subscribe to a window resize event in the init in order to handle window resizing
fn init(_url: Url, orders: &mut impl Orders<Msg>) -> Model {
    // Global style resets above normalize.css
    // Mostly box-sizing and global font.
    // .activate_init_styles should be called only once
    GlobalStyle::default()
        // ensure all buttons are centred by default and outline free
        .style("button", s().text_align_center().outline_style_none())
        //ensure box-sizing: border-box() for everything
        .style("html", s().box_sizing_border_box())
        .style("img", s().box_sizing_content_box())
        .style("*, *:before, *:after", s().box_sizing("inherit"))
        .activate_init_styles();

    // Setup a stream of Resize events.
    orders.stream(streams::window_event(Ev::Resize, |_| Msg::WindowResized));

    Model {}
}

// Default app start...
#[wasm_bindgen(start)]
pub fn start() {
    App::start("app", init, update, view);
}

//  View Entry Here, Sets up theme access, two themes are allowed access
//
//  The first is the app defined theme, the second provides access to seed style presets.
//  (At present `Theme` is not Clone therefore need to pass in as owned Vec)  to be improved in future.
//
//  ---------------
#[topo::nested] // Needed for Seed Hooks
pub fn view(model: &Model) -> Node<Msg> {
    // ensure our app is themed, include style presets theme.
    use_themes(
        || vec![app_theme::theme(), seed_style::style_presets()],
        || themed_view(model),
    )
}

pub fn themed_view(_model: &Model) -> Node<Msg> {
    let counter = use_state(|| 0); // A Seed Hook counter
    div![
        s().display_flex()
            .flex_direction_row()
            .justify_content_center()
            .align_items_center()
            .height(pc(100)),
        div![
            s().background_color(seed_colors::Gray::No2)
                .margin_x(px(80))
                .radius(px(4))
                .min_height(px(200))
                .display_flex()
                .flex_direction_column()
                .padding(&[px(24), px(32), px(48)]), // arrays passed to style properties associate values with breakpoints, so this is 24 pixels the smallest breakpoint and 32px at the next etc.
            h1![
                s().border_bottom_width(px(4))
                    .border_bottom_style_solid()
                    .justify_self_center()
                    .border_bottom_color(Color::Primary) // Color Primary is a theme defined color
                    .font_weight_v700()
                    .font_size(&[px(18), px(24), px(32)]),
                "Seed Rocks!"
            ],
            p![
                s().font_size(&[px(12), px(14), px(18)]),
                s().media("@media only screen and (max-width: 700px)") // example using media queries directly
                    .color(seed_colors::Red::No6),
                "The contents of this div are reactive, try shrinking the browser."
            ],
            p![
                s().font_size(&[px(12), px(14), px(18)]),
                "Also try clicking this button! You have clicked it ",
                counter,
                " time(s)"
            ],
            button![
                s().bg_color(Color::MutedPrimary)
                    .radius(px(5))
                    .my(px(20))
                    .align_self_center()
                    .padding_x(&[px(18), px(24)])
                    .padding_y(&[px(8), px(12)]),
                s().hover().bg_color(Color::Primary),
                s().active().bg_color(Color::DarkPrimary),
                "Click Me",
                counter.on_click(|c| *c += 1) // Example of seed hooks usage incrementing a counter.
            ],
            only_and_above(Breakpoint::Medium, || {
                // conditional rendering with `only_and_above` renders the block if at Medium Breakpoint and above.
                let hidden_counter = use_state(|| 0);
                div![
                    s().display_flex().flex_direction_column(),
                p![
                "This is only rendered at larger breakpoints. This is not display: hidden; it is actually conditional rendering. Notice it keeps its count even when hidden!"],
                button![
                    s().bg_color(hsl(20,70,50))
                    .radius(px(5))
                    .my(px(20))
                    .align_self_center()
                    .padding_x(&[px(18), px(24)])
                    .padding_y(&[px(8), px(12)]),
                s().hover().bg_color(hsl(20,80,70)),
                s().active().bg_color(hsl(20,50,30)),
                "Clicked ", hidden_counter, " times",
                hidden_counter.on_click(|c| *c+=1)
                ]
            ]
            })
        ]
    ]
}
