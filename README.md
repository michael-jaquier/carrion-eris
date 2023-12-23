# Objective

This repo serves as a bit of a test bed for various libraries and design patterns.

## `carrion-patterns`

Used to store our basic patterns. Right now it only stores Finite state machine that can be used to model transitions between gamestates. After using this pattern for a while it become evident that that state transition logic was a poor fit. Rather the command pattern with a light state enum wrapper provided the flexibility necessary for the CLI application you can find them in the `src::game::cli::states.rs` file

## `eris-macro`

Used to test some more interesting methods of code generation. ( e.g., parsing variant attributes ). You can find cleaner more robust implementations in crates like `darling` [https://crates.io/crates/darling] but deriving them from first priciples is a very interesting exercise in learning procedural macros.

## `items`

Stores the items we will automatically generate. The idea was to use a simple DSL to allow "Designers" to create game objects without rust knowledge

## `des.lua`

The wave-function-collapse algorithim is quite interesting and implementing it in rust is trivial but it felt like a fun exercise to write it in lua since I have never written lua before this and have it deserialized in the `src::game::cli::locations.rs` crate with some added logic

## `statc-config`

Configuration to help with the automatic code generation from `items` to code. I chose not to have the build output directly into `target` like something `tonic` would do intentionally. It is far less idiomatic but being able to trivally visualize the output was nicer for this project

While I fundamentally dislike auto code generation for more complex code; Code will be written once but read many times after all. It is a fun game to play to see how much code generation you can leave to tooling. 

## `bin::discord`

Creating a text-based "auto-battler" [https://en.wikipedia.org/wiki/Auto_battler] for discord was a fun challenge given rusts script rules on borrowing & ownerhip these types of frameworks ( serenity [https://github.com/serenity-rs/serenity] ) its fascinating to see how they manage to overcome some of the inherent difficulty. Bevy is another great example in that world of frameworks doing wild things with rust.

I also wanted an abstraction layer over DB models with a custom ORM. I would recomend diesel if you were serious about an orm [https://diesel.rs/] though.

For this project I wanted two basic DBs. An SQL-Like DB - For this project SurrealDB [https://surrealdb.com/] and simple in memory DB.

## `bin::cli`

The CLI implementation is a later implementation ( unfinished ) in order to test crossterm [https://docs.rs/crossterm/latest/crossterm/].

It uses the command pattern which seemes to be an obvious path for a CLI application mixed with a very psuedo-FSM and a WFC algorithim for map generation.

## `bin::two_d`

To be implemented. A version of the CLI WFC map with `bevy` in order to be able to render the world in 2d

## Testing

The tests in this repo are not designed to be run but rather for internal testing. There are no guarantees in this repo it does not hold it self to TDD or anything close. It serves as a test bed to try out odd things. The tests that exist worked at the time they were written but no guarantees beyond that :)