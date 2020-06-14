extern crate termion;

use clap::Clap;
use rodio::Source;
use std::io::Write;
use std::ops::Sub;

/// Reads a subtitle (.srt) file at the right pace for singing karaoke, straight from your terminal
#[derive(Clap)]
#[clap(version = "1.0", author = "Grégoire Surrel")]
struct Opts {
    /// Path to a lyrics file (srt)
    #[clap(short, long)]
    lyrics: String,

    /// Path to a music file (mp3, wav, ogg, and flac)
    #[clap(short, long)]
    song: Option<String>,

    /// The time in seconds a full terminal screen lasts
    #[clap(short, long, default_value = "5")]
    time_screen: u8,
}

fn main() {
    // Parse the command-line arguments
    let opts: Opts = Opts::parse();

    // Get a audio handle
    let device = rodio::default_output_device().unwrap();
    let mut playing = false;

    // Define how much time there is to go from the bottom of the screen to the top
    let screen_time = std::time::Duration::from_secs(opts.time_screen as u64);

    // Collect all the lyrics items from the file
    let items = srtparse::from_file(opts.lyrics).unwrap();

    // Be time-aware :)
    let start_time = std::time::SystemTime::now();
    let end_time = items.last().unwrap().end_time.into_duration();

    // Let's start by clearing the screen and hiding the cursor
    print!(
        "{}{}{}",
        termion::cursor::Goto(1, 1),
        termion::clear::All,
        termion::cursor::Hide
    );

    // Loop while there are some lyrics to read, with little intro and ending time added
    loop {
        // Get time and quit of last sub has passed
        let now = std::time::SystemTime::now()
            .duration_since(start_time)
            .unwrap();
        if now > end_time + screen_time {
            // Restore the cursor presence before quitting
            print!("{}", termion::cursor::Show);
            return;
        }

        // Start playing if not already. We can unwrap as we test if there is an audio file provided
        if !playing && now > 2 * screen_time / 3 && opts.song.is_some() {
            playing = true;
            let file = std::fs::File::open(opts.song.clone().unwrap()).unwrap();
            let source = rodio::Decoder::new(std::io::BufReader::new(file)).unwrap();
            rodio::play_raw(&device, source.convert_samples());
        }

        // Get the screen height and define the time for each line
        let (_w, h) = termion::terminal_size().unwrap_or((80, 40));
        let line_time = screen_time / h as u32;

        // New screen
        print!("{}", termion::cursor::Goto(1, 1));

        // For each line of the terminal, display what it should show
        for line_nb in 1..=h {
            // Compute the absolute time of the line to match with the lyrics
            let line_time = now + (line_time * line_nb as u32);
            if line_time < screen_time {
                // Skip lines with negative time
                continue;
            }

            let line_time = line_time.sub(screen_time);

            // Set the colors: red for the past, green for the current line, and white for the future
            if line_nb < h / 3 {
                print!(
                    "{}{}",
                    termion::color::Fg(termion::color::Red),
                    termion::color::Bg(termion::color::Reset)
                );
            } else if line_nb == h / 3 {
                print!(
                    "{}{}",
                    termion::color::Fg(termion::color::Green),
                    termion::color::Bg(termion::color::Reset)
                );
            } else {
                print!(
                    "{}{}",
                    termion::color::Fg(termion::color::White),
                    termion::color::Bg(termion::color::Reset)
                );
            }

            // Write the time-code of the line
            print!(
                "{} {:02.02} ",
                termion::cursor::Goto(10, line_nb),
                line_time.as_secs_f32()
            );

            // Display the right text in the line
            for item in &items {
                if item.start_time.into_duration() < line_time
                    && item.end_time.into_duration() > line_time
                {
                    print!(
                        "{}{}{}",
                        termion::cursor::Goto(20, line_nb),
                        item.text,
                        termion::clear::AfterCursor
                    );
                }
            }
        }

        // Flush the buffer to let the terminal emulator draw everything before sleeping for 15 fps
        std::io::stdout().flush().unwrap();
        std::thread::sleep(std::time::Duration::from_millis(64));
    }
}
