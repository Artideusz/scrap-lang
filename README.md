# Scrap-Lang - A simple PPL (Piped Processing Language) written in Rust

This is a little toy language I'm making for fun. This is a very early version of this project so it will probably be expanded with new commands and functions.


Usage:
```
cargo run ./test.scrap
```

Syntax:
```
| generate count=5 /* Generates 5 rows with 1 column */
| eval hello="world" /* Creates a new column and fills its cells with a defined value */
```

Roadmap:

- Moving and cleaning the code up a bit.
- Adding REPL.
- Better error handling and adding an error formatter so the user knows which section of the query caused an error.
- Adding `rename` - renames a column.
- Adding `remove` - removes a column.
- Adding `where` - Filters rows by some condition.
- Adding aggregation functions.
- Adding `sort`.
- And more!