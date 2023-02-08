use std::path::Path;
use crate::cmus;

/// Search in the track directory for the cover image or the lyrics(depending on the `regx`).
/// If the cover image or the lyrics is not found, search in the parent directory, and so on, until the max depth is reached.
/// If the cover image or the lyrics is not found, return `None`.
pub fn search_for(search_directory: &str, max_depth: u8, regx: &[&str]) -> std::io::Result<Option<String>> {
    // Search in the track directory.
    for entry in std::fs::read_dir(search_directory)? {
        if let Ok(entry) = entry {
            if let Ok(file_type) = entry.file_type() {
                if file_type.is_file() {
                    let Ok(file_name) = entry.file_name().into_string() else { continue; };
                    // Check if the file name matches any of the regular expressions.
                    if regx.iter().any(|&regx| file_name.contains(regx)) {
                        let path = entry.path();
                        let Some(path) = path.to_str() else { continue; };
                        return Ok(Some(path.to_string()));
                    }
                }
            }
        }
    }
    // If the max depth is reached, return `None`.
    if max_depth == 0 {
        Ok(None)
    } else { // If the max depth is not reached, search in the parent directory (recursively).
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
        } else if c == '}' { // Replace the key with their matching value if exists, if not replace with the empty string.
            processed = processed.replace(&format!("{{{}}}", key), match key.as_str() {
                "title" => track.get_name(),
                _ => track.metadata.get(&key).unwrap_or(""),
            });
        } else {
            key.push(c);
        }
    }

    processed
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::str::FromStr;
    use test_context::{test_context, TestContext};

    struct TestContextWithFullTrack {
        track: cmus::Track,
    }

    impl TestContext for TestContextWithFullTrack {
        fn setup() -> Self {
            Self {
                track: cmus::Track::from_str(include_str!("../tests/samples/cmus-remote-output-with-all-tags.txt")).unwrap()
            }
        }
    }

    #[test_context(TestContextWithFullTrack)]
    #[test]
    fn test_process_path_template(ctx: &TestContextWithFullTrack) {
        let cover_path_template = String::from("{title}/{artist}/{album}/{tracknumber}");
        let cover_path = process_template_placeholders(&cover_path_template, &ctx.track);

        assert_eq!(cover_path, "Photograph/Alex Goot/Alex Goot & Friends, Vol. 3/8");
    }
}
