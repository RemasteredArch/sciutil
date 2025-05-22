// SPDX-License-Identifier: MPL-2.0
//
// Copyright © 2025 RemasteredArch
//
// This Source Code Form is subject to the terms of the Mozilla Public License, version 2.0. If a
// copy of the Mozilla Public License was not distributed with this file, You can obtain one at
// <https://mozilla.org/MPL/2.0/>.

//! Sciutil is a library for scientific computation.
//!
//! ## Features
//!
//! - [`rounding`]: Facilities for rounding floating-point values.
//! - [`units`]: Traits and wrapper structs for treating floating-point values as physical
//!   measurements.
//! - [`statistics`]: List operations for statistics.
//!   - [`statistics::derivatives`]: A few forms of numeric derivatives.
//! - [`display`]: Miscellaneous facilities for pretty-printing things.
//!
//! ## License
//!
//! Sciutil is licensed under the Mozilla Public License, version 2.0 or (as the license
//! stipulates) any later version. A copy of the license should be distributed with sciutil,
//! located at `LICENSE`, or you can obtain one at <https://mozilla.org/MPL/2.0/>.

#![warn(clippy::nursery, clippy::pedantic)]

pub mod display;
pub mod err;
pub mod rounding;
pub mod statistics;
pub mod units;
