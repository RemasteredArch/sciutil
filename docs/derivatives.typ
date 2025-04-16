
// SPDX-License-Identifier: MPL-2.0
//
// Copyright © 2025 RemasteredArch
//
// This Source Code Form is subject to the terms of the Mozilla Public License, version 2.0. If a
// copy of the Mozilla Public License was not distributed with this file, You can obtain one at
// <https://mozilla.org/MPL/2.0/>.

// `derivatives.typ`: documentation in Typst covering the math behind the code for derivatives.
//
// For the unitiated, this is a [Typst](https://typst.app/) document. It's intended to be viewed in
// a rendered form. There is an official (closed source) web app available, but you can also render
// it with the open source CLI:
//
// ```console
// $ # Install Typst
// $ cargo install typst-cli
// $ # Compile the document
// $ typst compile 'docs/derivatives.typ' 'docs/derivatives.pdf'
// $ # Now open the PDF in your preferred reader.
// ```
//
// There's a _lot_ happening in this file before any of the content starts, because it's based on
// the same preamble I use for more advanced Typst documents. I'll probably trim it down later, but
// it doesn't affect the rendered document so I don't much care.


#let dth = $d theta$
#let ddth = $d / dth$
#let Dth = $Delta theta$
#let lth0 = $limits(lim)_(Dth -> 0)$
#let lxa = $limits(lim)_(x -> a)$
#let lxap = $limits(lim)_(x -> a^+)$
#let lxan = $limits(lim)_(x -> a^-)$
#let lx1 = $limits(lim)_(x -> 1)$
#let lxi = $limits(lim)_(x -> infinity)$
#let lni = $limits(lim)_(n -> infinity)$
#let lNi = $limits(lim)_(N -> infinity)$

#let pm = $plus.minus$

#let int = $integral$

// Center, mean
#let xbar = $dash(x)$

// Centimeters or Center of Mass
#let cm = $"cm"$
// System
#let sys = $"sys"$
// External
#let ext = $"ext"$
// Net
#let net = $"net"$
// Approximate
// #let approx = $"approx"$

// Primes
#let pr = $prime$
#let prd = $prime.double$
#let prt = $prime.triple$

// These are all going to display with italics. Is that fine? I could use `#[]` to make them not.
#let vD = $harpoon(D)$
#let vF = $harpoon(F)$
#let vM = $harpoon(M)$
#let vN = $harpoon(N)$
#let vT = $harpoon(T)$
#let vW = $harpoon(W)$
#let va = $harpoon(a)$
#let vb = $harpoon(b)$
#let vc = $harpoon(c)$
#let vd = $harpoon(d)$
#let vf = $harpoon(f)$
#let vg = $harpoon(g)$
#let vp = $harpoon(p)$
#let vP = $harpoon(P)$
#let vl = $harpoon(l)$
#let vL = $harpoon(L)$
#let vr = $harpoon(r)$
#let vr1 = $harpoon(r_1)$
#let vr2 = $harpoon(r_2)$
#let vr3 = $harpoon(r_3)$
#let vs = $harpoon(s)$
#let vv = $harpoon(v)$
#let vx = $harpoon(x)$

#let vom = $harpoon(omega)$
#let val = $harpoon(alpha)$
#let vta = $harpoon(tau)$

#let Dvr = $Delta harpoon(r)$
#let Dvv = $Delta harpoon(v)$
#let Dvp = $Delta vp$
#let DvP = $Delta vP$

#let dvrdt = $(d vr) / (d t)$
#let dvvdt = $(d vv) / (d t)$
#let dvPdt = $(d vP) / (d t)$
#let dvldt = $(d vl) / (d t)$
#let dvLdt = $(d vL) / (d t)$

#let ihat = $hat(i)$
#let jhat = $hat(j)$
#let khat = $hat(k)$
#let inhat = $hat(times.circle)$
#let outhat = $hat(dot.circle)$

#let avg = "avg"

#let dt = $d t$
#let dx = $d x$
#let dy = $d y$
#let df = $d f$
#let dz = $d z$
#let dv = $d v$
#let dV = $d V$
#let dm = $d m$
#let ddx = $d / dx$
#let ddr = $d / (d r)$
#let dydx = $dy / dx$
#let dydt = $dy / dt$
#let drdt = $(d r) / dt$
#let dVdt = $(d V) / dt$
#let dvdt = $dv / dt$
#let ddt = $d / dt$
#let dfdt = $df / dt$
#let dxdt = $dx / dt$
#let dzdt = $dz / dt$
#let dhdt = $(d h) / dt$
#let dTdt = $(d T) / dt$
#let dAdt = $(d A) / dt$
#let Da = $Delta a$
#let DE = $Delta E$
#let DL = $Delta L$
#let Dm = $Delta m$
#let DM = $Delta M$
#let Df = $Delta f$
#let Dg = $Delta g$
#let Dh = $Delta h$
#let DH = $Delta H$
#let Ds = $Delta s$
#let DS = $Delta S$
#let Dt = $Delta t$
#let Du = $Delta u$
#let DU = $Delta U$
#let Dk = $Delta k$
#let DK = $Delta K$
#let Dv = $Delta v$
#let DV = $Delta V$
#let Dx = $Delta x$
#let Dy = $Delta y$
#let Dz = $Delta z$

#let lDx0 = $limits(lim)_(Dx -> 0)$
#let lDt0 = $limits(lim)_(Dt -> 0)$
#let lx0 = $limits(lim)_(x -> 0)$
#let lx0p = $limits(lim)_(x -> 0^+)$
#let lx0n = $limits(lim)_(x -> 0^-)$
#let lh0 = $limits(lim)_(h -> 0)$
#let lh0p = $limits(lim)_(h -> 0^+)$
#let lh0n = $limits(lim)_(h -> 0^-)$
#let lt0 = $limits(lim)_(t -> 0)$


#let units = (
  centimeter: "cm",
  meter: "m",
  kilogram: "kg",
  pounds: "lbs",
  newton: "N",
  second: "s",
  joule: "J",
  watt: "W",
)

#let dimensions = (
  mass: "M",
  length: "L",
  time: "T",
)
#dimensions.insert("force", $#dimensions.mass dot dimensions.length / dimensions.time^2$)

#show quote.where(block: true): block.with(stroke: (left: 1.5pt + gray, rest: none))

#let bx(content) = box(stroke: black, inset: 3pt)[#content]

#set heading(numbering: "1.")
#let no_num(content) = {
  set heading(numbering: none)
  content
}
#let title(content) = {
  no_num[= #smallcaps(content)]
}

// Override heading display logic to allow for finer styling.
// Specifically, this sets numbering to be a a smaller size, lighter color, and lighter weight,
// and places a larger space between numbering and the body.
//
// Does not format headings without numbering.
#show heading: it => {
  // No formatting tweaks for non-standard headings (without numbering).
  if it.numbering == none {
    return it.body
  }

  // Insert page breaks before level one headings.
  //
  // Avoids inserting a page break before the first heading to avoid an empty first page.
  let maybe_break = if it.level == 1 and counter(heading).get() != (1,) {
    pagebreak()
  }

  // Separates the heading into a grid:
  //
  // ```txt
  // |-----------|-----|--------------|
  // | Numbering | Gap | Heading Body |
  // |-----------|-----|--------------|
  // ```
  (
    maybe_break
      + grid(
        columns: 2,
        // `1em` gap between numbering and body.
        column-gutter: 1em,
        inset: (bottom: 0.5em, top: 0.25em),
        // Custom display logic for numbering.
        block(
          // Lines up the `0.75em` text to be roughly aligned with the bottom of the first line of
          // text.
          inset: (top: 0.15em),
          // Size of `0.75em` and a lighter color and weight.
          text(counter(heading).display(it.numbering), size: 0.75em, weight: "semibold", luma(50)),
        ),
        // Body displayed as normal.
        it.body,
      )
  )
}

#show link: text.with(blue)
#show link: underline.with(offset: 0.15em)
#let url(url, breakable: false) = box(block(breakable: breakable)[\<#link(url)>])

#set page(
  paper: "us-letter",
  numbering: "( 1 / 1 )",
  number-align: top + right,
)

#let title_text = [Numeric Derivatives]

#set document(
  title: title_text,
  author: "RemasteredArch",
)

#title[#title_text]

= Introduction

This document covers the math behind the code for numeric derivatives in sciutil.

This document assumes basic calculus knowledge.
Steve Brunton's YouTube video
"Calculus Review: The Derivative (and the Power Law and Chain Rule)"
#url("https://youtu.be/-NhgElcA3K8")
may be worth a watch if you are unfamiliar with calculus.
I have not watched this video and thus cannot vouch for it in confidence,
but I found his videos on numeric differentiation to be excellent.
Alternatively,
3Blue1Brown's YouTube course "Essence of calculus"
#url("https://www.youtube.com/playlist?list=PLZHQObOWTQDMsr9K-rj53DwVRMYO3t5Yr")
comes highly reviewed,
and I can vouch for the first few videos being excellent.

== Notation

For the unfamiliar, here are a few notable pieces of notation used in this document:

#[
  #set par(
    // Wider line spacing (default is `0.65em`).
    leading: 0.65 * 1.5em,
  )

  - $t$ is the independent variable, $f$ is the dependent variable.
    These are equivalent to $x$ and $y$, respectively.
    - Derivatives are noted with "primes",
      so the first derivative ("first-order") is $f pr$,
      the second ("second-order") is $f prd$,
      and the third is $f prt$.
      Higher-order derivatives are noted with digits in parentheses.
      The fourth derivative is $f^((4))$, for example.
  - Subscripts hold different modifiers, separated by commas.
    They denote indices, intervals, averages, midpoints, etc.
    - For example, $t_1$ is the first item in the list $t$
      (lists are 1-indexed).
  - $Delta$ refers to change in a variable over some interval.
    For example, $Dt_(1, 2) = t_2 - t_1$.
  - In subscripts, two-digit numbers refer to intervals,
    dropping the comma for convenience.
    For example,
    $Dt_12 = Dt_(1, 2) = t_2 - t_1$.
    - "$avg$" in subscripts denotes the average value over this interval.
      For example, $f_(avg, 12) = (f_1 + f_2) / Dt_12$.
    - Similarly,
      "$"mid"$" in subscripts denotes the midpoint in this interval.
      For example, $t_("mid", 12) = (t_1 + t_2) / 2$.
  - $because$ is shorthand for "because."
    This is used to justify a claim.
  - $|$ is notation for "where."
    This is used to define terms or specify requirements for an expression.
  - $[a, b]$ is the interval from $a$ to $b$ (including $a$ and $b$).
    For example, on the x-axis, it's every point between $x = a$ to $x = b$.
  - $in$ is notation for "in,"
    used to claim that a point exists within an interval.
    For example, $x in [a, b]$ says that some point $x$ exists at or between $a$ and $b$.
  - $<<$ is notation for "much less than."
]

== Definitions

#block(breakable: false, width: 100%)[
  Given some function $f$,
  some constant offset $Dt$ (that may be positive or negative),
  and some $eta in [t, t + Dt]$,
  we define:

  $$$
  // alpha, n + 1; sub dt for alpha
    R_(n + 1) &= (Dt)^(n + 1) / (n + 1)! f^((n + 1)) (t + eta)
  $$$

  This is something we'll use during Taylor Series expansions.
]

= Traditional numeric derivatives

All these derivatives work off the simple principle of "rise over run."
They find their differences only in where they gather the points to compare.

== First-order

The first-order derivative
operates on some ordered list of $(t, f)$ values,
returning a list of each derivative of $f$ with respect to $t$
(a list of $(t, f pr)$).

For the first item in the list,
it relies on the forward difference derivative.
For the last item in the list,
it relies on the backward difference derivative.
For every other item in the list,
it relies on the central difference derivative.
Note that the forward and backward difference derivatives have greater error,
so the first and last items will be less accurate than the middle items.
This is mostly fine for the first-order derivative,
but it's problematic for higher-order derivatives.

=== Central difference derivative

Given some point $t_2$,
the central difference derivative
estimates the derivative of $f$ at $t_2$ as follows:

$$$
  f^pr_2 = (f_3 - f_1) / (t_3 - t_1)
$$$

This is the preferred method over the forward and backwards difference derivatives
because its error from the actual value of $f pr$
is proportional to $f prt$ instead of $f prd$,
but it cannot be used on the first or last item of a list.

==== Calculating error <central_derivative_error>

Suppose $f$ is a continuous function
with $n + 1$ derivatives,
rather than a discrete set of points.
We define the central difference derivative
in terms of an arbitrary $Dt$ instead.

$$$
  f pr (t) &approx (f(t + Dt) - f(t - Dt)) / (2 Dt)\
$$$

#block(breakable: false, width: 100%)[
  We can use the Taylor Series expansions of
  $f(t + Dt)$ (see @forward_derivative_error)
  and $f(t - Dt)$ (see @backward_derivative_error)
  to find the exact value of our approximation:

  $$$
    &(f(t + Dt) - f(t - Dt)) / (2 Dt)\
    &= #block[$ ((#block[$ &( f(t)
      + Dt f pr (t)
      + (Dt)^2 / 2! f prd (t)
      + dots
      + (Dt)^n / n! f^((n)) (t)
      + R_(n + 1) )\
    &""- ( f(t)
      - Dt f pr (t)
      + (Dt)^2 / 2! f pr (t)
      - (Dt)^3 / 3! f prt (t)
      + dots
      + (-Dt)^n / n! f^((n)) (t)
      + R_(n + 1)) $] )) / (2 Dt)
    $]\
    &= (
      2 Dt f pr (t)
      + 2 (Dt)^3 / 3! f prt (t)
      + dots
      + 2 (Dt)^m / m! f^((m)) (t)
      + 2 R_(m + 1)
    ) / (2 Dt) "     "| m = cases(
      n &| n "is odd",
      n - 1 &| n "is even",
    )\
    &= f pr (t)
      + (Dt)^2 / 3! f prt (t)
      + dots
      + (Dt)^(m - 1) / m! f^((m)) (t)
      + R_(m + 1) / Dt
  $$$
]

Because the expansion of $f(t + Dt)$ is always positive
and the expansion of $f(t - Dt)$
is negative for terms with odd-ordered derivatives
and positive for even-ordered derivatives,
this expansion includes only the terms with odd-ordered derivatives.

This expansion has the actual value of $f pr (t)$ present,
so the rest is the error of our approximation.
This error is on the order of $(Dt)^2$.
Because for small $Dt$,
increasingly higher powers of $Dt$ are increasingly small,
all terms but the leading term are insignificant,
giving us a leading order error term of $(Dt)^2 / 3! f prt (t)$.
Further,
that $Dt^2 << Dt$
leads us to the claim that the central difference derivative
is much more accurate than the forward and backward difference derivatives.

=== Forward difference derivative

Given some point $t_2$,
the forward difference derivative
estimates the derivative of $f$ at $t_2$ as follows:

$$$
  f^pr_2 = (f_3 - f_2) / (t_3 - t_2)
$$$

The central difference derivative should be preferred over this method
because its error from the actual value of $f pr$
is proportional to $f prd$ instead of $f prt$,
but this is the only option for the first item in a list.

==== Calculating error <forward_derivative_error>

Suppose $f$ is a continuous function
with $n + 1$ derivatives,
rather than a discrete set of points.
We define the forward difference derivative
in terms of an arbitrary $Dt$ instead.

$$$
  f pr (t) &approx (f(t + Dt) - f(t)) / Dt\
$$$

#block(breakable: false, width: 100%)[
  We can use a Taylor Series expansion of $f(t + Dt)$
  to find the exact value of our approximation:

  $$$
    f(t + Dt) &= sum_(m = 0)^n (Dt)^m / m! f^((m)) (t) + R_(n + 1)\
    f(t + Dt) &=
    (Dt)^0 / 0! f^((0)) (t)
    + (Dt)^1 / 1! f^((1)) (t)
    + (Dt)^2 / 2! f^((2)) (t)
    + dots
    + (Dt)^n / n! f^((n)) (t)
    + R_(n + 1)\
    f(t + Dt) &=
    f(t)
    + Dt f pr (t)
    + (Dt)^2 / 2! f prd (t)
    + dots
    + (Dt)^n / n! f^((n)) (t)
    + R_(n + 1)\
    (f(t + Dt) - f(t)) / Dt &=
    f pr (t)
    + Dt / 2! f prd (t)
    + dots
    + (Dt)^(n - 1) / n! f^((n)) (t)
    + R_(n + 1) / Dt\
  $$$
]

This expansion has the actual value of $f pr (t)$ present,
so the rest is the error of our approximation.
This error is on the order of $Dt$.
For sufficiently small $Dt$,
$(Dt)^n | n > 1$ is so much smaller than $Dt$ that all but the leading term are insignificant,
giving us a leading order error term of $Dt / 2! f prd (t)$.

=== Backward difference derivative

Given some point $t_2$,
the backward difference derivative
estimates the derivative of $f$ at $t_2$ as follows:

$$$
  f^pr_2 = (f_2 - f_1) / (t_2 - t_1)
$$$

The central difference derivative should be preferred over this method
because its error from the actual value of $f pr$
is proportional to $f prd$ instead of $f prt$,
but this is the only option for the last item in a list.

==== Calculating error <backward_derivative_error>

Suppose $f$ is a continuous function
with $n + 1$ derivatives,
rather than a discrete set of points.
We define the backward difference derivative
in terms of an arbitrary $Dt$ instead.

$$$
  f pr (t) &approx (f(t) - f(t - Dt)) / Dt\
$$$

#block(breakable: false, width: 100%)[
  We can use a Taylor Series expansion of $f(t + Dt)$
  to find the exact value of our approximation:

  $$$
    f(t - Dt) &= sum_(m = 0)^n (-Dt)^m / m! f^((m)) (t) + R_(n + 1)\
    f(t - Dt) &=
    (-Dt)^0 / 0! f^((0)) (t)
    + (-Dt)^1 / 1! f^((1)) (t)
    + (-Dt)^2 / 2! f^((2)) (t)\
    &#hide[$=$] "" + (-Dt)^3 / 3! f^((3)) (t)
    + dots
    + (-Dt)^n / n! f^((n)) (t)
    + R_(n + 1)\
    f(t - Dt) &=
    f(t)
    - Dt f pr (t)
    + (Dt)^2 / 2! f pr (t)
    - (Dt)^3 / 3! f prt (t)
    + dots
    + (-Dt)^n / n! f^((n)) (t)
    + R_(n + 1)\
    (f(t) - f(t - Dt)) / Dt &= - 1/ Dt (
    - Dt f pr (t)
    + (Dt)^2 / 2! f pr (t)
    - (Dt)^3 / 3! f prt (t)
    + dots
    + (-Dt)^n / n! f^((n)) (t)
    + R_(n + 1))\
    (f(t) - f(t - Dt)) / Dt &=
    f pr (t)
    - Dt / 2! f pr (t)
    + (Dt)^2 / 3! f prt (t)
    - dots
    - (-Dt)^(n - 1) / n! f^((n)) (t)
    - R_(n + 1) / Dt\
  $$$
]

This expansion has the actual value of $f pr (t)$ present,
so the rest is the error of our approximation.
This error is on the order of $Dt$.
For sufficiently small $Dt$,
$(Dt)^n | n > 1$ is so much smaller than $Dt$ that all but the leading term are insignificant,
giving us a leading order error term of $- Dt / 2! f prd (t)$.

== Higher-order

The higher-order derivative
operates on some ordered list of $(t, f)$ values,
return a list of each $n$-th derivative of $f$ with respect to $t$
(a list of $(t, f^((n)))$).

It does this by repeatedly re-calculating the first-order derivative on the list.
The first-order derivative will be less accurate for the first and last items in the list,
but this error propagates further into the list
with every recalculation.
Here's a Desmos graph that reimplements the algorithms used by sciutil
to visualize that error.
Notice how the last couple items in $(sin x) prd$
deviate so significantly.
The first couple items do too,
but do so less because of how small $(sin x) prd$ is at $x = 0 $.

#align(
  center,
  [
    #image("images/error propagation graph for numeric derivatives.svg", width: 75%)
    #url("https://www.desmos.com/calculator/e2k5vughza")
  ],
)

= Time-shifted derivatives

Time-shifted derivatives recognize that traditional "rise over run" derivatives
calculate the average derivative over a time interval ($f^pr_("avg", 23)$),
not the derivative precisely at the start of the time interval ($f^pr_2$).
This technique comes from
William Leonard's article "Dangers of Automated Data Analysis,"
pub. _The Physics Teacher,_ vol. 35, April 1996, pp. 220--222,
#link("https://doi.org/10.1119/1.2344655")[DOI 10.1119/1.2344655].

== First-order

Time-shifted derivatives estimate the derivative at the start of a time interval ($f^pr_2$)
instead of the average of a time interval ($f^pr_("avg", 23)$).

The precise equation is as follows.
For example, $t$ might be time, $f$ position, and $f pr$ velocity.

$$$
  f^pr_2 &= (f^pr_(avg, 23) Dt_12 + f^pr_(avg, 12) Dt_23) / Dt_13\
$$$

Where:

- $f^pr_(avg, 12) = Df_12 / Dt_12 = (f_2 - f_1) / (t_2 - t_1)$

- $f^pr_(avg, 23) = Df_23 / Dt_23 = (f_3 - f_2) / (t_3 - t_2)$

The precise equation comes from linear interpolation.
It interpolates between the average derivatives
(the usual manner of finding a numeric derivative)
over $Dt_12$ and $Dt_23$
in order to estimate the derivative at $t_2$.
For the curious,
I will derive this equation on the following page.
For the sake of notational simplicity,
I will use simple $x$ and $y$ terms
when expanding linear interpolation
(finding some $(x, y)$ between $(x_1, y_1)$ and $(x_2, y_2)$),
then plug in the exact variables and time intervals used
for further simplification.

$$$
  (y - y_1) / (x - x_1) &= (y_2 - y_1) / (x_2 - x_1) &&because "linear interpolation"\
  y - y_1 &= (x - x_1) (y_2 - y_1) / (x_2 - x_1)\
  y &= (x - x_1) (y_2 - y_1) / (x_2 - x_1) + y_1\
  y &= (x - x_1) (y_2 - y_1) / (x_2 - x_1) + (x_2 - x_1) / (x_2 - x_1) y_1\
  y &= ((x - x_1) (y_2 - y_1) + (x_2 - x_1) y_1) / (x_2 - x_1)\
  y &= ((x y_2 - x y_1 - x_1 y_2 + x_1 y_1) + (x_2 y_1 - x_1 y_1)) / (x_2 - x_1)\
  y &= ((x y_2 - x_1 y_2) + (-x y_1 + x_1 y_1 + x_2 y_1 - x_1 y_1)) / (x_2 - x_1)\
  y &= (y_2 (x - x_1) + y_1 (-x + x_1 + x_2 - x_1)) / (x_2 - x_1)\
  y &= (y_2 (x - x_1) + y_1 (x_2 - x)) / (x_2 - x_1) &&(<- "this is also just linear interpolation")\
  f^pr_2 &= (f^pr_(avg, 23) (t_2 - t_("mid", 12)) + f^pr_(avg, 12) (t_("mid", 23) - t_2)) / (t_("mid", 23) - t_("mid", 12)) &&because "see Fig. 2 in the article"\
  f^pr_2 &= (f^pr_(avg, 23) (t_2 - (t_1 + t_2) / 2) + f^pr_(avg, 12) ((t_2 + t_3) / 2 - t_2)) / ((t_2 + t_3) / 2 - (t_1 + t_2) / 2)\
  f^pr_2 &= (f^pr_(avg, 23) (2)(t_2 - (t_1 + t_2) / 2) + f^pr_(avg, 12) (2)((t_2 + t_3) / 2 - t_2)) / ((t_2 + t_3) - (t_1 + t_2))\
  f^pr_2 &= (f^pr_(avg, 23) (2 t_2 - t_1 - t_2) + f^pr_(avg, 12) (t_2 + t_3 - 2t_2)) / (t_3 - t_1)\
  f^pr_2 &= (f^pr_(avg, 23) (t_2 - t_1) + f^pr_(avg, 12) (t_3 - t_2)) / (t_3 - t_1)\
  f^pr_2 &= (f^pr_(avg, 23) Dt_12 + f^pr_(avg, 12) Dt_23) / Dt_13\
$$$

== Second-order

Time-shifted derivatives recognize that traditional "rise over run" derivatives
calculate the average derivative over a time interval ($f^pr_("avg", 23)$).
A first-order time-shifted derivative estimates the derivative at the start of a time interval ($f^pr_2$),
but the second-order time-shifted derivative is much simpler.

The precise equation is as follows.
For example, $t$ might be time, $f$ position, $f pr$ velocity, and $f prd$ acceleration.

$$$
  f^prd_2 &= 2 (f^pr_(avg, 23) - f^pr_(avg, 12)) / (t_3 - t_1)\
$$$

Where:

- $f^pr_(avg, 12) = Df_12 / Dt_12 = (f_2 - f_1) / (t_2 - t_1)$

- $f^pr_(avg, 23) = Df_23 / Dt_23 = (f_3 - f_2) / (t_3 - t_2)$

For the curious, I will derive this now.
This is a much simpler process for the second-order time-shifted derivative
than first-order time-shifted derivative.
It is just a central difference derivative
oriented around $t_("mid", 12)$ and $t_("mid", 23)$
instead of $t_1$ and $t_3$.

$$$
  f^prd_2 &= (f^pr_(avg, 23) - f^pr_(avg, 12)) / (t_("mid", 23) - t_("mid", 12))\
  f^prd_2 &= (f^pr_(avg, 23) - f^pr_(avg, 12)) / ((t_3 + t_2) / 2 - (t_2 + t_1) / 2)\
  f^prd_2 &= (f^pr_(avg, 23) - f^pr_(avg, 12)) / (1 / 2 (t_3 + t_2 - t_2 - t_1))\
  f^prd_2 &= 2 (f^pr_(avg, 23) - f^pr_(avg, 12)) / (t_3 - t_1)\
$$$
