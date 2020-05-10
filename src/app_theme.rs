use seed_style::measures::px;
use seed_style::*;
// Theme Definition
// -----------------
//
// A Theme Object is where all css related values and presets can be stored
// and then accessed at any point in the view.
//
// The Theme Object is broadly consistent with the Theme specification that is used in
// several css librarys: https://theme-ui.com/theme-spec/
//
// A Theme object is made up of named css values called aliases
// as well as scales for css values.
//
// Having a scale is useful for things like sizes and spacing
// because you can have consistent layout throughout your app.  For instance pixel gaps
// at 4px increments.
//
// Having named aliases for things like colors is useful because it means
// swapping out colors, or having a dark/light theme can be defined in a central location.
//
// In order to use cssvalue aliases we use an enum.
//
// // Main Color Theme Keys
#[derive(Hash, PartialEq, Eq, Clone)]
pub enum Color {
    Background,
    MainText,
    Primary,
    MutedPrimary,
    DarkPrimary,
    MutedSecondary,
    Secondary,
    DarkSecondary,
    Highlight,
}
impl ColorTheme for Color {} // Allows you to use a `Color` variant as a CssColor alias in the theme.

// Named Breakpoints Keys allow you to refer to a named breakpoint in layout helpers and css media queries.
#[derive(Hash, PartialEq, Eq, Clone, Debug)]
pub enum Breakpoint {
    ExtraSmall,
    Small,
    Medium,
    Large,
    ExtraLarge,
}
impl BreakpointTheme for Breakpoint {} // Enable `Breakpoint` as a Breakpoint alias.

// The good thing about declaring the keys ahread of time is that it is easy for
// A user of the app / component to know what can be set to configure the style of the app

// WIth the keys declared, we can now actually define a theme that we want to use:
// We can have multiple themes for a single Theme definition if needed.

// The following value classes are themeable with named aliases:
//
//  BorderTheme,
//  BorderWidthTheme,
//  BorderStyleTheme,
//  SpaceTheme,
//  LineHeightTheme,
//  LetterSpacingTheme,
//  BorderRadiusTheme,
//  FontTheme,
//  FontSizeTheme,
//  SizeTheme,
//  TransitionTheme,
//  ZIndexTheme,
//  DisplayTheme,
//  ColorTheme,
//  ShadowTheme,
//  StyleTheme,
//  BreakpointTheme,

// We now write a function to provide an instance of a theme.
// A different function could provide a completely different theme
// For instance a dark mode theme.

pub fn theme() -> Theme {
    use Breakpoint::*;

    // I generally set the named aliases seperately from the theme scales:
    let theme = Theme::new()
        // Various theme colors
        .set_color(Color::Background, CssColor::Hex(0xFFFFFF))
        .set_color(Color::MainText, CssColor::Hex(0x000000))
        .set_color(Color::Primary, CssColor::Hsl(100.0, 70.0, 40.0))
        .set_color(Color::MutedPrimary, CssColor::Hsl(100.0, 50.0, 50.0))
        .set_color(Color::Secondary, CssColor::Hex(0xBBBBBB))
        .set_color(Color::MutedSecondary, CssColor::Hex(0xDDDDDD)) // or use the hsl shortcut
        .set_color(Color::Highlight, hsl(310, 70, 85))
        .set_color(Color::DarkPrimary, hsl(200, 70, 35))
        .set_color(Color::DarkSecondary, hsl(300, 60, 20))
        // set named breakpoints for conditional rendering
        .set_breakpoint(ExtraSmall, (0, Some(600))) // Breakpoints are upper bound exclusive lower bound inclusive.
        .set_breakpoint(Small, (600, Some(960)))
        .set_breakpoint(Medium, (960, Some(1280)))
        .set_breakpoint(Large, (1280, Some(1920)))
        .set_breakpoint(ExtraLarge, (1920, None));

    // other aliases you can set include :
    // color, space, size, font_size, font, border, border_width, border_style, border_radius, transition
    // line_height, letter_spacing,

    // scales
    // https://styled-system.com/guides/array-scales/
    let theme = theme
        .border_width_scale(&[px(2), px(4), px(8), px(12), px(16), px(24), px(32)])
        .space_scale(&[px(2), px(4), px(8), px(12), px(16), px(24), px(32)])
        .size_scale(&[px(1), px(2), px(4)])
        .font_size_scale(&[px(14), px(18), px(20), px(36)])
        // Other scales you can set are :
        // sizes_scale, spaces_scale, borders_scale, font_sizes_scale, border_widths_scale, border_styles_scale, radii_scale, colors_scale, shadows_scale,
        //
        // Breakpoint scales for 'in-property' setting of values that are responsive to breakpoints
        .breakpoint_scale([600, 960, 1280, 1920]); // standard-material-ui breakpoints

    // Global styles to apply to any selectors you choose.
    // When a theme is changed for a new one then that themes global styles are then used.
    theme.set_global_styles(
        GlobalStyle::default()
            .style("p", s().padding(px(24)))
            .style(
                "a,div,p,h1,h2,h3,h4,li,dd,dt,button,label,input",
                s().font_family("'Lato',sans-serif")
                    .webkit_font_smoothing_antialiased(),
            )
            .style("h1", s().mb(px(24)).padding(px(18)))
            .style(
                "a,div,p,h1,h2,h3,h4,li,dd,dt,button,label,input",
                s().font_family("Roboto,sans-serif")
                    .webkit_font_smoothing_antialiased(),
            )
            .style(
                "body",
                s().bg_color(CssColor::Hex(0xBBBBBB))
                    .color(CssColor::Hex(0x000000)),
            ),
    )
}
