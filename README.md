# Scrap-Lang - A simple PPL (Piped Processing Language) written in Rust

This is a little toy language I'm making for fun. This is a very early version of this project so it will probably be expanded with new commands and functions.


Usage:
```
cargo run
```

Example Syntax:
```
| generate count=5 /* Generates 5 rows with 1 column */
| eval hello="world" /* Creates a new column and fills its cells with a defined value */
```

## Available Scrap-Lang commands:

### generate

`generate` is a command which allows you to create a new table with a specified number of rows.

#### Syntax

```
| generate count=<number> [name=<string>]
```
- `count` - The amount of rows to generate.
- `name` - An optional name for the generated column (Default: `$1`).

#### Example

```
| generate count = 3

+---------+
| $1      |
+---------+
| <Empty> |
+---------+
| <Empty> |
+---------+
```

### eval

`eval` allows you to:
- Create new columns with a specified value for every row.
- Modify existing columns.

#### Syntax

```
| eval <identifier> = <expression>
```

#### Example

```
| generate count=10 name="hello"
| eval x = 5
| eval hello = x + $rowcount

+-------------+
| hello   | x |
+-------------+
| 6       | 5 |
+-------------+
| 7       | 5 |
+-------------+
| 8       | 5 |
+-------------+
| 9       | 5 |
+-------------+
| 10      | 5 |
+-------------+
| 11      | 5 |
+-------------+
| 12      | 5 |
+-------------+
| 13      | 5 |
+-------------+
| 14      | 5 |
+-------------+
| 15      | 5 |
+-------------+
```

### where

This command filters the results by some comparison.

#### Syntax

```
| where <value> <comparison_operator> <value>
```

- `value` - Either String, Number, Identifier or SpecialIdentifier.
- `comparison_operator` - Either `<`, `>`, `<=`, `>=`, `!=` or `==`.

#### Example

```
| generate count=5 name="xyz"
| eval xyz = $rowcount
| where xyz < 3

+---------+
| xyz     |
+---------+
| 1       |
+---------+
| 2       |
+---------+
```

## Available REPL commands:

Scrap-Lang comes with a REPL environment by default, where you can play with the language. These commands do not interact with the language and are just there for ease of use.

### quit

Exits out of the REPL environmnet.

### table

Displays the output table.

### clear

Clears the terminal.

## Roadmap:

- Moving and cleaning the code up a bit.
- Adding REPL. (Done)
- Better error handling and adding an error formatter so the user knows which section of the query caused an error.
- Adding `rename` - renames a column.
- Adding `remove` - removes a column.
- Adding `where` - Filters rows by some condition.
- Adding aggregation functions.
- Adding `sort`.
- And more!