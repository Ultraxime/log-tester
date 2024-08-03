// @Author: Ultraxime
//
// This file is part of Log Tester.
//
// Log Tester is free software: you can redistribute it and/or modify it
// under the terms of the GNU General Public License as published by the
// Free Software Foundation, either version 3 of the License.
//
// Log Tester is distributed in the hope that it will be useful,
// but WITHOUT ANY WARRANTY; without even the implied warranty
// of MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.
// See the GNU General Public License for more details.
//
// You should have received a copy of the GNU General Public License
// along with Log Tester. If not, see <https://www.gnu.org/licenses/>.

#![doc = include_str!("../README.md")]

use std::sync::RwLock;

use log::{Level, Log, Record};

/// The list of captured logs.
static LOGS: RwLock<Vec<CapturedLog>> = RwLock::new(Vec::new());

static INIT: std::sync::Once = std::sync::Once::new();

/// The logger
pub struct LogTester;

/// A log that was captured
#[derive(Debug)]
pub struct CapturedLog {
    /// The formatted log message.
    pub body: String,
    /// The level.
    pub level: Level,
    /// The target.
    pub target: String,
}

impl LogTester {
    /// Start the logger
    ///
    /// This should only be called once
    ///
    /// # Examples
    ///
    /// ```
    /// use log_tester::LogTester;
    ///
    /// LogTester::start();
    /// ```
    pub fn start() {
        INIT.call_once(|| log::set_logger(&LogTester).expect("Failed to start the logger"));
        log::set_max_level(log::LevelFilter::Trace);
    }

    /// Returns true if there is an entry with the given level that contains the given content
    ///
    /// # Examples
    ///
    /// ```
    /// use log_tester::LogTester;
    /// use log;
    ///
    /// LogTester::start();
    /// log::info!("Hello, world!");
    /// assert!(LogTester::contains(log::Level::Info, "Hello, world!"));
    /// ```
    pub fn contains(level: Level, content: &str) -> bool {
        let logs = LOGS
            .read()
            .expect("Failed to get the read lock on the logs");
        for log in logs.iter() {
            if log.level == level && log.body.contains(content) {
                return true;
            }
        }
        false
    }

    /// Returns the number of captured logs
    ///
    /// # Examples
    ///
    /// ```
    /// use log_tester::LogTester;
    /// use log;
    ///
    /// LogTester::start();
    /// log::info!("Hello, world!");
    /// assert_eq!(LogTester::len(), 1);
    /// ```
    pub fn len() -> usize {
        LOGS.read()
            .expect("Failed to get the read lock on the logs")
            .len()
    }

    /// Clears the captured logs
    ///
    /// # Examples
    ///
    /// ```
    /// use log_tester::LogTester;
    /// use log;
    ///
    /// LogTester::start();
    /// log::info!("Hello, world!");
    /// LogTester::clear();
    /// assert_eq!(LogTester::len(), 0);
    /// ```
    pub fn clear() {
        LOGS.write()
            .expect("Failed to get the write lock on the logs")
            .clear();
    }
}

impl std::fmt::Display for CapturedLog {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.body)
    }
}

impl std::fmt::Debug for LogTester {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let logs = LOGS
            .read()
            .expect("Failed to get the read lock on the logs");
        write!(f, "{:?}", logs)
    }
}

impl std::fmt::Display for LogTester {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let logs = LOGS
            .read()
            .expect("Failed to get the read lock on the logs");
        for log in logs.iter() {
            writeln!(f, "{}", log)?;
        }
        Ok(())
    }
}

impl Log for LogTester {
    fn enabled(&self, _: &log::Metadata) -> bool {
        true
    }

    fn log(&self, record: &Record) {
        LOGS.write()
            .expect("Failed to get the write lock on the logs")
            .push(record.into());
    }

    fn flush(&self) {}
}

impl From<&Record<'_>> for CapturedLog {
    fn from(record: &Record<'_>) -> Self {
        CapturedLog {
            body: record.args().to_string(),
            level: record.level(),
            target: record.target().to_string(),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use log::{debug, error, info, log_enabled, trace, warn};

    #[test]
    fn test_log() {
        LogTester::start();
        assert_eq!(LogTester::len(), 0);
        trace!("trace");
        debug!("debug");
        info!("info");
        warn!("warn");
        error!("error");
        assert!(LogTester::contains(Level::Trace, "trace"));
        assert!(LogTester::contains(Level::Debug, "debug"));
        assert!(LogTester::contains(Level::Info, "info"));
        assert!(LogTester::contains(Level::Warn, "warn"));
        assert!(LogTester::contains(Level::Error, "error"));
        assert!(!LogTester::contains(Level::Info, "nothing"));
        assert_eq!(LogTester::len(), 5);
    }

    #[test]
    fn test_max_level() {
        LogTester::start();
        assert_eq!(log::max_level(), log::LevelFilter::Trace);
        for level in Level::iter() {
            assert!(log_enabled!(level));
        }
    }

    #[test]
    fn test_flush() {
        LogTester::start();
        log::logger().flush();
    }

    #[test]
    fn test_clear() {
        LogTester::start();
        trace!("trace");
        debug!("debug");
        info!("info");
        warn!("warn");
        error!("error");
        assert_eq!(LogTester::len(), 5);
        LogTester::clear();
        assert_eq!(LogTester::len(), 0);
    }

    #[test]
    fn test_display() {
        LogTester::start();
        trace!("trace");
        debug!("debug");
        info!("info");
        warn!("warn");
        error!("error");
        assert!(format!("{}", LogTester).contains("trace"));
        assert!(format!("{}", LogTester).contains("debug"));
        assert!(format!("{}", LogTester).contains("info"));
        assert!(format!("{}", LogTester).contains("warn"));
        assert!(format!("{}", LogTester).contains("error"));
    }

    #[test]
    fn test_debug() {
        LogTester::start();
        trace!("trace");
        debug!("debug");
        info!("info");
        warn!("warn");
        error!("error");
        assert!(format!("{:?}", LogTester).contains("trace"));
        assert!(format!("{:?}", LogTester).contains("debug"));
        assert!(format!("{:?}", LogTester).contains("info"));
        assert!(format!("{:?}", LogTester).contains("warn"));
        assert!(format!("{:?}", LogTester).contains("error"));
    }
}
