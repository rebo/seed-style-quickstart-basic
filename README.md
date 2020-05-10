# Seed Styling Example App

**Intro**

**You need to be using recent nightly! i.e. at least from 4th April 2020**

A base app for a proposed Seed Style System.

This is very much alpha and there are quite a few missing methods or properties such as `active()` (just need to add them!).

Simply 

```
use seed_hooks::*;
use seed_hooks::style::*;
```

And begin to define styles like this: 

```rust
div![ s().background_color("red").padding("8px").font_size("20px"), "Hello"]
```

Navigating to root wil demonstrate the following ways of using the style system:

```
unstyled_counter(),
basic_styled_counter(),
css_styled_counter(),
conveinience_styled_counter(),
tw_styled_counter(),
hover_counter(),
media_query_counter(),
variant_counter(),
themed_counter(),
styles_on_args_counter(s().bg_indigo_600().font_size("20px")),
```


**To get started:**

- Clone this repo: `git clone https://github.com/seed-rs/seed-quickstart.git`

- If you don't have Rust and cargo-make installed, [Download it](https://www.rust-lang.org/tools/install), and run the following commands:

`rustup update`

`rustup target add wasm32-unknown-unknown`

`cargo install cargo-make`

Run `cargo make build` in a terminal to build the app, and `cargo make serve` to start a dev server
on `127.0.0.1:8000`.

If you'd like the compiler automatically check for changes, recompiling as
needed, run `cargo make watch` instead of `cargo make build`.

**Deploy**

1. Run `cargo make build release`
2. Upload `index.html` and `pkg` to your web server

---

New Rust-only quickstart in development! => [Seeder](https://github.com/MartinKavik/seeder)
