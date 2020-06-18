use crate::theme::Color;
use seed_style::*;

pub fn global_init() {
    // make everything border-box
    GlobalStyle::default()
        .style(
            "a,ul,li,div,p,h1,h2,h3,h4,li,dd,dt,button,label,input",
            s().font_family("'Lato',sans-serif")
                .webkit_font_smoothing_antialiased(),
        )
        .style("img", s().box_sizing_content_box())
        .style("*, *:before, *:after", s().box_sizing("inherit"))
        .activate_init_styles();
}

pub fn use_themed_styles() {
    GlobalStyle::default()
        .style("body", s().background_color(Color::Background))
        .activate_styles();
}
