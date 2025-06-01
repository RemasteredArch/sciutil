// SPDX-License-Identifier: MPL-2.0
//
// Copyright © 2025 RemasteredArch
//
// This Source Code Form is subject to the terms of the Mozilla Public License, version 2.0. If a
// copy of the Mozilla Public License was not distributed with this file, You can obtain one at
// <https://mozilla.org/MPL/2.0/>.

use std::io::{BufRead, BufReader, Read};

use sciutil::units::{Degrees, Float, Meters, Per, Seconds};
use time::UtcDateTime;

#[derive(Copy, Clone, Debug, Default)]
pub struct Coordinates {
    latitude: Degrees,
    longitude: Degrees,
}

impl Coordinates {
    pub fn midpoint(&self, other: &Self) -> Self {
        Self {
            latitude: self.latitude.get().midpoint(other.longitude.get()).into(),
            longitude: self.longitude.get().midpoint(other.longitude.get()).into(),
        }
    }

    pub fn distance(&self, other: &Self) -> Degrees {
        (self.latitude.get() - other.latitude.get())
            .hypot(self.longitude.get() - other.longitude.get())
            .into()
    }
}

pub type Velocity = Per<Degrees, Seconds, 1>;

/// Represents a track point from a GPX file.
///
/// For example:
///
/// ```xml
/// <trkpt lat="50.790867" lon="4.404968">
///     <ele>109.0</ele>
///     <time>2023-12-31T23:00:00.000Z</time>
/// </trkpt>
/// ```
#[derive(Clone, Debug)]
pub struct TrackPoint {
    coordinates: Coordinates,
    elevation: Meters,
    time: UtcDateTime,
}

impl Default for TrackPoint {
    fn default() -> Self {
        Self {
            coordinates: Coordinates::default(),
            elevation: Meters::default(),
            time: UtcDateTime::UNIX_EPOCH,
        }
    }
}

#[derive(Clone, Debug)]
pub struct TrackSegment(Vec<TrackPoint>);

impl TrackSegment {
    pub const fn new() -> Self {
        Self(Vec::new())
    }

    pub fn coordinates(&self) -> impl Iterator<Item = Coordinates> {
        self.iter().map(|p| p.coordinates)
    }

    pub fn elevation(&self) -> impl Iterator<Item = Meters> {
        self.iter().map(|p| p.elevation)
    }

    pub fn time(&self) -> impl Iterator<Item = UtcDateTime> {
        self.iter().map(|p| p.time)
    }

    pub fn iter(&self) -> impl Iterator<Item = &TrackPoint> {
        self.0.iter()
    }

    pub const fn len(&self) -> usize {
        self.0.len()
    }

    pub const fn is_empty(&self) -> bool {
        self.0.is_empty()
    }

    pub fn first(&self) -> Option<&TrackPoint> {
        self.0.first()
    }

    /// Returns the time since the first track point and the total distance traveled since then.
    ///
    /// The total distance is the sum of [`Self::distance`] between each pair of points between the
    /// first point and the current point. So if you walk to the other side of a room repeatedly,
    /// that will record as you having walked quite far, even though your _displacement_ is (close
    /// to) zero since you began.
    pub fn degrees_traveled_by_seconds(&self) -> Vec<(Seconds, Degrees)> {
        if self.is_empty() {
            return Vec::new();
        }

        let mut list = Vec::with_capacity(self.len());

        let mut prev_point = self.0.first().unwrap();

        let starting_time = prev_point.time;
        list.push((Seconds::new(0.0), Degrees::new(0.0)));

        let mut total_distance = 0.0;

        for point in self.iter().skip(1) {
            total_distance += prev_point.coordinates.distance(&point.coordinates).get();

            list.push((
                Seconds::new((point.time - starting_time).as_seconds_f64()),
                total_distance.into(),
            ));

            prev_point = point;
        }

        list
    }

    /// Parses the first `<trkseg> ... </trkseg>` in a file, with the file passed as a string slice.
    ///
    /// Expects a series of `<trkpt>`s, with each XML tag taking one line:
    ///
    /// ```xml
    /// <!-- ... -->
    /// <trkseg>
    ///     <trkpt lat="50.790867" lon="4.404968">
    ///         <ele>109.0</ele>
    ///         <time>2023-12-31T23:00:00.000Z</time>
    ///     </trkpt>
    ///     <trkpt lat="50.790714" lon="4.405036">
    ///         <ele>110.8</ele>
    ///         <time>2023-12-31T23:00:03.180Z</time>
    ///     </trkpt>
    ///     <!-- ... -->
    /// </trkseg>
    /// <!-- ... -->
    /// ```
    ///
    /// This is not a great parser, but it's good enough for this simple example.
    pub fn parse_first_in_file(gpx_contents: impl Read) -> Self {
        let track_segment = BufReader::new(gpx_contents)
            .lines()
            .map(|maybe_line| maybe_line.unwrap().trim().to_string())
            .skip_while(|line| line != "<trkseg>")
            .skip(1)
            .take_while(|line| line != "</trkseg>");

        let mut current_track_point = TrackPoint::default();
        let mut track_points = Vec::<TrackPoint>::new();

        for line in track_segment {
            if line.starts_with("<trkpt ") {
                let (latitude, line) = line
                    .trim_start_matches("<trkpt lat=\"")
                    .split_once('"')
                    .expect("expected latitude and longitude");

                let (longitude, _) = line
                    .trim_start_matches(" lon=\"")
                    .split_once('"')
                    .expect("expected latitude and longitude");

                current_track_point.coordinates.latitude = latitude
                    .parse::<f64>()
                    .expect("expected valid float for latitude")
                    .into();
                current_track_point.coordinates.longitude = longitude
                    .parse::<f64>()
                    .expect("expected valid float for longitude")
                    .into();

                continue;
            }

            if line.starts_with("<ele>") {
                let (elevation, _) = line
                    .trim_start_matches("<ele>")
                    .split_once("</ele>")
                    .expect("expected elevation");

                current_track_point.elevation = elevation
                    .parse::<f64>()
                    .expect("expected valid float for elevation")
                    .into();

                continue;
            }

            if line.starts_with("<time>") {
                let (time, _) = line
                    .trim_start_matches("<time>")
                    .split_once("</time>")
                    .expect("expected time");

                current_track_point.time = UtcDateTime::parse(
                    time,
                    &time::format_description::well_known::Iso8601::PARSING,
                )
                .expect("expected valid ISO 8601 for time");

                continue;
            }

            if line == "</trkpt>" {
                track_points.push(current_track_point.clone());

                continue;
            }

            panic!("unexpected line in GPX trace!")
        }

        Self(track_points)
    }
}

impl IntoIterator for TrackSegment {
    type Item = TrackPoint;

    type IntoIter = <Vec<TrackPoint> as IntoIterator>::IntoIter;

    fn into_iter(self) -> Self::IntoIter {
        <Vec<TrackPoint> as IntoIterator>::into_iter(self.0)
    }
}
