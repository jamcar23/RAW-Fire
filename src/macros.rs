/** 
* Copyright (c) 2016 James Carroll
* macros.rs is part of RAW Fire.
*
* RAW Fire is free software: you can redistribute it and/or modify
* it under the terms of the GNU General Public License as published by
* the Free Software Foundation, either version 3 of the License, or
* (at your option) any later version.
*
* RAW Fire is distributed in the hope that it will be useful,
* but WITHOUT ANY WARRANTY; without even the implied warranty of
* MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
* GNU General Public License for more details.
*
* You should have received a copy of the GNU General Public License
* along with RAW Fire.  If not, see <http://www.gnu.org/licenses/>.
*/

/// Macro for printing, using ```println!```, if $verb is above a certain level.
/// Convenient for printing at different verbose / log levels.
///
/// See [println!](https://doc.rust-lang.org/src/std/up/src/libstd/macros.rs.html#118-121) for more.
///
/// # Args
///
/// $lvl -- The level needed for this log to be printed.
/// $verb -- The user defined verbosity.
/// $s -- The expression normally passed into ```println!```.
/// $arg -- The args that will be passed into ```println!```.
///
/// # Panics
/// Panics if writing to `io::stdout()` fails.
///
/// # Examples
/// Here ```conf.verbose``` is an int representing the level of detail to log.
///
///
/// ```
/// log!(1, conf.verbose, "Level 1: Hello {}", "World");
/// log!(2, conf.verbose, "Level 2: Hello {}", "World 2");
/// ```

#[macro_export]
macro_rules! log {
    ($lvl:expr, $verb:expr, $s:expr) => ({
        if $verb >= $lvl {
            println!($s);
        }
    });
    ($lvl:expr, $verb:expr, $s:expr, $($arg:tt)*) => ({
        if $verb >= $lvl {
            println!($s, $($arg)*);
        }
    });
}
