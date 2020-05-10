# Seed Styling Quickstart

**Intro**

**Note : You need to be using nightly**

This is a quickstart Seed app demonstrating the final api surface for Seed Style.

Seed Style has many, features including:

* optional typed CSS properties and values
* integrated theme support
* atomic design
* full layout system including composition and first class layouts
* conditional rendering on breakpoints
* pseudo selector support
* combinator support
* media query and breakpoint support
* style composition and reuse
* animation and keyframe support
* convienience methods
* global styling support

This quickstart demonstrates just a few of these features.

**Basic Styling**

Add a style to an element by using `s()` followed by a number of property method calls. All properties are optionally typed, however you can also 
use strings for any property.  Properties with single option variants (for instance `display: grid;`) can called using a variant prefix. I.e. `display_grid()`.

Useful helper functions :

* `rgb()`, `rgba()` and `hsl()` create a `CssColor` value which can be used in any property that accepts a color value.
* `px()`, `em()`, `rem()` `cm()` can be used in any property that can accept an exact length.
* `pc()` can be used in any property that accepts a percentage.

Example:

```
div![
    s().background_color(hsl(20,70,30))
        .border_radius(px(4))
        .padding(px(8))
        .display_flex()
        .font_size(px(24)),
    "This is a styled flex div
]
```

**To get started:**

- Clone this repo: `git clone https://github.com/rebo/seed_styling_quickstart`

- If you don't have Rust and cargo-make installed, [Download it](https://www.rust-lang.org/tools/install), and run the following commands:

`rustup update`

`rustup target add wasm32-unknown-unknown`

Run `cargo make build` in a terminal to build the app, and then `cargo make serve` to start a dev server
on `127.0.0.1:8000`.

Alternatively, `cargo make start` both builds and starts a running server.

If you'd like the compiler automatically check for changes, recompiling as
needed, run `cargo make watch` instead of `cargo make build`.

**Deploy**

1. Run `cargo make build release`
2. Upload `index.html` and `pkg` to your web server

---

New Rust-only quickstart in development! => [Seeder](https://github.com/MartinKavik/seeder)
