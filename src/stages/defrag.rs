use super::InstallationStage;
use crate::config::DefragConfig;
use crate::log_generator::LogGenerator;
use crate::ui::{BlockGrid, Cell, HiddenCursor, ProgressBar, ProgressStyle, Spinner};
use colored::*;
use rand::Rng;
use std::io;
use std::thread;
use std::time::Duration;

struct Transfer {
    source: usize,
    target: usize,
}

pub struct DefragStage {
    config: DefragConfig,
}

impl DefragStage {
    pub fn new(config: DefragConfig) -> Self {
        Self { config }
    }

    fn scatter(&self, grid: &mut BlockGrid) {
        let mut rng = rand::thread_rng();
        let total = grid.len();
        let occupied = (total as f64 * self.config.fill_ratio) as usize;

        for i in 0..total {
            grid.set(i, Cell::Free);
        }

        let mut placed = 0;
        while placed < occupied {
            let index = rng.gen_range(0..total);
            if grid.get(index) == Cell::Free {
                let run = rng.gen_range(1..4);
                for offset in 0..run {
                    let index = index + offset;
                    if index < total && grid.get(index) == Cell::Free && placed < occupied {
                        grid.set(
                            index,
                            if rng.gen_bool(0.45) {
                                Cell::Fragmented
                            } else {
                                Cell::Used
                            },
                        );
                        placed += 1;
                    }
                }
            }
        }

        for _ in 0..self.config.unmovable_count {
            grid.set(rng.gen_range(total / 2..total), Cell::Unmovable);
        }
        if rng.gen_bool(self.config.bad_cluster_chance) {
            grid.set(rng.gen_range(0..total), Cell::Bad);
        }
    }

    fn refragment(&self, grid: &mut BlockGrid, head: usize) {
        let mut rng = rand::thread_rng();
        let total = grid.len();
        let displaced = (head as f64 * self.config.restart_scatter) as usize;

        for _ in 0..displaced {
            let Some(source) = grid.rfind(head, Cell::is_data) else {
                break;
            };
            let target = rng.gen_range(head..total);
            if grid.get(target) == Cell::Free {
                grid.set(target, Cell::Fragmented);
                grid.set(source, Cell::Free);
            }
        }
    }

    fn pending(grid: &BlockGrid) -> usize {
        match grid.find(0, |cell| cell == Cell::Free) {
            Some(head) => (head..grid.len())
                .filter(|i| grid.get(*i).is_data())
                .count(),
            None => 0,
        }
    }

    fn footer(&self, progress: f32, cluster: u64, eta: f64, note: Option<&str>) -> Vec<String> {
        let bar = ProgressBar::new(ProgressStyle::Block);
        let legend = [
            Cell::Used,
            Cell::Fragmented,
            Cell::Unmovable,
            Cell::Free,
            Cell::Reading,
            Cell::Writing,
        ]
        .map(Cell::legend)
        .join("   ");

        vec![
            String::new(),
            format!("  {}", legend),
            String::new(),
            format!(
                "  Reading cluster {} of {}",
                thousands(cluster).bright_white(),
                thousands(self.config.total_clusters).dimmed()
            ),
            format!("  Full Optimization  {}", bar.render(progress)),
            String::new(),
            format!(
                "  Estimated time remaining: {}",
                format_duration(eta).bright_white()
            ),
            match note {
                Some(note) => format!("  {}", note.bright_red().bold()),
                None if eta > self.config.initial_eta_minutes => format!(
                    "  {}",
                    format!(
                        "(was: {})",
                        format_duration(self.config.initial_eta_minutes)
                    )
                    .dimmed()
                ),
                None => String::new(),
            },
        ]
    }
}

impl InstallationStage for DefragStage {
    fn name(&self) -> &'static str {
        "Disk Defragmenter"
    }

    fn run(&self, exit_check: &dyn Fn() -> bool) -> io::Result<()> {
        println!("\n{}", format!("> {}", self.name()).bright_cyan().bold());
        println!();

        let mut rng = rand::thread_rng();

        println!(
            "{} Drive {} ({}, {} clusters)",
            LogGenerator::timestamp().dimmed(),
            self.config.volume.bright_white(),
            self.config.filesystem,
            thousands(self.config.total_clusters)
        );

        let mut spinner = Spinner::new();
        spinner.animate(
            "Analyzing volume information",
            self.config.analyze_delay,
            exit_check,
        )?;

        let fragmentation = rng.gen_range(28..64);
        println!(
            "{} Drive {} is {} fragmented.",
            LogGenerator::timestamp().dimmed(),
            self.config.volume,
            format!("{}%", fragmentation).yellow()
        );
        println!(
            "{} You should defragment this volume now.",
            LogGenerator::timestamp().dimmed()
        );
        println!();
        thread::sleep(Duration::from_millis(900));

        let mut grid = BlockGrid::fit(self.config.grid_max_width, self.config.grid_height);
        self.scatter(&mut grid);

        let total_work = Self::pending(&grid).max(1);
        let batch = (total_work / self.config.target_frames).clamp(
            self.config.clusters_per_frame.start,
            self.config.clusters_per_frame.end,
        );
        let mut eta = self.config.initial_eta_minutes;
        let mut restarts = 0;
        let mut transfers: Vec<Transfer> = Vec::new();
        let mut cluster = 0;
        let _cursor = HiddenCursor::new();

        loop {
            if exit_check() {
                return Err(io::Error::new(io::ErrorKind::Interrupted, "User interrupt"));
            }

            for transfer in transfers.drain(..) {
                grid.set(transfer.target, Cell::Used);
                grid.set(transfer.source, Cell::Free);
            }

            for _ in 0..rng.gen_range(batch..=batch + batch / 2 + 1) {
                let Some(target) = grid.find(0, |cell| cell == Cell::Free) else {
                    break;
                };
                let Some(source) = grid.rfind(grid.len(), Cell::is_data) else {
                    break;
                };
                if source < target {
                    break;
                }

                grid.set(source, Cell::Reading);
                grid.set(target, Cell::Writing);
                transfers.push(Transfer { source, target });
                cluster =
                    (source as f64 / grid.len() as f64 * self.config.total_clusters as f64) as u64;
            }

            let remaining = Self::pending(&grid) + transfers.len();
            let progress = 1.0 - (remaining as f32 / total_work as f32);
            eta = (eta - 0.04).max(1.0);
            if rng.gen_bool(self.config.eta_spike_chance) {
                eta *= rng.gen_range(self.config.eta_spike_factor.clone());
            }

            grid.draw(&self.footer(progress, cluster, eta, None))?;

            if transfers.is_empty() {
                break;
            }

            if restarts < self.config.max_restarts && rng.gen_bool(self.config.restart_chance) {
                let note = "Drive contents changed: restarting defragmentation...";
                grid.draw(&self.footer(progress, cluster, eta, Some(note)))?;
                thread::sleep(Duration::from_millis(self.config.restart_hold));

                for transfer in transfers.drain(..) {
                    grid.set(transfer.source, Cell::Fragmented);
                    grid.set(transfer.target, Cell::Free);
                }
                let head = grid.find(0, |cell| cell == Cell::Free).unwrap_or(0);
                self.refragment(&mut grid, head);

                eta = self.config.initial_eta_minutes;
                restarts += 1;
            }

            thread::sleep(Duration::from_millis(
                rng.gen_range(self.config.frame_delay_range.clone()),
            ));
        }

        grid.draw(&self.footer(1.0, self.config.total_clusters, 0.0, None))?;
        grid.finish();
        drop(_cursor);

        println!();
        println!(
            "{} Defragmentation of drive {} is complete.",
            LogGenerator::timestamp().dimmed(),
            self.config.volume
        );
        if restarts > 0 {
            println!(
                "{} Pass restarted {} time(s) due to volume activity.",
                LogGenerator::timestamp().dimmed(),
                restarts
            );
        }
        println!(
            "{} {}",
            LogGenerator::timestamp().dimmed(),
            "Windows recommends defragmenting this volume again in 3 days."
                .bright_green()
                .bold()
        );

        thread::sleep(Duration::from_millis(500));
        Ok(())
    }
}

fn thousands(value: u64) -> String {
    value
        .to_string()
        .as_bytes()
        .rchunks(3)
        .rev()
        .map(|chunk| String::from_utf8_lossy(chunk).into_owned())
        .collect::<Vec<_>>()
        .join(",")
}

fn format_duration(minutes: f64) -> String {
    let total = minutes.round() as u64;
    if total == 0 {
        return "0 minutes".to_string();
    }

    let parts = [
        (total / 1440, "day"),
        (total % 1440 / 60, "hour"),
        (total % 60, "minute"),
    ];

    parts
        .iter()
        .filter(|(value, _)| *value > 0)
        .map(|(value, unit)| format!("{} {}{}", value, unit, if *value == 1 { "" } else { "s" }))
        .collect::<Vec<_>>()
        .join(" ")
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_thousands() {
        assert_eq!(thousands(0), "0");
        assert_eq!(thousands(512), "512");
        assert_eq!(thousands(2_097_152), "2,097,152");
    }

    #[test]
    fn test_format_duration() {
        assert_eq!(format_duration(0.0), "0 minutes");
        assert_eq!(format_duration(1.0), "1 minute");
        assert_eq!(format_duration(11.4), "11 minutes");
        assert_eq!(format_duration(252.0), "4 hours 12 minutes");
        assert_eq!(format_duration(1500.0), "1 day 1 hour");
    }
}
