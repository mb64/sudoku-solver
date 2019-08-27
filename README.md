# Sudoku solver

This is quick-and-dirty code that I don't intend to publish to Crates.io. It
gets the right answer, but there is no documentation and it definitely could
be prettier.

Licensed under either the MIT license or Apache 2.0, at your choice.

## Data layout

A Sudoku board (the `Board` struct) is represented as a 9x9 array of squares
(the `Square` struct).

Each square is either known, a number 1-9, or unknown, in which case the set of
numbers it is allowed to be is stored.

The `Square` struct uses a `u16`, storing known numbers in the lower 4 bits and
using the upper bits as a bitset to store the possibilites.

## Algorithm

It loops through the squares, and any unknown square with only one possibility
in its set gets promoted to a known square, and the possibility sets of its
neighbors (the squares in its row, column, or box) get updated.

If there are no such squares, it finds the unknown square with the fewest
possibilities and tries all of them recursively.

## Interface

You need a `Board` – this can be done either with a `[[Square; 9]; 9]` or the
`from_text` method. Examples of the text format are in the `sudokus` directory.

Then, calling `.solve()` will solve it. `.solve(true)` will attempt to find all
solutions, while `.solve(false)` will return the first one it finds.
