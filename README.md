# jbtterminal

v0.1.1

A value calculator supporting multiple value lists in a terminal

## How to run it

### From source

-   run `git clone https://github.com/Lolikarbuzik/jbterminal.git`
-   install bun
-   run `bun run build`
-   you will have a `jbconsole.js` file use `bun jbconsole.js`

### Releases

-   Download latest release
-   Install a javascript runtime (node/bun)
-   run `node jbconsole.js` or if you are using bun `bun jbconsole.js`

## How to use

-   `@` changes the value list.
-   `!` sets a new counter trade
-   `?` Info about the current trade
-   `^` Previous trade
-   `/<user>` to check if user has duped

-   To calculate value just type the items.
-   If an item has a space use `_` to seperate.
-   If an item is duped in frost put `duped_<item>` it will use the duped_value
    or value. Remember some items dont have duped_value.
-   If you want to have multiple items use `<count>x_<item>` example:
    `4x_concept`

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
