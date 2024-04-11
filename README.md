# jbtterminal

A value calculator supporting multiple value lists in a terminal

## How to use

-   `$` changes the value list jbtc -> jbtrading etc.
-   `@` sets a new counter trade

-   To calculate value just type the items.
-   If an item has a space use `_` to seperate.
-   If an item is duped in use `duped_<item>` it will use the duped_value
    or value.

## Values are outdated

### !! Initialize secrets.json for more go to src/trading/jbtc/parser.ts

If values are outdated run `bun run update` it will run all the parser scripts
in `src/trading`.

To install dependencies:

```bash
bun install
```

To run:

```bash
bun run src/index.ts
```

To Build:

```bash
bun run build
```
