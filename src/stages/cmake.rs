use super::InstallationStage;
use crate::config::CmakeConfig;
use crate::log_corpus::LogCorpus;
use crate::log_generator::LogGenerator;
use crate::ui::{ProgressBar, ProgressStyle, Spinner};
use colored::*;
use rand::Rng;
use std::io;
use std::thread;
use std::time::Duration;

pub struct CmakeStage {
    logs: LogCorpus,
    config: CmakeConfig,
}

impl CmakeStage {
    pub fn new(config: CmakeConfig) -> Self {
        Self {
            logs: LogCorpus::new(
                include_str!("../../data/cmake.log"),
                include_str!("../../data/error/cmake.log"),
            ),
            config,
        }
    }

    /// Split "[ 42%] Building C object ..." into its percentage and message
    fn split_progress(log: &str) -> Option<(f32, &str)> {
        let rest = log.trim_start().strip_prefix('[')?;
        let (percent, message) = rest.split_once("%]")?;
        let percent: f32 = percent.trim().parse().ok()?;

        Some((percent / 100.0, message.trim()))
    }

    /// Colorize the tail of a "-- Looking for foo - found" probe line
    fn render_probe(log: &str) -> String {
        match log.rsplit_once(" - ") {
            Some((probe, result)) => {
                let result = if result.starts_with("not found") || result.starts_with("failed") {
                    result.yellow()
                } else {
                    result.green()
                };
                format!("{} - {}", probe.dimmed(), result)
            }
            None => format!("{}", log.dimmed()),
        }
    }

    /// Display a recorded cmake transcript line by line
    fn display_logs(&self, logs: &[String], exit_check: &dyn Fn() -> bool) -> io::Result<()> {
        let mut rng = rand::thread_rng();
        // A compiler diagnostic spans several lines: the message, the offending
        // source line, the caret and any notes. Keep them coloured as one block.
        let mut in_diagnostic = false;

        for log in logs {
            if exit_check() {
                return Err(io::Error::new(io::ErrorKind::Interrupted, "User interrupt"));
            }

            if log.contains("error:") || log.starts_with("CMake Error") {
                in_diagnostic = true;
            } else if log.starts_with("--") || log.trim_start().starts_with('[') {
                in_diagnostic = false;
            }

            if let Some((percent, message)) = Self::split_progress(log) {
                let bar = ProgressBar::new(ProgressStyle::Block);

                if message.starts_with("Linking") {
                    bar.animate(
                        &format!("{} {}", LogGenerator::timestamp().dimmed(), message.cyan()),
                        rng.gen_range(self.config.link_duration_range.clone()),
                        exit_check,
                    )?;
                } else {
                    println!(
                        "{} {} {}",
                        LogGenerator::timestamp().dimmed(),
                        bar.render(percent),
                        message.cyan()
                    );
                    thread::sleep(Duration::from_millis(
                        rng.gen_range(self.config.build_delay_range.clone()),
                    ));
                }
            } else if log.contains("error:") {
                println!(
                    "{} {}",
                    LogGenerator::timestamp().dimmed(),
                    log.bright_red().bold()
                );
                thread::sleep(Duration::from_millis(200));
            } else if in_diagnostic {
                println!(
                    "{} {}",
                    LogGenerator::timestamp().dimmed(),
                    log.bright_red()
                );
                thread::sleep(Duration::from_millis(120));
            } else if log.contains("warning:") || log.contains("deprecated") {
                println!("{} {}", LogGenerator::timestamp().dimmed(), log.yellow());
                thread::sleep(Duration::from_millis(80));
            } else if log.starts_with("-- ") {
                println!(
                    "{} {}",
                    LogGenerator::timestamp().dimmed(),
                    Self::render_probe(log)
                );
                thread::sleep(Duration::from_millis(
                    rng.gen_range(self.config.probe_delay_range.clone()),
                ));
            } else {
                println!("{} {}", LogGenerator::timestamp().dimmed(), log.dimmed());
                thread::sleep(Duration::from_millis(
                    rng.gen_range(self.config.probe_delay_range.clone()),
                ));
            }
        }

        Ok(())
    }
}

impl InstallationStage for CmakeStage {
    fn name(&self) -> &'static str {
        "CMake Project Build"
    }

    fn run(&self, exit_check: &dyn Fn() -> bool) -> io::Result<()> {
        println!("\n{}", format!("> {}", self.name()).bright_yellow().bold());
        println!();

        println!(
            "{} {}",
            LogGenerator::timestamp().dimmed(),
            "cmake -S /usr/src/curl-8.10.1 -B /usr/src/curl-build && cmake --build ."
                .bright_white()
        );
        println!();

        let mut rng = rand::thread_rng();

        if rng.gen_bool(self.config.failure_rate) {
            self.display_logs(self.logs.error_logs(), exit_check)?;

            println!();
            println!(
                "{} {}",
                LogGenerator::timestamp().dimmed(),
                "Build failed. Blaming the compiler cache...".bright_red()
            );

            let mut spinner = Spinner::new();
            spinner.animate(
                "Wiping CMakeFiles and reconfiguring",
                self.config.retry_delay,
                exit_check,
            )?;
            println!();
        }

        self.display_logs(self.logs.success_logs(), exit_check)?;

        println!();
        println!(
            "{} {}",
            LogGenerator::timestamp().dimmed(),
            "Build target installed to /dev/null".bright_green().bold()
        );

        thread::sleep(Duration::from_millis(500));
        Ok(())
    }
}
