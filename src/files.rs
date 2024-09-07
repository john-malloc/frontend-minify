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
pub fn get_files() -> Vec<std::path::PathBuf> {
    let mut vec: Vec<std::path::PathBuf> = Vec::new();
    let html: &Option<&std::ffi::OsStr> = &Some(std::ffi::OsStr::new("html"));
    let css: &Option<&std::ffi::OsStr> = &Some(std::ffi::OsStr::new("css"));
    let js: &Option<&std::ffi::OsStr> = &Some(std::ffi::OsStr::new("js"));

    for element in walkdir::WalkDir::new(".") {
        let path: std::path::PathBuf = match element {
            Ok(e) => e.into_path(),
            Err(err) => panic!("Failed on reading files -> {}", err),
        };
        if path.to_str().unwrap_or("").contains("min") {
            continue;
        }
        if path.extension().eq(html) || path.extension().eq(css) || path.extension().eq(js) {
            vec.push(path.to_owned());
        }
    }
    return vec;
}
