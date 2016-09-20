/** 
* Copyright (c) 2016 James Carroll
* image.rs is part of RAW Fire.
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

#[derive(Debug)]
pub struct IO <T> {
    pub i: T,
    pub o: T,
}

impl <T> IO<T> {
    pub fn new(input: T, output: T) -> IO<T> {
        return IO {i: input, o: output};
    }
}

#[derive(Debug)]
pub struct Config {
    pub path: IO<String>,
    pub file_type: IO<String>,
    pub verbose: u8,
    pub gui: bool,
}