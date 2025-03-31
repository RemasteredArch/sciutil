# Sciutil

A library for scientific computation.

## Features

- [`rounding`](./src/rounding/): Rounding floating-point values to match floating-point uncertainty values
  to one or two significant figures.
  - E.g., `rounding::round_with_uncertainty(1024.0511231255, 0.015555312, "g")`
    -> `1024.051 g ± 0.016 g`.
- [`units`](./src/units/): Wrapper structs to mark arbitrary floating-point values as SI units.
- [`statistics`](./src/statistics/): List operations for statistics.
- [`display`](./src/display/): Miscellaneous facilities for pretty-printing things.

## Stability

For the time being, this library make no guarantees of stability,
it's code I will write, update, and break whenever I feel the need or desire to.
I am writing this for my own use,
but if you benefit from this library,
I would be happy to make a stronger stability guarantee.
You are also welcome to request features
or make contributions.

A part of these breaking changes includes possible name changes,
if "sciutil" is ever to be registered by someone else
or I otherwise decide there's a better name.

## License

Sciutil is licensed under the Mozilla Public License,
version 2.0 or (as the license stipulates) any later version.
A copy of the license should be distributed with sciutil,
located at [`LICENSE`](./LICENSE),
or you can obtain one at
<https://mozilla.org/MPL/2.0/>.
