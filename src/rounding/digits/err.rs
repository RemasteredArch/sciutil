// SPDX-License-Identifier: MPL-2.0
//
// Copyright © 2025 RemasteredArch
//
// This Source Code Form is subject to the terms of the Mozilla Public License, version 2.0. If a
// copy of the Mozilla Public License was not distributed with this file, You can obtain one at
// <https://mozilla.org/MPL/2.0/>.

//! `err`: Error types for [`super`].

use thiserror::Error;

/// The error given when the consumer provides invalid parts to [`super::Digits::from_parts`].
#[derive(Error, Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd, Hash)]
pub enum InvalidDigitsPartsError {
    #[error("received dot index that is greater than the length of the digits list")]
    OutOfBoundsDot,
    #[error("received digits list without any digits")]
    EmptyDigitsList,
}

/// The error given when the consumer provided an [`super::Place`] that does not exist in the
/// [`super::Digits`] queried.
#[derive(Error, Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Default)]
#[error("received an place that does not exist in this `Digits`")]
pub struct OutOfBoundsPlaceError;
