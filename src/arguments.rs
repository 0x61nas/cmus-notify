use clap::Parser;
use serde::{Deserialize, Serialize};

const NOTIFICATION_TIMEOUT: u8 = 5;
const NOTIFICATION_BODY: &str =
    "<b>Playing:</b> {title} from {album} \n\n <b>Artist:</b> {artist} - {year}";
const NOTIFICATION_SUMMARY: &str = "{artist} - {title}";
const NOTIFICATION_APP_NAME: &str = "C* Music Player";
const DEFAULT_MAX_DEPTH: u8 = 3;
const DEFAULT_INTERVAL_TIME: u64 = 1000; // 1000 ms

#[derive(Parser, Debug, Serialize, Deserialize)]
#[command(author, about, version, long_about = None)]
pub struct Arguments {
    /// The notification timeout, in seconds
    #[arg(short, long, default_value_t = NOTIFICATION_TIMEOUT)]
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
    #[arg(short = 'y', long, default_value = None)]
    pub lyrics_path: Option<String>,
    /// The maximum path depth to search for the cover and lyrics files,
    /// if the files are not found in the track's directory, or the directory specified by the `--cover-path`
    /// or `--lyrics-path`* options, the program will search in the parent directory,
    /// and so on, until the maximum depth is reached.
    #[arg(short, long, default_value_t = DEFAULT_MAX_DEPTH)]
    pub depth: u8,
    /// The name of the app to use for the notification.
    #[arg(short, long, default_value = NOTIFICATION_APP_NAME)]
    pub app_name: String,
    /// The summary of the notification.
    ///
    /// you can use the placeholder "{artist}" and "{album}" and "{title}" and "{track_number}" and
    /// "{disc_number}" and "{year}" and "{genre}" in the summary, they will be replaced with the corresponding metadata.
    /// but if the metadata is not available, the placeholder will be replaced with an empty string.
    /// e.g. "{artist} - {title}"
    #[arg(short, long, default_value = NOTIFICATION_SUMMARY)]
    pub summary: String,
    #[cfg(feature = "lyrics")]
    /// The body of the notification.
    ///
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
    #[arg(default_value = NOTIFICATION_BODY)]
    pub body: String,
    #[cfg(not(feature = "lyrics"))]
    /// The body of the notification.
    ///
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
    #[arg(default_value = NOTIFICATION_BODY)]
    pub body: String,
    /// The cmus-remote binary path, if not given, the program will search for it in the PATH environment variable.
    ///
    /// if you're using a custom package format like flatpak, or snap, you can give it the full run command (without any arguments),
    /// e.g. "flatpak run io.github.cmus.cmus", "snap run cmus"
    #[arg(short = 'b', long = "cmus-remote-bin", default_value = None)]
    pub cmus_remote_bin_path: Option<String>,
    /// The cmus socket address, if not given, the program will use the default socket address, which is "$XDG_RUNTIME_DIR/cmus-socket".
    #[arg(short = 'k', long = "cmus-socket", default_value = None)]
    pub cmus_socket_address: Option<String>,
    /// The cmus socket password, if any.
    #[arg(short = 'x', long = "socket-password", default_value = None)]
    pub cmus_socket_password: Option<String>,
    /// The interval to request the cmus status, in milliseconds.
    ///
    /// if you set it to 0, the program will only request the cmus status once and wait for the track duration time to make another request.
    ///  e.g. if the track duration is 3 minutes, the program will request the cmus status once, and wait for 3 minutes to make another request,
    /// and if the track changes to another track with 5 minutes duration, the program will request the cmus status once, and wait for 5 minutes to make another request at so on.
    ///
    /// this is useful if you have a potato computer, and you don't want to waste your CPU and battery,
    /// but it will make the notification a little bit stupid, if you change the track manually, the notification will not update until the track duration time is reached.
    ///
    /// but I recommend 1s, it's not too fast, and not too slow, and it will not waste your CPU and battery.
    #[arg(short = 'r', long, default_value_t = DEFAULT_INTERVAL_TIME)]
    pub interval: u64,
    /// Link the program with cmus, if the cmus are not running, the program will exit.
    #[arg(short = 'l', long)]
    pub link: bool,
    /// Force the program to use the external cover file, if available, and not even try to get the cover from the track's metadata.
    /// this is useful if you have a cover file with a better quality than the cover in the track's metadata.
    #[arg(short = 'u', long)]
    pub force_use_external_cover: bool,
    #[cfg(feature = "lyrics")]
    /// Fotrce the program to use the external lyrics file, if available, and not even try to get the lyrics from the track's metadata.
    #[arg(short = 'm', long)]
    pub force_use_external_lyrics: bool,
    /// No use the external cover file, even if it's available and the track's metadata doesn't have a cover.
    #[arg(short = 'n', long)]
    pub no_use_external_cover: bool,
    #[cfg(feature = "lyrics")]
    /// No use the external lyrics file, even if it's available and the track's metadata doesn't have a lyrics.
    #[arg(short = 'o', long)]
    pub no_use_external_lyrics: bool,
}

impl Default for Arguments {
    fn default() -> Self {
        Self {
            timeout: NOTIFICATION_TIMEOUT,
            persistent: false,
            show_track_cover: true,
            notification_static_icon: None,
            cover_path: None,
            lyrics_path: None,
            depth: DEFAULT_MAX_DEPTH,
            app_name: NOTIFICATION_APP_NAME.to_string(),
            summary: NOTIFICATION_SUMMARY.to_string(),
            body: NOTIFICATION_BODY.to_string(),
            cmus_remote_bin_path: None,
            cmus_socket_address: None,
            cmus_socket_password: None,
            interval: DEFAULT_INTERVAL_TIME,
            link: false,
            force_use_external_cover: false,
            #[cfg(feature = "lyrics")]
            force_use_external_lyrics: false,
            no_use_external_cover: false,
            #[cfg(feature = "lyrics")]
            no_use_external_lyrics: false,
        }
    }
}
