# Sciutil

A library for scientific computation.

## Features

- [`rounding`](./src/rounding/):
  Rounding floating-point values to match floating-point uncertainty values
  to one or two significant figures.
  - E.g., `rounding::round_with_uncertainty(1024.0511231255, 0.015555312, "g")`
    -> `1024.051 g ± 0.016 g`.
- [`units`](./src/units/):
  Traits and wrapper structs for treating floating-point values as physical measurements.
  - Traits and structs that embed physical units at the type level.
  - A struct that pairs a measured value with its absolute uncertainty.
- [`statistics`](./src/statistics/):
  List operations for statistics.
  - [`statistics::derivatives`](./src/statistics/derivatives/):
    A few forms of numeric derivatives.
- [`display`](./src/display/):
  Miscellaneous facilities for pretty-printing things.

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

## Looking forwards

There's a few features I'm looking to add in the future.
I'm not certain if I will ever add them,
but feature requests and pull requests are welcome.

- Calculating a best-fit line with a linear regression is something I want,
  but I am not familiar enough with linear algebra to do that easily,
  as it's not something I understand.
  As of right now, I just use Desmos's built-in implementation.
- A simple and opinionated API for generating plots from data is also something I'd like.
  As of right now, I use Desmos plots,
  but Desmos can be limiting.
  Specifically, being limited to one-dimensional arrays
  and not having sciutil's rounding implementation
  are limiting for me.
  I'm hesitant to do it because I know it will involve a lot of work.

## License

Sciutil is licensed under the Mozilla Public License,
version 2.0 or (as the license stipulates) any later version.
A copy of the license should be distributed with sciutil,
located at [`LICENSE`](./LICENSE),
or you can obtain one at
<https://mozilla.org/MPL/2.0/>.
