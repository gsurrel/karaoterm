extern crate termion;

use clap::Clap;
use std::io::Write;
use std::ops::Sub;

/// Reads a subtitle (.srt) file at the right pace for singing karaoke, straight from your terminal
#[derive(Clap)]
#[clap(version = "1.0", author = "Grégoire Surrel")]
struct Opts {
    /// Path to a subtitle file
    #[clap(short, long)]
    subtitle: String,

    /// The time in seconds a full terminal screen lasts
    #[clap(short, long, default_value = "5")]
    time_screen: u8,
}

fn main() {
    let opts: Opts = Opts::parse();

    let screen_time = std::time::Duration::from_secs(opts.time_screen as u64);

    let items = srtparse::from_file(opts.subtitle).unwrap();

    let start_time = std::time::SystemTime::now();
    let end_time = items.last().unwrap().end_time.into_duration();

    print!("{}{}", termion::cursor::Goto(1, 1), termion::clear::All);

    loop {
        // Get time and quit of last sub has passed
        let now = std::time::SystemTime::now()
            .duration_since(start_time)
            .unwrap();
        if now > end_time + screen_time {
            return;
        }

        // Compute screen parameters
        let (_w, h) = termion::terminal_size().unwrap_or((80, 40));
        let line_time = screen_time / h as u32;

        // New screen
        print!("{}", termion::cursor::Goto(1, 1));

        for line_nb in 1..=h {
            let line_time = now + (line_time * line_nb as u32);
            if line_time < screen_time {
                continue;
            }
            let line_time = line_time.sub(screen_time);

            // Set the colors
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

            // Clear the line
            print!(
                "{} {:02.02} ",
                termion::cursor::Goto(10, line_nb),
                line_time.as_secs_f32()
            );

            // Display the right text
            for item in &items {
                if item.start_time.into_duration() < line_time
                    && item.end_time.into_duration() > line_time
                {
                    print!(
                        "{}{}{}{}",
                        termion::cursor::Goto(20, line_nb),
                        item.text,
                        termion::cursor::Hide,
                        termion::clear::AfterCursor
                    );
                }
            }
        }
        std::io::stdout().flush().unwrap();
        std::thread::sleep(std::time::Duration::from_millis(64));
    }
}
