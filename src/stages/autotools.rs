use super::InstallationStage;
use crate::config::AutotoolsConfig;
use crate::log_corpus::LogCorpus;
use crate::log_generator::LogGenerator;
use crate::ui::{ProgressBar, ProgressStyle, Spinner};
use colored::*;
use rand::Rng;
use std::io;
use std::thread;
use std::time::Duration;

pub struct AutotoolsStage {
    logs: LogCorpus,
    config: AutotoolsConfig,
}

impl AutotoolsStage {
    pub fn new(config: AutotoolsConfig) -> Self {
        Self {
            logs: LogCorpus::new(
                include_str!("../../data/autotools.log"),
                include_str!("../../data/error/autotools.log"),
            ),
            config,
        }
    }

    /// Colorize a "checking for foo... yes" line so the answer stands out
    fn render_check(log: &str) -> String {
        match log.rsplit_once("... ") {
            Some((question, answer)) => {
                let answer = if answer.starts_with("no") || answer.contains("not found") {
                    answer.yellow()
                } else {
                    answer.green()
                };
                format!("{}... {}", question.dimmed(), answer)
            }
            None => format!("{}", log.dimmed()),
        }
    }

    /// Display a recorded configure/make transcript line by line
    fn display_logs(&self, logs: &[String], exit_check: &dyn Fn() -> bool) -> io::Result<()> {
        let mut rng = rand::thread_rng();
        // Both configure and clang keep talking after the line that failed, so
        // the follow-up lines stay part of the same red block.
        let mut in_diagnostic = false;

        for log in logs {
            if exit_check() {
                return Err(io::Error::new(io::ErrorKind::Interrupted, "User interrupt"));
            }

            let trimmed = log.trim_start();

            if log.contains("error:") {
                in_diagnostic = true;
            } else if log.starts_with("checking ") || log.starts_with("config.status:") {
                in_diagnostic = false;
            }

            if log.starts_with("checking ") {
                println!(
                    "{} {}",
                    LogGenerator::timestamp().dimmed(),
                    Self::render_check(log)
                );
                thread::sleep(Duration::from_millis(
                    rng.gen_range(self.config.check_delay_range.clone()),
                ));
            } else if log.starts_with("configure: error:") || trimmed.starts_with("make: ***") {
                println!(
                    "{} {}",
                    LogGenerator::timestamp().dimmed(),
                    log.bright_red().bold()
                );
                thread::sleep(Duration::from_millis(400));
            } else if in_diagnostic {
                println!(
                    "{} {}",
                    LogGenerator::timestamp().dimmed(),
                    log.bright_red()
                );
                thread::sleep(Duration::from_millis(120));
            } else if log.contains("warning:") {
                println!("{} {}", LogGenerator::timestamp().dimmed(), log.yellow());
                thread::sleep(Duration::from_millis(80));
            } else if trimmed.starts_with("CCLD") || trimmed.starts_with("LD") {
                let progress = ProgressBar::new(ProgressStyle::Block);
                progress.animate(
                    &format!("{} {}", LogGenerator::timestamp().dimmed(), log.cyan()),
                    rng.gen_range(self.config.link_duration_range.clone()),
                    exit_check,
                )?;
            } else if trimmed.starts_with("CC") || trimmed.starts_with("AR") {
                println!("{} {}", LogGenerator::timestamp().dimmed(), log.cyan());
                thread::sleep(Duration::from_millis(
                    rng.gen_range(self.config.compile_delay_range.clone()),
                ));
            } else if log.starts_with("configure:") || log.starts_with("config.status:") {
                println!(
                    "{} {}",
                    LogGenerator::timestamp().dimmed(),
                    log.bright_cyan()
                );
                thread::sleep(Duration::from_millis(60));
            } else {
                println!("{} {}", LogGenerator::timestamp().dimmed(), log.dimmed());
                thread::sleep(Duration::from_millis(
                    rng.gen_range(self.config.check_delay_range.clone()),
                ));
            }
        }

        Ok(())
    }
}

impl InstallationStage for AutotoolsStage {
    fn name(&self) -> &'static str {
        "GNU Autotools Build"
    }

    fn run(&self, exit_check: &dyn Fn() -> bool) -> io::Result<()> {
        println!("\n{}", format!("> {}", self.name()).bright_yellow().bold());
        println!();

        println!(
            "{} {}",
            LogGenerator::timestamp().dimmed(),
            "cd /usr/src/coreutils-9.5 && ./configure --disable-nls".bright_white()
        );
        println!();

        let mut rng = rand::thread_rng();

        if rng.gen_bool(self.config.failure_rate) {
            self.display_logs(self.logs.error_logs(), exit_check)?;

            println!();
            println!(
                "{} {}",
                LogGenerator::timestamp().dimmed(),
                "configure aborted: build dependency missing".bright_red()
            );

            let mut spinner = Spinner::new();
            spinner.animate(
                "Fetching libssl-dev from mirror.oldsoft.org",
                self.config.retry_delay,
                exit_check,
            )?;
            println!();
            println!(
                "{} {}",
                LogGenerator::timestamp().dimmed(),
                "Re-running ./configure --disable-nls".bright_white()
            );
            println!();
        }

        self.display_logs(self.logs.success_logs(), exit_check)?;

        println!();
        println!(
            "{} {}",
            LogGenerator::timestamp().dimmed(),
            "make: build completed, nothing left to install"
                .bright_green()
                .bold()
        );

        thread::sleep(Duration::from_millis(500));
        Ok(())
    }
}
