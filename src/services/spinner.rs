use indicatif::{ProgressBar, ProgressStyle};
use std::time::Duration;

pub struct Spinner {
    progress_bar: ProgressBar,
}

impl Spinner {
    pub fn new(message: &str) -> Self {
        let pb = ProgressBar::new_spinner();
        pb.enable_steady_tick(Duration::from_millis(100));
        pb.set_style(
            ProgressStyle::with_template("{msg:.cyan}{spinner:.cyan}")
                .unwrap()
                // For more spinners check out the cli-spinners project:
                // https://github.com/sindresorhus/cli-spinners/blob/master/spinners.json
                .tick_strings(&[
                    "▐⠂       ▌",
                    "▐⠈       ▌",
                    "▐ ⠂      ▌",
                    "▐ ⠠      ▌",
                    "▐  ⡀     ▌",
                    "▐  ⠠     ▌",
                    "▐   ⠂    ▌",
                    "▐   ⠈    ▌",
                    "▐    ⠂   ▌",
                    "▐    ⠠   ▌",
                    "▐     ⡀  ▌",
                    "▐     ⠠  ▌",
                    "▐      ⠂ ▌",
                    "▐      ⠈ ▌",
                    "▐       ⠂▌",
                    "▐       ⠠▌",
                    "▐       ⡀▌",
                    "▐      ⠠ ▌",
                    "▐      ⠂ ▌",
                    "▐     ⠈  ▌",
                    "▐     ⠂  ▌",
                    "▐    ⠠   ▌",
                    "▐    ⡀   ▌",
                    "▐   ⠠    ▌",
                    "▐   ⠂    ▌",
                    "▐  ⠈     ▌",
                    "▐  ⠂     ▌",
                    "▐ ⠠      ▌",
                    "▐ ⡀      ▌",
                    "▐⠠       ▌",
                ]),
        );
        pb.set_message(message.to_string());
        Self { progress_bar: pb }
    }

    pub fn finish(self) {
        self.progress_bar.finish_and_clear();
    }
}

pub fn create_reasoning_spinner() -> Spinner {
    Spinner::new("Thinking")
}

pub fn create_api_spinner() -> Spinner {
    Spinner::new("Sending request to API")
}
