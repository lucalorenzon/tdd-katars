use std::time::Duration;
use serde::Serialize;

#[allow(dead_code)]
const RESET: &'static str = "\x1b[0m";
#[allow(dead_code)]
const GREEN: &'static str = "\x1b[32m";
#[allow(dead_code)]
const CYAN: &'static str = "\x1b[36m";
#[allow(dead_code)]
const RED: &'static str = "\x1b[31m";
#[allow(dead_code)]
const YELLOW: &'static str = "\x1b[33m";
#[allow(dead_code)]
const WHITE: &'static str = "\x1b[37m";

#[allow(dead_code)]
pub async fn delay(ms: u32) -> u32 {
    tokio::time::sleep(Duration::from_millis(100)).await;
    ms
}
#[allow(dead_code)]
pub fn array_from(start: u32, count: u32) -> Vec<u32> {
    (start..(start + count)).collect()
}
#[allow(dead_code)]
pub fn pretty<T>(obj: &T) -> String
where T: ?Sized + Serialize, {
    serde_json::to_string(obj).unwrap_or("".to_owned())
}
#[allow(dead_code)]
pub fn green(message: &str) -> String {
    format!("{GREEN}{message}{RESET}")
}
#[allow(dead_code)]
pub fn red(message: &str) -> String {
    format!("{RED}{message}{RESET}")
}

#[allow(dead_code)]
pub fn yellow(message: &str) -> String {
    format!("{YELLOW}{message}{RESET}")
}
#[allow(dead_code)]
pub fn cyan(message: &str) -> String {
    format!("{CYAN}{message}{RESET}")
}
#[allow(dead_code)]
pub fn white(message: &str) -> String {
    format!("{WHITE}{message}{RESET}")
}


#[cfg(test)]
mod tests {
    use crate::helpers::delay;

    #[tokio_macros::test]
    async fn should_it_work() {
        delay(100).await;
        assert_eq!("a","a");
    }
}