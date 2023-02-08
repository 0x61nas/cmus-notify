use crate::cmus;
use std::path::Path;

/// Search in the track directory for the cover image or the lyrics(depending on the `regx`).
/// If the cover image or the lyrics is not found, search in the parent directory, and so on, until the max depth is reached.
/// If the cover image or the lyrics is not found, return `None`.
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
mod tests {
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
