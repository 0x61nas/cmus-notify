use clap::{Parser};

#[derive(Parser, Debug)]
#[command(author, about, version, long_about = None)]
pub struct Arguments {
    /// The notification timeout, in seconds
    #[arg(short, long, default_value = "5")]
    pub timeout: u8,
    /// Make the notification persistent, i.e. not disappear after a timeout (you can dismiss it manually)
    #[arg(short, long)]
    pub persistent: bool,
    /// Show the track cover in the notification, if available
    #[arg(short = 'c', long = "cover")]
    pub show_track_cover: bool,
    /// The static icon to use for the notification, it not effective if the track cover is shown,
    /// but if the cover is not available or you disabled it, this icon will be used.
    ///
    /// you can give it the full path to an image file or a name of an icon from the current icon theme
    /// (e.g. "audio-x-generic" or "spotify-client")
    #[arg(short = 'i', long = "icon", default_value = None)]
    pub notification_static_icon: Option<String>,
    /// The path to look for the cover image, if not given, the cover will be searched in the track's directory
    /// for an image file with the name "cover".
    ///
    /// You can use the placeholder "{artist}" and "{album}" and "{title}" and "{track_number}" and
    /// "{disc_number}" and "{year}" and "{genre}" in the path, they will be replaced with the corresponding metadata.
    /// but if the metadata is not available, the placeholder will be replaced with an empty string.
    /// And you can use the simple glob pattern `*` to match any character.
    /// e.g. "covers/{artist}/{album}/cover.*", "covers/{artist}/{album}/*",
    ///
    /// If you not specify the full path, the cover will be started from the track's directory.
    #[arg(short = 'w', long = "cover-path", default_value = None)]
    pub cover_path: Option<String>,
    #[cfg(feature = "lyrics")]
    /// The lyrics file path, if not given, the lyrics will be searched in the track's directory
    /// for a text file with the name "lyrics", or with the same name as the track.
    ///
    /// You can use the placeholder "{artist}" and "{album}" and "{title}" and "{track_number}" and
    /// "{disc_number}" and "{year}" and "{genre}" in the path, they will be replaced with the corresponding metadata.
    /// but if the metadata is not available, the placeholder will be replaced with an empty string.
    /// And you can use the simple glob pattern `*` to match any character.
    /// e.g. "lyrics/{artist}/{album}/{title}.lrc", "lyrics/{artist}/{album}/*",
    ///
    /// If you not specify the full path, the lyrics will be started from the track's directory.
    #[arg(short, long, default_value = None)]
    pub lyrics_path: Option<String>,
    /// The maximum path depth to search for the cover and lyrics files,
    /// if the files are not found in the track's directory, or the directory specified by the `--cover-path`
    /// or `--lyrics-path`* options, the program will search in the parent directory,
    /// and so on, until the maximum depth is reached.
    #[arg(short, long, default_value = "3")]
    pub depth: u8,
    /// The name of the app to use for the notification.
    #[arg(short, long, default_value = "C* Music Player")]
    pub app_name: String,
    /// The summary of the notification.
    ///
    /// you can use the placeholder "{artist}" and "{album}" and "{title}" and "{track_number}" and
    /// "{disc_number}" and "{year}" and "{genre}" in the summary, they will be replaced with the corresponding metadata.
    /// but if the metadata is not available, the placeholder will be replaced with an empty string.
    /// e.g. "{artist} - {title}"
    #[arg(short, long, default_value = "{artist} - {title}")]
    pub summary: String,
    #[cfg(feature = "lyrics")]
    /// The body of the notification.
    /// you can use the placeholder "{artist}" and "{album}" and "{title}" and "{track_number}" and
    /// "{disc_number}" and "{year}" and "{genre}" in the body, they will be replaced with the corresponding metadata.
    /// but if the metadata is not available, the placeholder will be replaced with an empty string.
    ///
    /// And you can use the placeholder "{lyrics}" to show the lyrics of the track, if available.
    /// But if you use this placeholder, the notification will be persistent, and you need to dismiss it manually tow times.
    ///
    /// Also you can use the placeholder "{progress}" to show the progress of the track, in the format "00:00 / 00:00".
    ///
    /// Also you can use the placeholder "{progress_bar}" to show the progress bar of the track.
    ///
    /// Like the "{lyrics}" placeholder, if you use the "{progress}" or "{progress_bar}" placeholder,
    /// the notification will be persistent, and you need to dismiss it manually tow times.
    ///
    /// Also you can use the simple html markup, if your notification server supports it.
    #[arg(default_value = "<b>Playing:</b> {title} from {album} \n\n <b>Artist:</b> {artist} - {year}")]
    pub body: String,
    #[cfg(not(feature = "lyrics"))]
    /// The body of the notification.
    /// you can use the placeholder "{artist}" and "{album}" and "{title}" and "{track_number}" and
    /// "{disc_number}" and "{year}" and "{genre}" in the body, they will be replaced with the corresponding metadata.
    /// but if the metadata is not available, the placeholder will be replaced with an empty string.
    ///
    /// And you can use the placeholder "{progress}" to show the progress of the track, in the format "00:00 / 00:00".
    /// Also you can use the placeholder "{progress_bar}" to show the progress bar of the track.
    ///
    /// But if you use the "{progress}" or "{progress_bar}" placeholder,
    /// the notification will be persistent, and you need to dismiss it manually tow times.
    ///
    /// Also you can use the simple html markup, if your notification server supports it.
    #[arg(default_value = "<b>Playing:</b> {title} from {album} \n\n <b>Artist:</b> {artist} - {year}")]
    pub body: String,
}
