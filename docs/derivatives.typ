
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

= Time shifted

== First order

Time shifted derivatives estimate the derivative at the start of a time interval ($f^pr_2$)
instead of the average of a time interval ($f^pr_("avg", 23)$).

The precise equation is as follows.
For example, $t$ might be time, $f$ position, and $f pr$ velocity.

$$$
  f^pr_2 &= (f^pr_(avg, 23) Dt_12 + f^pr_(avg, 12) Dt_23) / Dt_13\
$$$

Where:

- $f^pr_(avg, 12) = Df_12 / Dt_12 = (f_2 - f_1) / (t_2 - t_1)$

- $f^pr_(avg, 23) = Df_23 / Dt_23 = (f_3 - f_2) / (t_3 - t_2)$


This technique comes from
William Leonard's "The Dangers of Automated Data Analysis,"
pub. _The Physics Teacher,_ vol. 35, April 1996, p. 220.

The precise equation comes from linear interpolation.
It interpolates between the average derivatives
(the usual manner of finding a numeric derivative)
over $Dt_12$ and $Dt_23$
in order to estimate the derivative at $t_2$.
For the curious,
I will derive this equation on the following page.
For the sake of notational simplicity,
I will use simple $x$ and $y$ terms
when expanding linear interpolation,
then plug in the exact variables and time intervals used
for further simplification.

$$$
  (y - y_0) / (x - x_0) &= (y_1 - y_0) / (x_1 - x_0) &&because "linear interpolation"\
  y - y_0 &= (x - x_0) (y_1 - y_0) / (x_1 - x_0)\
  y &= (x - x_0) (y_1 - y_0) / (x_1 - x_0) + y_0\
  y &= (x - x_0) (y_1 - y_0) / (x_1 - x_0) + (x_1 - x_0) / (x_1 - x_0) y_0\
  y &= ((x - x_0) (y_1 - y_0) + (x_1 - x_0) y_0) / (x_1 - x_0)\
  y &= ((x y_1 - x y_0 - x_0 y_1 + x_0 y_0) + (x_1 y_0 - x_0 y_0)) / (x_1 - x_0)\
  y &= ((x y_1 - x_0 y_1) + (-x y_0 + x_0 y_0 + x_1 y_0 - x_0 y_0)) / (x_1 - x_0)\
  y &= (y_1 (x - x_0) + y_0 (-x + x_0 + x_1 - x_0)) / (x_1 - x_0)\
  y &= (y_1 (x - x_0) + y_0 (x_1 - x)) / (x_1 - x_0) &&(<- "this is also just linear interpolation")\
  f^pr_2 &= (f^pr_(avg, 23) (t_2 - t_("mid", 12)) + f^pr_(avg, 12) (t_("mid", 23) - t_2)) / (t_("mid", 23) - t_("mid", 12)) &&because "see Fig. 2 in the article"\
  f^pr_2 &= (f^pr_(avg, 23) (t_2 - (t_1 + t_2) / 2) + f^pr_(avg, 12) ((t_2 + t_3) / 2 - t_2)) / ((t_2 + t_3) / 2 - (t_1 + t_2) / 2)\
  f^pr_2 &= (f^pr_(avg, 23) (2)(t_2 - (t_1 + t_2) / 2) + f^pr_(avg, 12) (2)((t_2 + t_3) / 2 - t_2)) / ((t_2 + t_3) - (t_1 + t_2))\
  f^pr_2 &= (f^pr_(avg, 23) (2 t_2 - t_1 - t_2) + f^pr_(avg, 12) (t_2 + t_3 - 2t_2)) / (t_3 - t_1)\
  f^pr_2 &= (f^pr_(avg, 23) (t_2 - t_1) + f^pr_(avg, 12) (t_3 - t_2)) / (t_3 - t_1)\
  f^pr_2 &= (f^pr_(avg, 23) Dt_12 + f^pr_(avg, 12) Dt_23) / Dt_13\
$$$
