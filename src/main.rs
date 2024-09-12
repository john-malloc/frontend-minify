// frontend-minify reduce the size of the files of frontend applications
// Copyright (C) 2024 john-malloc

// This program is free software: you can redistribute it and/or modify
// it under the terms of the GNU General Public License as published by
// the Free Software Foundation, either version 3 of the License, or
// (at your option) any later version.

// This program is distributed in the hope that it will be useful,
// but WITHOUT ANY WARRANTY; without even the implied warranty of
// MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
// GNU General Public License for more details.

// You should have received a copy of the GNU General Public License
// along with this program.  If not, see <https://www.gnu.org/licenses/>.

#![allow(clippy::style)]
#[warn(clippy::return_self_not_must_use)]
mod cmd;
mod files;
mod minify;

fn spawn_thread(file_path: String, args: &cmd::CmdArgs) -> tokio::task::JoinHandle<()> {
    let mut no_extreme: bool = false;
    for arg in &args.no_extreme {
        if file_path.ends_with(arg) {
            no_extreme = true;
        }
    }

    let mut license_lines: usize = 0;
    for arg in &args.license_lines {
        if file_path.ends_with(&arg.file_name) {
            license_lines = arg.num_line;
        }
    }

    return tokio::task::spawn(minify::minify(file_path, no_extreme, license_lines));
}

#[tokio::main]
async fn main() {
    let args: cmd::CmdArgs = cmd::cmd();
    let tasks: Vec<tokio::task::JoinHandle<()>> = files::get_files()
        .into_iter()
        .map(|file_path: String| spawn_thread(file_path, &args))
        .collect();
    for task in tasks {
        match task.await {
            Ok(_) => (),
            Err(err) => panic!("Failed on join thread -> {}", err),
        };
    }
}
