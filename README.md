# jbtterminal

A value calculator supporting multiple value lists in a terminal

## How to use

-   `$` changes the value list jbtc -> jbtrading etc.
-   `@` sets a new counter trade

-   To calculate value just type the items.
-   If an item has a space use `_` to seperate.
-   If an item is duped in frost put `duped_<item>` it will use the duped_value
    or value. Remember some items dont have duped_value.
-   If you want to have multiple items use `<count>x_<item>` example:
    `4x_concept`

## Values are outdated

If values are outdated run `bun run update` it will run all the parser scripts
in `src/trading`. Or if you are on node use

## Dev related stuff

To Update cache:

```bash
# Initialize secrets.json for more go to src/trading/jbtc/parser.ts
bun run update
```

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
