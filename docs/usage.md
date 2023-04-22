# Command-Line Help for `cmus-notify`

This document contains the help content for the `cmus-notify` command-line program.

**Command Overview:**

* [`cmus-notify`↴](#cmus-notify)

## `cmus-notify`

A simple notification daemon for cmus

**Usage:** `cmus-notify [OPTIONS] [BODY]`

###### **Arguments:**

* `<BODY>` — The body of the notification

###### **Options:**

* `-t`, `--timeout <TIMEOUT>` — The notification timeout, in seconds
* `-p`, `--persistent` — Make the notification persistent, i.e. not disappear after a timeout (you can dismiss it manually)
* `-c`, `--cover` — Show the track cover in the notification, if available
* `-i`, `--icon <NOTIFICATION_STATIC_COVER>` — The static icon to use for the notification, it not effective if the track cover is shown, but if the cover is not available or you disabled it, this icon will be used
* `-w`, `--cover-path <COVER_PATH_TEMPLATE>` — The path to look for the cover image, if not given, the cover will be searched in the track's directory for an image file with the name "cover"
* `-y`, `--lyrics-path <LYRICS_PATH>` — The lyrics file path, if not given, the lyrics will be searched in the track's directory for a text file with the name "lyrics", or with the same name as the track
* `-d`, `--depth <DEPTH>` — The maximum path depth to search for the cover and lyrics files, if the files are not found in the track's directory, or the directory specified by the `--cover-path` or `--lyrics-path`* options, the program will search in the parent directory, and so on, until the maximum depth is reached
* `-a`, `--app-name <APP_NAME>` — The name of the app to use for the notification
* `-s`, `--summary <SUMMARY>` — The summary of the notification
* `-b`, `--cmus-remote-bin <CMUS_REMOTE_BIN_PATH>` — The cmus-remote binary path, if not given, the program will search for it in the PATH environment variable
* `-k`, `--cmus-socket <CMUS_SOCKET_ADDRESS>` — The cmus socket address, if not given, the program will use the default socket address, which is "$XDG_RUNTIME_DIR/cmus-socket"
* `-x`, `--socket-password <CMUS_SOCKET_PASSWORD>` — The cmus socket password, if any
* `-r`, `--interval <INTERVAL>` — The interval to request the cmus status, in milliseconds
* `-l`, `--link` — Link the program with cmus, if the cmus are not running, the program will exit
* `-u`, `--force-use-external-cover` — Force the program to use the external cover file, if available, and not even try to get the cover from the track's metadata. this is useful if you have a cover file with a better quality than the cover in the track's metadata
* `-m`, `--force-use-external-lyrics` — Fotrce the program to use the external lyrics file, if available, and not even try to get the lyrics from the track's metadata
* `-n`, `--no-use-external-cover` — No use the external cover file, even if it's available and the track's metadata doesn't have a cover
* `-o`, `--no-use-external-lyrics` — No use the external lyrics file, even if it's available and the track's metadata doesn't have a lyrics
* `-g`, `--show-player-notifications` — Show the player notifications, like if you change the shuffle mode, or the repeat mode, or the volume
* `-B`, `--volume-notification-body <VOLUME_NOTIFICATION_BODY>` — The volume change notification body. you can use the placeholders like "{volume}" in the body, it will be replaced with the shuffle mode
* `-E`, `--volume-notification-summary <VOLUME_NOTIFICATION_SUMMARY>` — The volume change notification summary
* `-T`, `--volume-notification-timeout <VOLUME_NOTIFICATION_TIMEOUT>` — The time out of the volume change notification, in seconds
* `-S`, `--shuffle-notification-body <SHUFFLE_NOTIFICATION_BODY>` — The shuffle mode change notification body. you can use the placeholders like "{shuffle}" in the body, it will be replaced with the shuffle mode
* `-U`, `--shuffle-notification-summary <SHUFFLE_NOTIFICATION_SUMMARY>` — The shuffle mode change notification summary. you can use the placeholders like "{shuffle}" in the summary, it will be replaced with the shuffle mode
* `-Y`, `--shuffle-notification-timeout <SHUFFLE_NOTIFICATION_TIMEOUT>` — The time out of the shuffle mode change notification, in seconds
* `-R`, `--repeat-notification-body <REPEAT_NOTIFICATION_BODY>` — The repeat mode change notification body. you can use the placeholders like "{repeat}" in the body, it will be replaced with the repeat mode
* `-G`, `--repeat-notification-summary <REPEAT_NOTIFICATION_SUMMARY>` — The repeat mode change notification summary. you can use the placeholders like "{repeat}" in the summary, it will be replaced with the repeat mode
* `-H`, `--repeat-notification-timeout <REPEAT_NOTIFICATION_TIMEOUT>` — The time out of the repeat mode change notification, in seconds
* `-A`, `--aaa-mode-notification-body <AAA_MODE_NOTIFICATION_BODY>` — The aaa mode change notification body. you can use the placeholders like "{aaa_mode}" in the body, it will be replaced with the aaa mode
* `-D`, `--aaa-mode-notification-summary <AAA_MODE_NOTIFICATION_SUMMARY>` — The aaa mode change notification summary. you can use the placeholders like "{aaa_mode}" in the summary, it will be replaced with the aaa mode
* `-F`, `--aaa-mode-notification-timeout <AAA_MODE_NOTIFICATION_TIMEOUT>` — The time out of the aaa mode change notification, in seconds
* `-L`, `--lyrics-notification-body <LYRICS_NOTIFICATION_BODY>` — The lyrics notification body, if you want to show the lyrics separate notification. you can use the placeholders like "{lyrics}" in the body, it will be replaced with the lyrics
* `-M`, `--lyrics-notification-summary <LYRICS_NOTIFICATION_SUMMARY>` — The lyrics notification summary, if you want to show the lyrics separate notification. you can use the placeholders like "{lyrics}" in the summary, it will be replaced with the lyrics
* `-O`, `--status-notification-body <STATUS_NOTIFICATION_BODY>` — The status change notification body. you can use the placeholders like "{status}" in the body, it will be replaced with the aaa mode
* `-P`, `--status-notification-summary <STATUS_NOTIFICATION_SUMMARY>` — The status change notification summary. you can use the placeholders like "{status}" in the summary, it will be replaced with the aaa mode
* `-Q`, `--status-notification-timeout <STATUS_NOTIFICATION_TIMEOUT>` — The time out of the status change notification, in seconds
* `--markdown-help`
* `--config <CONFIG_PATH>` — Use a custom config path



<hr/>

<small><i>
    This document was generated automatically by
    <a href="https://crates.io/crates/clap-markdown"><code>clap-markdown</code></a>.
</i></small>

