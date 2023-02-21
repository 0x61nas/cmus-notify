pub mod events;
pub mod player_settings;
pub mod query;

use crate::cmus::query::CmusQueryResponse;
#[cfg(feature = "debug")]
use log::{debug, info};
use parse_display::Display;
use std::collections::HashMap;
use std::fmt::Debug;
use std::num::ParseIntError;
use std::str::FromStr;
use thiserror::Error;
use typed_builder::TypedBuilder;

pub trait TemplateProcessor {
    fn process(&self, template: String) -> String;

    /// Returns a vector of keys found in the template.
    /// The keys are the strings between curly braces.
    fn get_keys(template: &str) -> Vec<String> {
        let mut keys = Vec::new(); // Just a buffer to store the keys.
        let mut key = String::new(); // Just a buffer to build the key.

        for c in template.chars() {
            if c == '{' {
                key = String::new();
            } else if c == '}' {
                #[cfg(feature = "debug")]
                debug!("Found key: {}", key);
                keys.push(key.clone());
            } else {
                key.push(c);
            }
        }

        #[cfg(feature = "debug")]
        debug!("Found keys: {:?}", keys);

        keys
    }
}

#[derive(Debug, PartialEq, Default, Clone)]
pub struct TrackMetadata {
    tags: HashMap<String, String>,
}

#[derive(Display, Debug, PartialEq, Default, Clone)]
pub enum TrackStatus {
    Playing,
    Paused,
    #[default]
    Stopped,
}

#[derive(Debug, TypedBuilder, PartialEq, Default, Clone)]
pub struct Track {
    pub status: TrackStatus,
    pub path: String,
    pub metadata: TrackMetadata,
    pub duration: u32,
    pub position: u32,
}

#[derive(Debug, PartialEq, Error)]
pub enum CmusError {
    #[error("Cmus running error: {0}")]
    CmusRunningError(String),
    #[error("Unknown status")]
    UnknownStatus,
    #[error("No status")]
    NoStatus,
    #[error("Empty path")]
    EmptyPath,
    #[error("Duration error: {0}")]
    DurationError(String),
    #[error("Position error: {0}")]
    PositionError(String),
    #[error("Unknown error: {0}")]
    UnknownError(String),
    #[error("Unknown AAA mode: {0}")]
    UnknownAAAMode(String),
    #[error("Unknown shuffle mode: {0}")]
    UnknownShuffleMode(String),
    #[error("No events")]
    NoEvents,
}

impl TemplateProcessor for Track {
    /// Process the template with the track metadata.
    /// The template is a string with placeholders that will be replaced with the track metadata.
    /// The unknown placeholders will be skipped (don't replaced with anything, because they are maybe placeholders for player settings).
    #[inline(always)]
    fn process(&self, template: String) -> String {
        #[cfg(feature = "debug")]
        {
            info!("Processing the template placeholders.");
            debug!("Template: {template}");
            debug!("Track: {self:?}");
        }
        let mut processed = template.to_string();

        Self::get_keys(template.as_str()).iter().for_each(|key| {
            #[cfg(feature = "debug")]
            debug!("Replacing the placeholder {{{key}}} with its matching value.");
            // Replace the key with their matching value if exists, if not replace with the empty string.
            let status = self.status.to_string();
            if let Some(value) = match key.as_str() {
                "status" => Some(status.as_str()),
                "title" => Some(self.get_name()),
                _ => self.metadata.get(&key),
            } {
                processed = processed.replace(&format!("{{{key}}}"), value);
            }
        });

        #[cfg(feature = "debug")]
        debug!("Processed template: {processed}");

        processed
    }
}

impl FromStr for TrackStatus {
    type Err = CmusError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "playing" => Ok(TrackStatus::Playing),
            "paused" => Ok(TrackStatus::Paused),
            "stopped" => Ok(TrackStatus::Stopped),
            _ => Err(CmusError::UnknownStatus),
        }
    }
}

impl FromStr for Track {
    type Err = CmusError;

    /// Creates a `Track` from the output of `cmus-remote -Q`.
    ///
    /// Pares the first 4 lines.
    /// The first line is the status, the second is the path, the third is the duration, and the fourth is the position.
    /// The rest of the lines are tags, and the player settings, so we'll send them to `TrackMetadata::parse`, to get the tags.
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        #[cfg(feature = "debug")]
        info!("Parsing track from string: {}", s);

        let mut lines = s.lines();

        Ok(Track::builder()
            .status(TrackStatus::from_str(
                lines
                    .next()
                    .ok_or(CmusError::NoStatus)?
                    .split_once(' ')
                    .ok_or(CmusError::NoStatus)?
                    .1,
            )?)
            .path(
                lines
                    .next()
                    .ok_or(CmusError::EmptyPath)?
                    .split_once(' ')
                    .ok_or(CmusError::EmptyPath)?
                    .1
                    .to_string(),
            )
            .duration(
                lines
                    .next()
                    .ok_or(CmusError::DurationError("Missing duration".to_string()))?
                    .split_once(' ')
                    .ok_or(CmusError::DurationError("Empty duration".to_string()))?
                    .1
                    .parse()
                    .map_err(|e: ParseIntError| CmusError::DurationError(e.to_string()))?,
            )
            .position(
                lines
                    .next()
                    .ok_or(CmusError::PositionError("Missing position".to_string()))?
                    .split_once(' ')
                    .ok_or(CmusError::PositionError("Empty position".to_string()))?
                    .1
                    .parse()
                    .map_err(|e: ParseIntError| CmusError::PositionError(e.to_string()))?,
            )
            .metadata(TrackMetadata::parse(lines))
            .build())
    }
}

impl TrackMetadata {
    /// Parse the tags from the rest of `cmus-remote -Q` output.
    /// This function will assume you processed the first 4 lines, and remove them from the iterator.
    ///
    /// and also assume the all tags is contained in the iterator.
    fn parse<'a>(mut lines: impl Iterator<Item = &'a str> + Debug) -> Self {
        #[cfg(feature = "debug")]
        info!("Parsing track metadata from lines: {:?}", lines);

        let mut tags = HashMap::new();

        while let Some(line) = lines.next() {
            #[cfg(feature = "debug")]
            debug!("Parsing line: {}", line);
            match line.trim().split_once(' ') {
                Some(("tag", rest)) => {
                    let Some((key, value)) = rest.split_once(' ') else {
                        continue; // Ignore lines that don't have a key and a value.
                    };
                    #[cfg(feature = "debug")]
                    debug!("Inserting tag: {} = {}", key, value);
                    tags.insert(key.to_string(), value.to_string());
                }
                _ => break, // We've reached the end of the tags.
            }
        }

        TrackMetadata { tags }
    }

    pub fn get(&self, key: &str) -> Option<&str> {
        self.tags.get(key).map(|s| s.as_str())
    }
}

impl Track {
    /// Returns the name of the track.
    ///
    /// This is the title, if it exists, otherwise it's the file name without the extension.
    pub fn get_name(&self) -> &str {
        self.metadata.get("title").unwrap_or_else(|| {
            self.path
                .split('/')
                .last()
                .unwrap_or("")
                .split_once(".")
                .unwrap_or(("", ""))
                .0
        })
    }
}

/// Make a status request to cmus.
/// And collect the output, and parse it into a `CmusQueryResponse`.
/// If the cmus is not running, or the socket is not available, this function will return an error.
#[inline]
pub fn ping_cmus(
    query_command: &mut std::process::Command,
) -> Result<CmusQueryResponse, CmusError> {
    // Just run the command, and collect the output.
    let output = query_command
        .output()
        .map_err(|e| CmusError::CmusRunningError(e.to_string()))?;

    if !output.status.success() {
        return Err(CmusError::CmusRunningError(
            String::from_utf8(output.stderr).map_err(|e| CmusError::UnknownError(e.to_string()))?,
        ));
    }

    let output =
        String::from_utf8(output.stdout).map_err(|e| CmusError::UnknownError(e.to_string()))?;

    CmusQueryResponse::from_str(&output).map_err(|e| CmusError::UnknownError(e.to_string()))
}

/// Build the query command.
/// This function it should call only one time entire the program life time, So it makes sense to make it inline.
/// This function will return a `std::process::Command` that can be used to query cmus, you should store it in a variable :).
#[inline(always)]
pub fn build_query_command(
    cmus_remote_bin: &str,
    socket_addr: &Option<String>,
    socket_pass: &Option<String>,
) -> std::process::Command {
    let cmd_arr = cmus_remote_bin.split_whitespace().collect::<Vec<_>>();
    let mut command = std::process::Command::new(cmd_arr[0]);

    // If there are more than 1 slice, then add the rest of the slices as arguments.
    if cmd_arr.len() > 1 {
        command.args(&cmd_arr[1..]);
    }

    if let Some(socket_addr) = socket_addr {
        command.arg("--server").arg(socket_addr); // Use the socket instead of the default socket.
    }

    if let Some(socket_pass) = socket_pass {
        command.arg("--passwd").arg(socket_pass);
    }

    command.arg("-Q");

    command
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::assert_matches::assert_matches;

    const OUTPUT_WITH_ALL_TAGS: &str =
        include_str!("../../tests/samples/cmus-remote-output-with-all-tags.txt");

    const SOME_TAGS: &str = r#"tag artist Alex Goot
        tag album Alex Goot & Friends, Vol. 3
        tag title Photograph
        tag date 2014
        tag genre Pop
        tag discnumber 1
        tag tracknumber 8
        tag albumartist Alex Goot
        tag replaygain_track_gain -9.4 dB
        tag composer Chad Kroeger
        tag label mudhutdigital.com
        tag publisher mudhutdigital.com
        tag bpm 146
        set aaa_mode artist
        ..."#;

    #[test]
    fn test_create_track_from_str() {
        let track = Track::from_str(OUTPUT_WITH_ALL_TAGS);

        assert_matches!(track, Ok(_));

        let track = track.unwrap();

        assert_eq!(track.status, TrackStatus::Playing);
        assert_eq!(track.path, "/mnt/Data/Music/FLAC/Alex Goot/Alex Goot - Alex Goot & Friends, Vol. 3/08 - Photograph.mp3");
        assert_eq!(track.duration, 284);
        assert_eq!(track.position, 226);
        assert_eq!(
            track.metadata.tags.get("artist"),
            Some(&"Alex Goot".to_string())
        );
        assert_eq!(
            track.metadata.tags.get("album"),
            Some(&"Alex Goot & Friends, Vol. 3".to_string())
        );
        assert_eq!(
            track.metadata.tags.get("title"),
            Some(&"Photograph".to_string())
        );
        assert_eq!(track.metadata.tags.get("date"), Some(&"2014".to_string()));
        assert_eq!(track.metadata.tags.get("genre"), Some(&"Pop".to_string()));
        assert_eq!(
            track.metadata.tags.get("discnumber"),
            Some(&"1".to_string())
        );
        assert_eq!(
            track.metadata.tags.get("tracknumber"),
            Some(&"8".to_string())
        );
        assert_eq!(
            track.metadata.tags.get("albumartist"),
            Some(&"Alex Goot".to_string())
        );
        assert_eq!(
            track.metadata.tags.get("replaygain_track_gain"),
            Some(&"-9.4 dB".to_string())
        );
        assert_eq!(
            track.metadata.tags.get("composer"),
            Some(&"Chad Kroeger".to_string())
        );
        assert_eq!(
            track.metadata.tags.get("label"),
            Some(&"mudhutdigital.com".to_string())
        );
        assert_eq!(
            track.metadata.tags.get("publisher"),
            Some(&"mudhutdigital.com".to_string())
        );
        assert_eq!(track.metadata.tags.get("bpm"), Some(&"146".to_string()));
    }

    #[test]
    fn test_parse_metadata_from_the_string() {
        let metadata = TrackMetadata::parse(SOME_TAGS.lines());

        assert_eq!(metadata.tags.get("artist"), Some(&"Alex Goot".to_string()));
        assert_eq!(
            metadata.tags.get("album"),
            Some(&"Alex Goot & Friends, Vol. 3".to_string())
        );
        assert_eq!(metadata.tags.get("title"), Some(&"Photograph".to_string()));
        assert_eq!(metadata.tags.get("date"), Some(&"2014".to_string()));
        assert_eq!(metadata.tags.get("genre"), Some(&"Pop".to_string()));
        assert_eq!(metadata.tags.get("discnumber"), Some(&"1".to_string()));
        assert_eq!(metadata.tags.get("tracknumber"), Some(&"8".to_string()));
        assert_eq!(
            metadata.tags.get("albumartist"),
            Some(&"Alex Goot".to_string())
        );
        assert_eq!(
            metadata.tags.get("replaygain_track_gain"),
            Some(&"-9.4 dB".to_string())
        );
        assert_eq!(
            metadata.tags.get("composer"),
            Some(&"Chad Kroeger".to_string())
        );
        assert_eq!(
            metadata.tags.get("label"),
            Some(&"mudhutdigital.com".to_string())
        );
        assert_eq!(
            metadata.tags.get("publisher"),
            Some(&"mudhutdigital.com".to_string())
        );
        assert_eq!(metadata.tags.get("bpm"), Some(&"146".to_string()));
    }

    #[test]
    fn test_build_the_query_command_with_no_custom_socket_and_no_pass() {
        let command = build_query_command("cmus-remote", &None, &None);

        assert_eq!(command.get_program(), "cmus-remote");
        assert_eq!(command.get_args().collect::<Vec<_>>(), &["-Q"]);
    }

    #[test]
    fn test_build_the_query_command_with_custom_socket_and_no_pass() {
        let command =
            build_query_command("cmus-remote", &Some("/tmp/cmus-socket".to_string()), &None);

        assert_eq!(command.get_program(), "cmus-remote");
        assert_eq!(
            command.get_args().collect::<Vec<_>>(),
            &["--server", "/tmp/cmus-socket", "-Q"]
        );
    }

    #[test]
    fn test_build_the_query_command_with_custom_socket_and_pass() {
        let command = build_query_command(
            "cmus-remote",
            &Some("/tmp/cmus-socket".to_string()),
            &Some("pass".to_string()),
        );

        assert_eq!(command.get_program(), "cmus-remote");
        assert_eq!(
            command.get_args().collect::<Vec<_>>(),
            &["--server", "/tmp/cmus-socket", "--passwd", "pass", "-Q"]
        );
    }

    #[test]
    fn test_build_the_query_command_with_custom_bin_path() {
        let command = build_query_command("flatpak run io.github.cmus.cmus", &None, &None);

        assert_eq!(command.get_program(), "flatpak");
        assert_eq!(
            command.get_args().collect::<Vec<_>>(),
            &["run", "io.github.cmus.cmus", "-Q"]
        );
    }
}
