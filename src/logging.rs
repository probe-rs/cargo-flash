use colored::*;
use env_logger::Builder;
use fmt::Display;
use indicatif::ProgressBar;
use log::{Level, LevelFilter};
use std::{
    fmt,
    io::Write,
    sync::{
        atomic::{AtomicUsize, Ordering},
        Arc, RwLock,
    },
};

use probe_rs::flashing::ProgressEvent;
use serde::Serialize;

static MAX_WINDOW_WIDTH: AtomicUsize = AtomicUsize::new(0);

lazy_static::lazy_static! {
    /// Stores the progress bar for the logging facility.
    static ref PROGRESS_BAR: RwLock<Option<Arc<ProgressBar>>> = RwLock::new(None);
}

struct Padded<T> {
    value: T,
    width: usize,
}

impl<T: fmt::Display> fmt::Display for Padded<T> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{: <width$}", self.value, width = self.width)
    }
}

fn max_target_width(target: &str) -> usize {
    let max_width = MAX_WINDOW_WIDTH.load(Ordering::Relaxed);
    if max_width < target.len() {
        MAX_WINDOW_WIDTH.store(target.len(), Ordering::Relaxed);
        target.len()
    } else {
        max_width
    }
}

fn colored_level(level: Level) -> ColoredString {
    match level {
        Level::Trace => "TRACE".magenta().bold(),
        Level::Debug => "DEBUG".blue().bold(),
        Level::Info => " INFO".green().bold(),
        Level::Warn => " WARN".yellow().bold(),
        Level::Error => "ERROR".red().bold(),
    }
}

pub struct TerminalOutputter {
    output_json: bool,
}

impl TerminalOutputter {
    pub fn init(level: Option<Level>, output_json: bool) -> Self {
        let mut builder = Builder::new();

        builder.filter_level(LevelFilter::Warn);

        if let Ok(s) = ::std::env::var("RUST_LOG") {
            builder.parse_filters(&s);
        }

        if let Some(level) = level {
            builder.filter_level(level.to_level_filter());
        }

        builder.format(move |f, record| {
            let target = record.target();
            let max_width = max_target_width(target);

            let level = colored_level(record.level());

            let mut style = f.style();
            let target = style.set_bold(true).value(Padded {
                value: target,
                width: max_width,
            });

            let guard = PROGRESS_BAR.write().unwrap();
            if let Some(pb) = &*guard {
                pb.println(format!("       {} {} > {}", level, target, record.args()));
            } else {
                eprintln!("       {} {} > {}", level, target, record.args());
            }

            Ok(())
        });

        builder.try_init().unwrap();

        TerminalOutputter { output_json }
    }

    /// Should we use colored output or not?
    fn use_colored_output(&self) -> bool {
        !self.output_json
    }
}

/// Sets the current progress bar in store for the logging facility.
pub fn set_progress_bar(progress: Arc<ProgressBar>) {
    let mut guard = PROGRESS_BAR.write().unwrap();
    *guard = Some(progress);
}

impl TerminalOutputter {
    fn write_line(&self, stream: &mut impl Write, message: String) {
        let guard = PROGRESS_BAR.write().unwrap();

        match guard.as_ref() {
            Some(pb) if !pb.is_finished() => {
                pb.println(message);
            }
            _ => {
                let message = if self.output_json {
                    serde_json::to_string(&JsonOutput::Message { content: message })
                        .expect("Serde JSON serialization failed. This is a bug.")
                } else {
                    message
                };

                writeln!(stream, "{}", message).expect("Failed to write to output stream.");
            }
        }
    }

    /// Writes an error to the log.
    /// This can be used for unwraps/eprintlns/etc.
    pub fn eprintln(&self, message: impl AsRef<str>) {
        self.write_line(&mut std::io::stderr(), message.as_ref().to_owned());
    }

    /// Writes an error to the log.
    /// This can be used for unwraps/eprintlns/etc.
    pub fn println(&self, message: impl AsRef<str>) {
        self.write_line(&mut std::io::stdout(), message.as_ref().to_owned());
    }

    /// Writes an error to the log.
    /// This can be used for unwraps/eprintlns/etc.
    pub fn println_formatted(&self, message: impl Display, heading: Option<impl Display>) {
        let formatted_message = match heading {
            Some(heading) => {
                if self.use_colored_output() {
                    format!("    {} {}", heading.to_string().green().bold(), message)
                } else {
                    format!("    {} {}", heading, message)
                }
            }
            None => format!("     {}", message),
        };
        self.write_line(&mut std::io::stdout(), formatted_message);
    }
}

#[derive(Serialize)]
#[serde(tag = "type")]
pub enum JsonOutput {
    #[serde(rename = "message")]
    Message { content: String },
    #[serde(rename = "progress")]
    Progress {
        event: String,
        #[serde(skip_serializing_if = "Option::is_none")]
        size: Option<u32>,
        #[serde(skip_serializing_if = "Option::is_none")]
        fill_size: Option<u32>,
        #[serde(skip_serializing_if = "Option::is_none")]
        erase_size: Option<u32>,
        #[serde(skip_serializing_if = "Option::is_none")]
        program_size: Option<u32>,
    },
}

impl JsonOutput {
    fn progress_event(event: &str) -> Self {
        JsonOutput::Progress {
            event: event.to_owned(),
            fill_size: None,
            erase_size: None,
            program_size: None,
            size: None,
        }
    }
}

impl From<ProgressEvent> for JsonOutput {
    fn from(event: ProgressEvent) -> Self {
        match event {
            ProgressEvent::Initialized { flash_layout } => {
                let fill_size = flash_layout.fills().iter().map(|f| f.size()).sum();
                let erase_size = flash_layout.sectors().iter().map(|f| f.size()).sum();
                let program_size = flash_layout.pages().iter().map(|f| f.size()).sum();

                JsonOutput::Progress {
                    event: "initialized".to_owned(),
                    fill_size: Some(fill_size),
                    erase_size: Some(erase_size),
                    program_size: Some(program_size),
                    size: None,
                }
            }
            ProgressEvent::StartedFilling => JsonOutput::progress_event("started_filling"),
            ProgressEvent::FinishedFilling => JsonOutput::progress_event("finished_filling"),
            ProgressEvent::StartedProgramming => JsonOutput::progress_event("started_flashing"),
            ProgressEvent::StartedErasing => JsonOutput::progress_event("started_erasing"),
            ProgressEvent::PageProgrammed { size, .. } => JsonOutput::Progress {
                event: "page_flashed".to_owned(),
                fill_size: None,
                erase_size: None,
                program_size: None,
                size: Some(size),
            },
            ProgressEvent::SectorErased { size, .. } => JsonOutput::Progress {
                event: "sector_erased".to_owned(),
                fill_size: None,
                erase_size: None,
                program_size: None,
                size: Some(size),
            },
            ProgressEvent::PageFilled { .. } => JsonOutput::progress_event("page_filled"),
            ProgressEvent::FailedProgramming => JsonOutput::progress_event("failed_programming"),
            ProgressEvent::FinishedProgramming => {
                JsonOutput::progress_event("finished_programming")
            }
            ProgressEvent::FailedErasing => JsonOutput::progress_event("failed_erasing"),
            ProgressEvent::FinishedErasing => JsonOutput::progress_event("finished_erasing"),
            ProgressEvent::FailedFilling => JsonOutput::progress_event("failed_filling"),
        }
    }
}
