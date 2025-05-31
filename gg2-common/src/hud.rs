use std::time::Duration;

#[derive(Debug, Clone)]
pub enum GamemodeHud {
    CaptureTheFlag {
        /// The starting time of the match timer
        time_limit: Duration,
        /// The time left on the match timer
        time_limit_left: Duration,
    },
}
