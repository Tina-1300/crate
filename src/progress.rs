use indicatif::{ProgressBar, ProgressStyle};


pub fn setup_progress_bar(total_items: usize) -> ProgressBar {
    let pb = ProgressBar::new(total_items as u64);
    pb.set_length(100);
    pb.set_style(
        ProgressStyle::default_bar()
            .template("{spinner:.green} [{bar:.cyan}] {pos}/{len}")
            .expect("Error in the progress bar template")
            .progress_chars("=>-"),
    );
    pb
}
