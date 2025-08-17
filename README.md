# Spider

A simple terminal spider solitaire implementation.

![Demo](./demo.gif)

Heaviliy inspired by [solVItaire](https://gir.st/sol.html). Controls are much the same.

## Features

- Automatically choose how many cards you want to move
- Saves your game every move
- Unlimited undo. Undo history is saved with the game
- Two "cheats" for if you nearly win but can't quite make it work.
    - Re-deal stacks takes one completed suit and puts it back on top of the deck
    - Harvet top row takes the top card from every stack and puts those back in the deck

## Todo

- Some effect on victory
- Shift + number to select how many cards you want to move
- Prevent dealing if you have an empty stack

## Building

As simple as `cargo run`.

## Controlls

<kbd>0</kbd>-<kbd>9</kbd> Select a stack to move from/to
<kbd>enter</kbd> Deal a row of cards
<kbd>Shift</kbd>+<kbd>R</kbd> Start a new game. Did this by accident? Old game will be moved to `spider-save.backup.json` so you can manully move it back.
<kbd>Shift</kbd>+<kbd>C</kbd> Open cheat menu