/**
 * frontend-minify reduce the size of the files of frontend applications
 * Copyright (C) 2024 john-malloc
 *
 * This program is free software: you can redistribute it and/or modify
 * it under the terms of the GNU General Public License as published by
 * the Free Software Foundation, either version 3 of the License, or
 * (at your option) any later version.
 *
 * This program is distributed in the hope that it will be useful,
 * but WITHOUT ANY WARRANTY; without even the implied warranty of
 * MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
 * GNU General Public License for more details.
 *
 * You should have received a copy of the GNU General Public License
 * along with this program.  If not, see <https://www.gnu.org/licenses/>.
 */
pub fn get_files() -> Vec<String> {
    let mut vec: Vec<String> = Vec::new();
    for element in walkdir::WalkDir::new(".") {
        let entry: walkdir::DirEntry = match element {
            Ok(e) => e,
            Err(err) => panic!("Failed on reading files -> {}", err),
        };
        let file_name: String = entry
            .path()
            .file_name()
            .and_then(std::ffi::OsStr::to_str)
            .unwrap_or("")
            .to_owned();
        if file_name.ends_with("min.html")
            || file_name.ends_with("min.css")
            || file_name.ends_with("min.js")
            || !file_name.contains(".")
        {
            continue;
        }
        if file_name.ends_with("html") || file_name.ends_with("css") || file_name.ends_with("js") {
            vec.push(file_name);
        }
    }
    return vec;
}
