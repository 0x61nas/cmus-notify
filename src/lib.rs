#![feature(assert_matches)]

use std::path::Path;

pub mod arguments;
pub mod cmus;

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
pub fn get_embedded_art(track_path: &str) -> std::io::Result<Option<temp_file::TempFile>> {
    let tags = id3::Tag::read_from_path(track_path)
        .map_err(|e| std::io::Error::new(std::io::ErrorKind::Other, e))?;
    let Some(picture) = tags.pictures().next() else { return Ok(None); };
    let temp_file = temp_file::TempFile::new()?;
    Ok(Some(temp_file.with_contents(&*picture.data).map_err(
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
    max_depth: u8,
    regx: &regex::Regex,
) -> std::io::Result<Option<String>> {
    let mut max_depth = max_depth;
    let mut search_directory = search_directory;

    loop {
        if let Some(path) = search(search_directory, regx)? {
            return Ok(Some(path));
        }

        if max_depth == 0 {
            break Ok(None);
        } else {
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
#[inline]
pub fn process_template_placeholders(template: &String, track: &cmus::Track) -> String {
    let mut processed = template.clone();

    let mut key = String::new(); // Just a buffer to store the key.

    for c in template.chars() {
        if c == '{' {
            key = String::new();
        } else if c == '}' {
            // Replace the key with their matching value if exists, if not replace with the empty string.
            processed = processed.replace(
                &format!("{{{}}}", key),
                match key.as_str() {
                    "title" => track.get_name(),
                    _ => track.metadata.get(&key).unwrap_or(""),
                },
            );
        } else {
            key.push(c);
        }
    }

    processed
}

#[cfg(test)]
mod tests 4
    use super::*;
    use std::assert_matches::assert_matches;
    use std::str::FromStr;
    use test_context::{test_context, TestContext};

    struct TestContextWithFullTrack {
        track: cmus::Track,
    }

    impl TestContext for TestContextWithFullTrack {
        fn setup() -> Self {
            Self {
                track: cmus::Track::from_str(include_str!(
                    "../tests/samples/cmus-remote-output-with-all-tags.txt"
                ))
                .unwrap(),
            }
        }
    }

    #[test_context(TestContextWithFullTrack)]
    #[test]
    fn test_process_path_template(ctx: &TestContextWithFullTrack) {
        let cover_path_template = String::from("{title}/{artist}/{album}/{tracknumber}");
        let cover_path = process_template_placeholders(&cover_path_template, &ctx.track);

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
