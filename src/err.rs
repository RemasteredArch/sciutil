// SPDX-License-Identifier: MPL-2.0
//
// Copyright © 2025 RemasteredArch
//
// This Source Code Form is subject to the terms of the Mozilla Public License, version 2.0. If a
// copy of the Mozilla Public License was not distributed with this file, You can obtain one at
// <https://mozilla.org/MPL/2.0/>.

//! `err`: Error types that are common across the crate.
//!
//! Defines error types that are (or might be in the future) used throughout the crate. Errors
//! specific to individual modules will be defined by that module.

use thiserror::Error;

/// The error given when the consumer provided an index that causes an out-of-bounds access
/// in a list.
#[derive(Error, Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Default)]
#[error("received an index that caused an out-of-bounds access in a list")]
pub struct OutOfBoundsIndexError;

/// The error given when the consumer provided a character or number for conversion to a digit that
/// is not a digit (is not 0--9).
#[derive(Error, Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Default)]
#[error("received a character or number that is not a valid digit (0-9)")]
pub struct InvalidDigitError;

/// The error given when the consumer provided a float (probably an [`f64`]) that is
/// either [`NaN`] or [infinite] where a [zero], [normal], or [subnormal] float was expected.
///
/// [`NaN`]: std::num::FpCategory::Nan
/// [infinite]: std::num::FpCategory::Infinite
/// [zero]: std::num::FpCategory::Zero
/// [normal]: std::num::FpCategory::Normal
/// [subnormal]: std::num::FpCategory::Subnormal
#[derive(Error, Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd, Hash)]
#[error("received a character or number that is not a valid digit (0-9)")]
pub enum InvalidFloatError {
    Nan,
    Infinite,
}
