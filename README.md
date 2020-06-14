# karaoterm

Reads a subtitle file to display it in the terminal for karaoke, along with music playback

```console
karaoterm 1.0
Grégoire Surrel
Reads a subtitle (.srt) file at the right pace for singing karaoke along with optional music playback, straight from
your terminal

USAGE:
    karaoterm [OPTIONS] --lyrics <lyrics>

FLAGS:
    -h, --help       Prints help information
    -V, --version    Prints version information

OPTIONS:
    -l, --lyrics <lyrics>              Path to a lyrics file (srt)
    -s, --song <song>                  Path to a music file (mp3, wav, ogg, and flac)
    -t, --time-screen <time-screen>    The time in seconds a full terminal screen lasts [default: 5]
```