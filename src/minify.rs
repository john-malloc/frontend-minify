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

fn new_file_name(path: &String) -> String {
    let _ = std::fs::create_dir("./build");
    // match std::fs::create_dir("./build") {
    //     Ok(_) => (),
    //     Err(err) => panic!("Failed on create build directory -> {}", err),
    // };

    let mut file_name: String = path.to_string();

    let slash_idx: usize = match path.find("/") {
        Some(idx) => idx,
        None => panic!("Failed on file has no slash"),
    };
    file_name.insert_str(slash_idx, "/build");

    let dot_idx: usize = match file_name.rfind(".") {
        Some(idx) => idx,
        None => panic!("Failed on file has no extention"),
    };
    file_name.insert_str(dot_idx, ".min");

    return file_name;
}

pub async fn minify(path: String, no_extreme: bool, license_lines: usize) {
    let mut my_str: String = String::new();
    let mut copy: bool = true;
    let file_content: String = match std::fs::read_to_string(std::path::Path::new(&path)) {
        Ok(f) => f,
        Err(err) => panic!("Failed on read file -> {}", err),
    };

    for (i, tmp_line) in file_content.lines().enumerate() {
        // Line terminators are not included in the lines
        // returned by the iterator ".lines()"
        let mut line: String = tmp_line.to_string();
        // LICENSE LINES
        if i < license_lines {
            line.push('\n');
            my_str.push_str(&line);
            continue;
        }
        // HTML COMMENT
        if line.contains("<!--") && line.contains("-->") {
            continue;
        }
        if line.contains("<!--") {
            copy = false;
        }
        if line.contains("-->") {
            copy = false;
        }
        // CSS / JS COMMENT
        if line.contains("/*") && line.contains("*/") {
            continue;
        }
        if line.contains("/*") {
            copy = false;
        }
        if line.contains("*/") {
            copy = false;
        }
        if line.contains("//") {
            continue;
        }
        // multi line string
        if no_extreme && line.contains("`") {
            copy = !copy;
        }
        if copy {
            my_str.push_str(&line.replace("\t", "").replace("    ", ""));
        } else {
            line.push('\n');
            my_str.push_str(&line);
        }
    }

    let mut file = match std::fs::File::create(new_file_name(&path)) {
        Ok(f) => f,
        Err(err) => panic!("Failed on create new file -> {}", err),
    };

    match std::io::Write::write_all(&mut file, my_str.as_bytes()) {
        Ok(_) => (),
        Err(err) => panic!("Failed on write to file -> {}", err),
    }
}
