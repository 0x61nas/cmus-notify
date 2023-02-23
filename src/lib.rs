#![feature(assert_matches)]

use crate::cmus::{TemplateProcessor, Track};
#[cfg(feature = "debug")]
use log::{debug, info};
use std::path::Path;

pub mod cmus;
pub mod notification;
pub mod settings;

/// Extracts the first embedded picture from an ID3 tag of an Audio file.
///
/// # Arguments
///
/// * `track_path` - The path to the Audio file.
///
/// # Returns
///
/// Returns a `Result` containing a `TempFile` object with the contents of the extracted picture, or `None` if the MP3 file doesn't have any embedded pictures.
/// In case of error, the `Result` will contain an error value of type `std::io::Error`.
///
/// # Example
///
/// ```
/// # use cmus_notify::get_embedded_art;
/// let result = get_embedded_art("/path/to/track.mp3");
///
/// match result {
///     Ok(Some(temp_file)) => {
///         // Use the temp file...
///         temp_file.path();
///     },
///     Ok(None) => println!("Track does not have an embedded picture"),
///     Err(error) => println!("Error: {}", error),
/// }
/// ```
pub fn get_embedded_art(track_path: &str) -> std::io::Result<Option<image::DynamicImage>> {
    let tags = id3::Tag::read_from_path(track_path)
        .map_err(|e| std::io::Error::new(std::io::ErrorKind::Other, e))?;
    let Some(picture) = tags.pictures().next() else { return Ok(None); };
    Ok(Some(image::load_from_memory(&picture.data).map_err(
        |e| std::io::Error::new(std::io::ErrorKind::Other, e),
    )?))
}

/// Searches for a file that matches the provided regular expression in the specified search directory and its subdirectories.
///
/// # Arguments
///
/// * `search_directory` - The directory to start the search from.
/// * `max_depth` - The maximum number of parent directories to search in.
/// * `regx` - The regular expression to match against the file names.
///
/// # Returns
///
/// Returns a `Result` containing the absolute path of the first file that matches the regular expression, or `None` if no such file is found.
/// In case of error, the `Result` will contain an error value of type `std::io::Error`.
///
/// # Example
///
/// ```
/// # use regex::Regex;
/// # use cmus_notify::search_for;
/// let regx = Regex::new(r".\.lrc$").unwrap(); // Match .lrc files
/// let result = search_for("tests/samples/Owl City/Cinematic", 2, &regx);
///
/// assert_eq!(result.unwrap(), Some("tests/samples/Owl City/Cinematic/08 - Always.lrc".to_string()));
/// ```
pub fn search_for(
    search_directory: &str,
    mut max_depth: u8,
    regx: &regex::Regex,
) -> std::io::Result<Option<String>> {
    let mut search_directory = if Path::new(search_directory).is_file() {
        #[cfg(feature = "debug")]
        {
            info!("The provided search directory is a file, searching in the parent directory.");
        }
        let Some(parent) = Path::new(search_directory).parent() else { return Ok(None); };
        let Some(parent) = parent.to_str() else { return Ok(None); };
        parent
    } else {
        search_directory
    };
    #[cfg(feature = "debug")]
    {
        info!("Searching for a file that matches the regular {regx:?} expression in \"{search_directory}\" and its subdirectories.");
        info!("Max depth: {max_depth}");
    }

    loop {
        if let Some(path) = search(search_directory, regx)? {
            return Ok(Some(path));
        }

        if max_depth == 0 {
            break Ok(None);
        } else {
            #[cfg(feature = "debug")]
            {
                info!("Could not find a file that matches the regular {regx:?} expression in \"{search_directory}\", searching in the parent directory.");
                info!("Max depth: {max_depth}");
            }
            // If the max depth is not reached, search in the parent directory.
            max_depth -= 1;
            search_directory = {
                let Some(parent) = Path::new(search_directory).parent() else { return Ok(None); };
                let Some(parent) = parent.to_str() else { return Ok(None); };
                parent
            };
        }
    }
}

/// The cover of a track.
#[derive(Debug, PartialEq)]
pub enum TrackCover {
    /// The cover is embedded in the track.
    /// The `TempFile` object contains the contents of the embedded picture.
    Embedded(image::DynamicImage),
    /// The cover is an external file.
    /// The `String` contains the absolute path of the external file.
    External(image::DynamicImage),
    /// The track does not have a cover.
    None,
}

impl TrackCover {
    pub fn set_notification_image(&self, notification: &mut notify_rust::Notification) {
        use TrackCover::*;
        match self {
            Embedded(cover) | External(cover) => {
                #[cfg(feature = "debug")]
                debug!("Setting the cover as the notification image.");
                let Ok(image) = notify_rust::Image::try_from(cover.clone()) else { return; };
                notification.image_data(image);
            }
            None => {
                #[cfg(feature = "debug")]
                debug!("The track does not have a cover.");
                // reset the notification image
                notification.image_path("");
            }
        }
    }
}

/// Returns the cover of a track.
/// If the track has an embedded cover, and `force_use_external_cover` is `false`, the embedded cover will be returned.
/// If the track does not have an embedded cover, and `no_use_external_cover` is `false`, the function will search for an external cover.
/// If the track has an embedded cover, and `force_use_external_cover` is `true`, the function will search for an external cover.
#[inline]
pub fn track_cover(
    mut path: String,
    track_name: &str,
    max_depth: u8,
    force_use_external_cover: bool,
    no_use_external_cover: bool,
) -> TrackCover {
    if !force_use_external_cover {
        #[cfg(feature = "debug")]
        info!("Trying to get the embedded cover of \"{path}\".");
        if let Ok(Some(cover)) = get_embedded_art(&path) {
            return TrackCover::Embedded(cover);
        }
    }

    if !no_use_external_cover {
        let (Ok(regx), path) = (match path.split("/").last() {
            Some(last_pat) if last_pat.contains("r#") => {
                (regex::Regex::new(&*last_pat.replace("r#", "")),
                // Remove the last part of the path
                path.remove(path.len() - last_pat.len() - 1).to_string())
            }
            _ => (regex::Regex::new(&format!(r"({track_name}).*\.(jpg|jpeg|png|gif)$")), path),
        }) else {
            #[cfg(feature = "debug")]
            info!("Could not get the cover.");
            return TrackCover::None;
        };
        #[cfg(feature = "debug")]
        info!("Trying to get the external cover of \"{path}\".");
        if let Ok(Some(cover)) = search_for(
            &path,
            max_depth,
            &regx,
        ) {
            #[cfg(feature = "debug")]
            info!("Found the external cover \"{cover}\".");
            let Ok(cover) = image::open(cover) else {
                #[cfg(feature = "debug")]
                info!("Could not open the external cover.");
                return TrackCover::None;
            };
            return TrackCover::External(cover);
        }
    }

    #[cfg(feature = "debug")]
    info!("Could not get the cover.");

    TrackCover::None
}

#[inline]
fn search(search_directory: &str, matcher: &regex::Regex) -> std::io::Result<Option<String>> {
    for entry in std::fs::read_dir(search_directory)? {
        let Ok(entry) = entry else { continue; };
        let Ok(file_type) = entry.file_type() else { continue; };
        if file_type.is_file() {
            let Ok(file_name) = entry.file_name().into_string() else { continue; };
            // Check if the file name matches the regular expression.
            if matcher.is_match(&file_name) {
                let path = entry.path();
                let Some(path) = path.to_str() else { continue; };
                return Ok(Some(path.to_string()));
            }
        }
    }
    Ok(None)
}

/// Replace all the placeholders in the template with their matching value.
#[inline(always)]
pub fn process_template_placeholders(
    template: String,
    track: &Track,
    player_settings: &cmus::player_settings::PlayerSettings,
) -> String {
    let res = track.process(template);
    player_settings.process(res)
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::assert_matches::assert_matches;
    use std::str::FromStr;
    use test_context::{test_context, TestContext};
    use crate::cmus::player_settings::PlayerSettings;

    struct TestContextWithFullTrack {
        track: Track,
        player_settings: PlayerSettings,
    }

    impl TestContext for TestContextWithFullTrack {
        fn setup() -> Self {
            Self {
                track: cmus::Track::from_str(include_str!(
                    "../tests/samples/cmus-remote-output-with-all-tags.txt"
                ))
                .unwrap(),
                player_settings: PlayerSettings::from_str(include_str!(
                    "../tests/samples/player_settings_mode-artist_vol-46_repeat-false_repeat_current-false_shuffle-tracks.txt"
                ))
                .unwrap(),
            }
        }
    }

    #[test_context(TestContextWithFullTrack)]
    #[test]
    fn test_process_path_template(ctx: &TestContextWithFullTrack) {
        let cover_path_template = String::from("{title}/{artist}/{album}/{tracknumber}");
        let cover_path = process_template_placeholders(cover_path_template, &ctx.track, &ctx.player_settings);

        assert_eq!(
            cover_path,
            "Photograph/Alex Goot/Alex Goot & Friends, Vol. 3/8"
        );
    }

    #[test]
    fn test_search_for_cover_with_the_cover_key_world() {
        let cover_path = search_for(
            "tests/samples/Owl City/Cinematic/cover",
            1,
            &regex::Regex::new(r"cover|.\.jpg|.\.png").unwrap(),
        );

        assert_matches!(cover_path, Ok(Some(_)));
        assert_eq!(
            cover_path.unwrap().unwrap(),
            "tests/samples/Owl City/Cinematic/cover/cover.jpg"
        );
    }

    #[test]
    fn test_search_for_cover_without_the_cover_key_world() {
        let cover_path = search_for(
            "tests/samples/Owl City/Cinematic/cover",
            1,
            &regex::Regex::new(r".\.jpg|.\.png").unwrap(),
        );

        assert_matches!(cover_path, Ok(Some(_)));
        assert_eq!(
            cover_path.unwrap().unwrap(),
            "tests/samples/Owl City/Cinematic/cover/cover.jpg"
        );
    }

    #[test]
    fn test_search_for_cover_without_the_cover_key_world_and_jpg() {
        let cover_path = search_for(
            "tests/samples/Owl City/Cinematic/cover",
            1,
            &regex::Regex::new(r".\.png").unwrap(),
        );

        assert_matches!(cover_path, Ok(Some(_)));
        assert_eq!(
            cover_path.unwrap().unwrap(),
            "tests/samples/Owl City/Cinematic/cover/cover.png"
        );
    }

    #[test]
    fn test_search_for_lrc_file_started_from_the_cover_directory() {
        let lrc_path = search_for(
            "tests/samples/Owl City/Cinematic/cover",
            1,
            &regex::Regex::new(r".\.lrc").unwrap(),
        );

        assert_matches!(lrc_path, Ok(Some(_)));
        assert_eq!(
            lrc_path.unwrap().unwrap(),
            "tests/samples/Owl City/Cinematic/08 - Always.lrc"
        );
    }

    #[test]
    fn test_search_for_not_exits_file() {
        let result = search_for(
            "tests/samples/Owl City/Cinematic/cover",
            3,
            &regex::Regex::new(r".\.mp3").unwrap(),
        );

        assert_matches!(result, Ok(None));
    }
}
