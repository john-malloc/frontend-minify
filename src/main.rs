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
mod files;
mod minify;

#[tokio::main]
async fn main() {
    let tasks: Vec<tokio::task::JoinHandle<()>> = files::get_files()
        .into_iter()
        .map(|file_path: std::path::PathBuf| {
            tokio::task::spawn(minify::minify(file_path, false, 0))
        })
        .collect();
    for task in tasks {
        match task.await {
            Ok(_) => (),
            Err(err) => panic!("Failed on join thread -> {}", err),
        };
    }
}
