use crate::cmus;
use std::path::Path;

/// Search in the track directory for the cover image or the lyrics(depending on the `regx`).
/// If the cover image or the lyrics is not found, search in the parent directory, and so on, until the max depth is reached.
/// If the cover image or the lyrics is not found, return `None`.
pub fn search_for(
    search_directory: &str,
    max_depth: u8,
    regx: &str,
) -> Result<Option<String>, String> {
    // Search in the track directory.
    for entry in std::fs::read_dir(search_directory).map_err(|e| e.to_string())? {
        let Ok(entry) = entry else { continue; };
        let Ok(file_type) = entry.file_type() else { continue; };
        if file_type.is_file() {
            let Ok(file_name) = entry.file_name().into_string() else { continue; };
            // Check if the file name matches the regular expression.
            let matcher = regex::Regex::new(regx).map_err(|e| e.to_string())?;
            if matcher.is_match(&file_name) {
                let path = entry.path();
                let Some(path) = path.to_str() else { continue; };
                return Ok(Some(path.to_string()));
            }
        }
    }
    // If the max depth is reached, return `None`.
    if max_depth == 0 {
        Ok(None)
    } else {
        // If the max depth is not reached, search in the parent directory (recursively).
        let Some(parent) = Path::new(search_directory).parent() else { return Ok(None); };
        let Some(parent) = parent.to_str() else { return Ok(None); };
        search_for(parent, max_depth - 1, regx)
    }
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
            "tests/samples/Owl City/Cinematic",
            1,
            r"cover|.\.jpg|.\.png",
        );

        assert_matches!(cover_path, Ok(Some(_)));
        assert_eq!(
            cover_path.unwrap().unwrap(),
            "tests/samples/Owl City/Cinematic/cover.jpg"
        );
    }

    #[test]
    fn test_search_for_cover_without_the_cover_key_world() {
        let cover_path = search_for("tests/samples/Owl City/Cinematic", 1, r".\.jpg|.\.png");

        assert_matches!(cover_path, Ok(Some(_)));
        assert_eq!(
            cover_path.unwrap().unwrap(),
            "tests/samples/Owl City/Cinematic/cover.jpg"
        );
    }
}
