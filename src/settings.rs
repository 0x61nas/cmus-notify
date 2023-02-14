use clap::Parser;
use serde::{Deserialize, Serialize};

const NOTIFICATION_TIMEOUT: u8 = 5;
const NOTIFICATION_BODY: &str =
    "<b>Playing:</b> {title} from {album} \n\n <b>Artist:</b> {artist} - {year}";
const NOTIFICATION_SUMMARY: &str = "{artist} - {title}";
const NOTIFICATION_APP_NAME: &str = "C* Music Player";
const DEFAULT_MAX_DEPTH: u8 = 3;
const DEFAULT_INTERVAL_TIME: u64 = 1000; // 1000 ms
const DEFAULT_VOLUME_CHANGE_NOTIFICATION_BODY: &str = "Volume changed to {volume}%";
const DEFAULT_VOLUME_CHANGE_NOTIFICATION_SUMMARY: &str = "Volume changed";
const DEFAULT_VOLUME_CHANGE_NOTIFICATION_TIMEOUT: u8 = 1;
const DEFAULT_SHUFFLE_NOTIFICATION_BODY: &str = "Shuffle mode changed to {shuffle}";
const DEFAULT_SHUFFLE_NOTIFICATION_SUMMARY: &str = "Shuffle mode changed";
const DEFAULT_SHUFFLE_NOTIFICATION_TIMEOUT: u8 = 1;
const DEFAULT_REPEAT_NOTIFICATION_BODY: &str = "Repeat mode changed to {repeat}";
const DEFAULT_REPEAT_NOTIFICATION_SUMMARY: &str = "Repeat mode changed";
const DEFAULT_REPEAT_NOTIFICATION_TIMEOUT: u8 = 1;
const DEFAULT_AAAMODE_NOTIFICATION_BODY: &str = "AAA mode changed to {aaa_mode}";
const DEFAULT_AAAMODE_NOTIFICATION_SUMMARY: &str = "AAA mode changed";
const DEFAULT_AAAMODE_NOTIFICATION_TIMEOUT: u8 = 1;
#[cfg(feature = "lyrics")]
const DEFAULT_LYRICS_NOTIFICATION_BODY: &str = "{lyrics}";
#[cfg(feature = "lyrics")]
const DEFAULT_LYRICS_NOTIFICATION_SUMMARY: &str = "Lyrics";


#[derive(Parser, Debug, Serialize, Deserialize)]
#[command(author, about, version, long_about = None)]
pub struct Settings {
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
    /// Show the player notifications, like if you change the shuffle mode, or the repeat mode, or the volume.
    #[arg(short = 'g', long)]
    pub show_player_notifications: bool,
    /// The volume change notification body.
    /// you can use the placeholders like "{volume}" in the body, it will be replaced with the shuffle mode.
    ///
    /// If you leave it empty, the notification will not be shown.
    #[arg(short = 'B', long, default_value = DEFAULT_VOLUME_CHANGE_NOTIFICATION_BODY)]
    pub volume_notification_body: String,
    /// The volume change notification summary.
    #[arg(short = 'E', long, default_value = DEFAULT_VOLUME_CHANGE_NOTIFICATION_SUMMARY)]
    pub volume_notification_summary: String,
    /// The time out of the volume change notification, in seconds.
    #[arg(short = 'T', long, default_value_t = DEFAULT_VOLUME_CHANGE_NOTIFICATION_TIMEOUT)]
    pub volume_notification_timeout: u8,
    /// The shuffle mode change notification body.
    /// you can use the placeholders like "{shuffle}" in the body, it will be replaced with the shuffle mode.
    ///
    /// If you leave it empty, the notification will not be shown.
    #[arg(short = 'S', long, default_value = DEFAULT_SHUFFLE_NOTIFICATION_BODY)]
    pub shuffle_notification_body: String,
    /// The shuffle mode change notification summary.
    /// you can use the placeholders like "{shuffle}" in the summary, it will be replaced with the shuffle mode.
    #[arg(short = 'U', long, default_value = DEFAULT_SHUFFLE_NOTIFICATION_SUMMARY)]
    pub shuffle_notification_summary: String,
    /// The time out of the shuffle mode change notification, in seconds.
    #[arg(short = 'Y', long, default_value_t = DEFAULT_SHUFFLE_NOTIFICATION_TIMEOUT)]
    pub shuffle_notification_timeout: u8,
    /// The repeat mode change notification body.
    /// you can use the placeholders like "{repeat}" in the body, it will be replaced with the repeat mode.
    ///
    /// If you leave it empty, the notification will not be shown.
    #[arg(short = 'R', long, default_value = DEFAULT_REPEAT_NOTIFICATION_BODY)]
    pub repeat_notification_body: String,
    /// The repeat mode change notification summary.
    /// you can use the placeholders like "{repeat}" in the summary, it will be replaced with the repeat mode.
    #[arg(short = 'G', long, default_value = DEFAULT_REPEAT_NOTIFICATION_SUMMARY)]
    pub repeat_notification_summary: String,
    /// The time out of the repeat mode change notification, in seconds.
    #[arg(short = 'H', long, default_value_t = DEFAULT_REPEAT_NOTIFICATION_TIMEOUT)]
    pub repeat_notification_timeout: u8,
    /// The aaa mode change notification body.
    /// you can use the placeholders like "{aaa_mode}" in the body, it will be replaced with the aaa mode.
    ///
    /// If you leave it empty, the notification will not be shown.
    #[arg(short = 'A', long, default_value = DEFAULT_AAAMODE_NOTIFICATION_BODY)]
    pub aaa_mode_notification_body: String,
    /// The aaa mode change notification summary.
    /// you can use the placeholders like "{aaa_mode}" in the summary, it will be replaced with the aaa mode.
    #[arg(short = 'D', long, default_value = DEFAULT_AAAMODE_NOTIFICATION_SUMMARY)]
    pub aaa_mode_notification_summary: String,
    /// The time out of the aaa mode change notification, in seconds.
    #[arg(short = 'F', long, default_value_t = DEFAULT_AAAMODE_NOTIFICATION_TIMEOUT)]
    pub aaa_mode_notification_timeout: u8,
    #[cfg(feature = "lyrics")]
    /// The lyrics notification body, if you want to show the lyrics separate notification.
    /// you can use the placeholders like "{lyrics}" in the body, it will be replaced with the lyrics.
    ///
    /// If you leave it empty, the notification will not be shown.
    #[arg(short = 'L', long, default_value = DEFAULT_LYRICS_NOTIFICATION_BODY)]
    pub lyrics_notification_body: String,
    #[cfg(feature = "lyrics")]
    /// The lyrics notification summary, if you want to show the lyrics separate notification.
    /// you can use the placeholders like "{lyrics}" in the summary, it will be replaced with the lyrics.
    #[arg(short = 'M', long, default_value = DEFAULT_LYRICS_NOTIFICATION_SUMMARY)]
    pub lyrics_notification_summary: String,
}

impl Default for Settings {
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
            show_player_notifications: false,
            volume_notification_body: DEFAULT_VOLUME_CHANGE_NOTIFICATION_BODY.to_string(),
            volume_notification_summary: DEFAULT_VOLUME_CHANGE_NOTIFICATION_SUMMARY.to_string(),
            volume_notification_timeout: DEFAULT_VOLUME_CHANGE_NOTIFICATION_TIMEOUT,
            shuffle_notification_body: DEFAULT_SHUFFLE_NOTIFICATION_BODY.to_string(),
            shuffle_notification_summary: DEFAULT_SHUFFLE_NOTIFICATION_SUMMARY.to_string(),
            shuffle_notification_timeout: DEFAULT_SHUFFLE_NOTIFICATION_TIMEOUT,
            repeat_notification_body: DEFAULT_REPEAT_NOTIFICATION_BODY.to_string(),
            repeat_notification_summary: DEFAULT_REPEAT_NOTIFICATION_SUMMARY.to_string(),
            repeat_notification_timeout: DEFAULT_REPEAT_NOTIFICATION_TIMEOUT,
            aaa_mode_notification_body: DEFAULT_AAAMODE_NOTIFICATION_BODY.to_string(),
            aaa_mode_notification_summary: DEFAULT_AAAMODE_NOTIFICATION_SUMMARY.to_string(),
            aaa_mode_notification_timeout: DEFAULT_AAAMODE_NOTIFICATION_TIMEOUT,
            #[cfg(feature = "lyrics")]
            lyrics_notification_body: DEFAULT_LYRICS_NOTIFICATION_BODY.to_string(),
            #[cfg(feature = "lyrics")]
            lyrics_notification_summary: DEFAULT_LYRICS_NOTIFICATION_SUMMARY.to_string(),
        }
    }
}

impl Settings {
    pub fn load_config_and_parse_args() -> Self {
        // load config file
        let cfg: Self = match confy::load("cmus-notify", "config") {
            Ok(cfg) => cfg,
            Err(err) => {
                eprintln!("Failed to load config: {}", err);
                Self::default()
            }
        };

        // parse the args
        let mut args = Settings::parse();

        // Combine the config and args(the args will override the config)
        if args.timeout == NOTIFICATION_TIMEOUT {
            args.timeout = cfg.timeout;
        }
        if args.persistent == false {
            args.persistent = cfg.persistent;
        }
        if args.show_track_cover == true {
            args.show_track_cover = cfg.show_track_cover;
        }
        args.notification_static_icon = args
            .notification_static_icon
            .or(cfg.notification_static_icon);
        args.cover_path = args.cover_path.or(cfg.cover_path);
        #[cfg(feature = "lyrics")]
        if args.lyrics_path == None {
            args.lyrics_path = cfg.lyrics_path;
        }

        if args.depth == DEFAULT_MAX_DEPTH {
            args.depth = cfg.depth;
        }
        if args.app_name == NOTIFICATION_APP_NAME {
            args.app_name = cfg.app_name;
        }
        if args.summary == NOTIFICATION_SUMMARY {
            args.summary = cfg.summary;
        }
        if args.body == NOTIFICATION_BODY {
            args.body = cfg.body;
        }
        args.cmus_remote_bin_path = args.cmus_remote_bin_path.or(cfg.cmus_remote_bin_path);
        args.cmus_socket_address = args.cmus_socket_address.or(cfg.cmus_socket_address);
        args.cmus_socket_password = args.cmus_socket_password.or(cfg.cmus_socket_password);
        if args.interval == DEFAULT_INTERVAL_TIME {
            args.interval = cfg.interval;
        }
        if args.link == false {
            args.link = cfg.link;
        }
        if args.force_use_external_cover == false {
            args.force_use_external_cover = cfg.force_use_external_cover;
        }
        #[cfg(feature = "lyrics")]
        if args.force_use_external_lyrics == false {
            args.force_use_external_lyrics = cfg.force_use_external_lyrics;
        }
        if args.no_use_external_cover == false {
            args.no_use_external_cover = cfg.no_use_external_cover;
        }
        #[cfg(feature = "lyrics")]
        if args.no_use_external_lyrics == false {
            args.no_use_external_lyrics = cfg.no_use_external_lyrics;
        }
        if args.show_player_notifications == false {
            args.show_player_notifications = cfg.show_player_notifications;
        }
        if args.volume_notification_body == DEFAULT_VOLUME_CHANGE_NOTIFICATION_BODY {
            args.volume_notification_body = cfg.volume_notification_body;
        }
        if args.volume_notification_summary == DEFAULT_VOLUME_CHANGE_NOTIFICATION_SUMMARY {
            args.volume_notification_summary = cfg.volume_notification_summary;
        }
        if args.volume_notification_timeout == DEFAULT_VOLUME_CHANGE_NOTIFICATION_TIMEOUT {
            args.volume_notification_timeout = cfg.volume_notification_timeout;
        }
        if args.shuffle_notification_body == DEFAULT_SHUFFLE_NOTIFICATION_BODY {
            args.shuffle_notification_body = cfg.shuffle_notification_body;
        }
        if args.shuffle_notification_summary == DEFAULT_SHUFFLE_NOTIFICATION_SUMMARY {
            args.shuffle_notification_summary = cfg.shuffle_notification_summary;
        }
        if args.shuffle_notification_timeout == DEFAULT_SHUFFLE_NOTIFICATION_TIMEOUT {
            args.shuffle_notification_timeout = cfg.shuffle_notification_timeout;
        }
        if args.repeat_notification_body == DEFAULT_REPEAT_NOTIFICATION_BODY {
            args.repeat_notification_body = cfg.repeat_notification_body;
        }
        if args.repeat_notification_summary == DEFAULT_REPEAT_NOTIFICATION_SUMMARY {
            args.repeat_notification_summary = cfg.repeat_notification_summary;
        }
        if args.repeat_notification_timeout == DEFAULT_REPEAT_NOTIFICATION_TIMEOUT {
            args.repeat_notification_timeout = cfg.repeat_notification_timeout;
        }
        if args.aaa_mode_notification_body == DEFAULT_AAAMODE_NOTIFICATION_BODY {
            args.aaa_mode_notification_body = cfg.aaa_mode_notification_body;
        }
        if args.aaa_mode_notification_summary == DEFAULT_AAAMODE_NOTIFICATION_SUMMARY {
            args.aaa_mode_notification_summary = cfg.aaa_mode_notification_summary;
        }
        if args.aaa_mode_notification_timeout == DEFAULT_AAAMODE_NOTIFICATION_TIMEOUT {
            args.aaa_mode_notification_timeout = cfg.aaa_mode_notification_timeout;
        }

        args
    }
}
