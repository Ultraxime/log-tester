# log_tester

[![crates.io](https://img.shields.io/crates/v/log_tester.svg)](https://crates.io/crates/log_tester)
[![Documentation](https://docs.rs/log_tester/badge.svg)](https://docs.rs/log_tester)
[![License: GNU GPLv3](https://img.shields.io/badge/License-GPLv3-green.svg)](https://www.gnu.org/licenses/gpl-3.0)

**log_tester** is a crate that takes care of capturing log messages produced by the [`log`](https://docs.rs/log) crate during test, and then perform checks on them.

## Usage

This crate is intend to be used in conjunction with the [`log`](https://docs.rs/log)
 crate. And only during test.

```toml
[dependencies]
log = "0.4"

[dev-dependencies]
log_tester = "0.1"
```

```rust, ignore
use log_tester::LogTester;
use log::Level;

#[test]
fn test_log() {
    LogTester::start();
    log::info!("Hello, world!");

    assert!(LogTester::contains(Level::Info, "Hello, world!"))
}
```

## Contributing

Pull requests are welcome. For major changes, please open an issue first to discuss what you would like to change.

## Authors

- [@Ultraxime](https://github.com/Ultraxime)

## License

This program is free software: you can redistribute it and/or modify it under the terms of the GNU General Public License as published by the Free Software Foundation, either version 3 of the License, or any later version.

This program is distributed in the hope that it will be useful, but WITHOUT ANY WARRANTY; without even the implied warranty of MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE. See the GNU General Public License for more details.

You should have received a copy of the GNU General Public License along with this program. If not, see <https://www.gnu.org/licenses/>.
