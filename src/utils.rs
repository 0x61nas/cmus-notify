use crate::cmus;

pub fn search_for_cover_image(search_directory: &str, max_depth: u8) -> Option<String> {



    None
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
