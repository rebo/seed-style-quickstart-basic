# Seed Styling Quickstart

**Intro**

**Note : You need to be using nightly**

This is a quickstart Seed app demonstrating the final api surface for Seed Style.

Seed Style has many, features including optional typed CSS properties, breakpoint-conditional rendering, full layout system including composition and first class layouts, integrated theme support, style reuse, media queries, animation and keyframe support, convienience methods and global styling support.

This quickstart just demonstrates a few of the features.

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
