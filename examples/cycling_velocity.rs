// SPDX-License-Identifier: MPL-2.0
//
// Copyright © 2025 RemasteredArch
//
// This Source Code Form is subject to the terms of the Mozilla Public License, version 2.0. If a
// copy of the Mozilla Public License was not distributed with this file, You can obtain one at
// <https://mozilla.org/MPL/2.0/>.

#![allow(dead_code)]
#![warn(clippy::nursery, clippy::pedantic)]

mod gpx;
use gpx::{TrackSegment, Velocity};

use sciutil::units::Float;

/// The contents of an example GPX file, compliant with the requirements for
/// [`TrackSegment::parse_first_in_file`].
const WITH_TIME_GPX: &str = include_str!("./data/with_time.gpx");

fn main() {
    let track_points = TrackSegment::parse_first_in_file(WITH_TIME_GPX.as_bytes());

    let position = track_points.degrees_traveled_by_seconds();
    let velocity = sciutil::statistics::derivatives::first_order_time_shifted(&position)
        .into_iter()
        .map(|(t, v)| (t, Velocity::new(v)))
        .collect::<Vec<_>>();

    let position_desmos = sciutil::display::pairs_to_desmos_list("d", position.as_slice());
    let velocity_desmos = sciutil::display::pairs_to_desmos_list("v", velocity.as_slice());

    println!(
        "To graph position (d) and velocity(d) in Desmos:

{position_desmos}

{velocity_desmos}",
    );
}
