use std::io::{Write, Stdout, stdout};
use std::sync::Arc;
use std::sync::mpsc::Receiver;
use termion::screen::*;
use termion::color;
use termion::cursor;
use termion::clear;

use chrono::prelude::*;

use std::{time, thread};

use super::{
    RunnerOptions,
    StatusUpdate
};

struct DisplayState {
    frame_start: u32,
    frame_end: u32,
    frames_to_render: u32,
    render_times: Vec<u32>,
    time_start: Option<DateTime<Local>>,
    time_end: Option<DateTime<Local>>
}

pub fn display_status(options: Arc<RunnerOptions>, rx: Receiver<StatusUpdate>) {
    let mut state = DisplayState {
        frame_start: options.frame_start,
        frame_end: options.frame_end,
        frames_to_render: (options.frame_end - options.frame_start + 1),
        render_times: Vec::new(),
        time_start: None,
        time_end: None
    };

    {
        let mut screen = AlternateScreen::from(stdout());

        write!(
            screen, "{}",
            cursor::Hide
        ).unwrap();
        screen.flush().unwrap();

        loop {
            match rx.recv() {
                Ok(message) => {
                    match message {
                        StatusUpdate::Started => {
                            state.time_start = Some(Local::now());
                            display_start(&mut screen, &state);
                            display_progress(&mut screen, &state);
                        },
                        StatusUpdate::RenderedFrame { render_time, frame_number: _ } => {
                            state.render_times.push(render_time);

                            display_progress(&mut screen, &state);
                        },
                        StatusUpdate::Finished => {
                            state.time_end = Some(Local::now());
                            display_finish(&mut screen, &state);
                        }
                    }
                },
                _ => {
                    break;
                }
            }
        }

        thread::sleep(time::Duration::from_secs(3));

        write!(
            screen, "{}",
            cursor::Show
        ).unwrap();
        screen.flush().unwrap();

        // AlternateScreen drops at the end of this block.
    }

    let frames_rendered = state.render_times.len() as u32;
    let elapsed = state.time_end.unwrap().signed_duration_since(state.time_start.unwrap()).num_milliseconds() as u32;
    let average_render_time = calc_average_render_time(&state.render_times);

    println!("Rendered {} frames in {}", frames_rendered, format_duration(elapsed));
    println!("  Started:  {}", format_time(state.time_start));
    println!("  Finished: {}", format_time(state.time_end));
    println!("  Average frame render time: {}", format_duration(average_render_time));
}

fn display_start(screen: &mut AlternateScreen<Stdout>, state: &DisplayState) {
    write!(
        screen,
        "{}Started at {}",
        cursor::Goto(1, 1),
        format_time(state.time_start)
    ).unwrap();
    write!(
        screen,
        "{}Rendering frames {} through {}",
        cursor::Goto(1, 2),
        state.frame_start,
        state.frame_end
    ).unwrap();
    screen.flush().unwrap();
}

fn display_progress(screen: &mut AlternateScreen<Stdout>, state: &DisplayState) {
    let frames_rendered = state.render_times.len() as u32;
    let frames_remaining = state.frames_to_render - frames_rendered;
    let percent_complete = (frames_rendered as f64) / (state.frames_to_render as f64);
    let average_render_time = calc_average_render_time(&state.render_times);
    let time_now = Local::now();
    let elapsed = time_now.signed_duration_since(state.time_start.unwrap()).num_milliseconds() as u32;

    write!(
        screen,
        "{}{}{}",
        cursor::Goto(1, 4),
        clear::CurrentLine,
        progress_bar(percent_complete)
    ).unwrap();

    write!(
        screen,
        "{}{}Rendered {} frames, {} remaining",
        cursor::Goto(1, 6),
        clear::CurrentLine,
        frames_rendered,
        frames_remaining
    ).unwrap();
    write!(
        screen,
        "{}{}Total time elapsed: {}",
        cursor::Goto(1, 7),
        clear::CurrentLine,
        format_duration(elapsed)
    ).unwrap();
    write!(
        screen,
        "{}{}Average frame render time: {}",
        cursor::Goto(1, 8),
        clear::CurrentLine,
        format_duration(average_render_time)
    ).unwrap();
    write!(
        screen,
        "{}",
        cursor::Goto(1, 9)
    ).unwrap();

    screen.flush().unwrap();
}

fn display_finish(screen: &mut AlternateScreen<Stdout>, state: &DisplayState) {
    write!(
        screen,
        "{}Finished at {}",
        cursor::Goto(1, 10),
        format_time(state.time_end)
    ).unwrap();
    screen.flush().unwrap();
}

fn calc_average_render_time(render_times: &Vec<u32>) -> u32 {
    let frame_count = render_times.len();
    let sum = render_times
        .iter()
        .fold(0, |acc, render_time| acc + render_time);
    let average = (sum as f64) / (frame_count as f64);

    (average.round() as u32)
}

fn progress_bar(percent_complete: f64) -> String {
    let bar_length = 30;

    let completed_length = ((bar_length as f64) * percent_complete).round() as usize;

    let completed_portion = " ".repeat(completed_length);
    let remaining_portion = " ".repeat(bar_length - completed_length);

    format!(
        "[ {}{}{}{}{} ] ({}%)",
        color::Bg(color::White),
        completed_portion,
        color::Bg(color::Black),
        remaining_portion,
        color::Bg(color::Reset),
        format!("{}", (percent_complete * 100.0).round())
    )
}

fn format_time(time: Option<DateTime<Local>>) -> String {
    match time {
        Some(time_value) => time_value.format("%b %-d, %-I:%M%P").to_string(),
        None => "---".to_owned()
    }
}

fn format_duration(duration_ms: u32) -> String {
    if duration_ms > 3_600_000 {
        let full_hours = duration_ms / 3_600_000;
        let remaining_ms_after_hours = duration_ms % 3_600_000;
        let full_minutes = remaining_ms_after_hours / 60_000;
        let remaining_ms_after_minutes = remaining_ms_after_hours % 60_000;
        let seconds = (remaining_ms_after_minutes as f64) / 1_000.0;

        return format!("{}h {}m {}s", full_hours, full_minutes, seconds.round());
    } else if duration_ms > 60_000 {
        let full_minutes = duration_ms / 60_000;
        let remaining_ms = duration_ms % 60_000;
        let seconds = (remaining_ms as f64) / 1_000.0;

        return format!("{}m {:.1}s", full_minutes, seconds);
    } else {
        let seconds = (duration_ms as f64) / 1_000.0;

        return format!("{:.1}s", seconds);
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_format_duration_seconds() {
        assert_eq!(format_duration(0_228), "0.2s");
        assert_eq!(format_duration(4_536), "4.5s");
    }

    #[test]
    fn test_format_duration_seconds_rounding() {
        assert_eq!(format_duration(4_576), "4.6s");
    }

    #[test]
    fn test_format_duration_minutes() {
        // == (4 * 60000) + (23 * 1000) + 700
        assert_eq!(format_duration(263_700), "4m 23.7s");

        // == (35 * 60000) + (18 * 1000) + 300
        assert_eq!(format_duration(2_118_300), "35m 18.3s");
    }

    #[test]
    fn test_format_duration_hours() {
        // Note the dropped precision on seconds
        // == (5 * 3600000) + (56 * 60000) + (29 * 1000) + 300
        assert_eq!(format_duration(21_389_300), "5h 56m 29s");
    }

    #[test]
    fn test_format_duration_hours_multiple_days() {
        // Note how we stay on hours instead of showing days
        // == (27 * 3600000) + (32 * 60000) + (6 * 1000) + 400
        assert_eq!(format_duration(99_126_400), "27h 32m 6s");
    }
}
