# frontend-minify reduce the size of the files of frontend applications
# Copyright (C) 2024  john-malloc

# This program is free software: you can redistribute it and/or modify
# it under the terms of the GNU General Public License as published by
# the Free Software Foundation, either version 3 of the License, or
# (at your option) any later version.

# This program is distributed in the hope that it will be useful,
# but WITHOUT ANY WARRANTY; without even the implied warranty of
# MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
# GNU General Public License for more details.

# You should have received a copy of the GNU General Public License
# along with this program.  If not, see <https://www.gnu.org/licenses/>.

import sys
from pathlib import Path

excluded_files = []
if len(sys.argv) > 1:
    if sys.argv[1] == "--exclude-extreme":
        excluded_files = sys.argv[2:]

files_path = []
files_path.extend(list(Path(".").rglob("**/*.[hH][tT][mM][lL]")))
files_path.extend(list(Path(".").rglob("**/*.[cC][sS][sS]")))
files_path.extend(list(Path(".").rglob("**/*.[jJ][sS]")))

for file_path in files_path:
    if "min" in str(file_path):
        continue

    found = False
    for excluded_file in excluded_files:
        if excluded_file == str(file_path):
            found = True
    
    tmp = str(file_path).split(".")
    file = open(file_path, "r")
    write_to_file = ""
    if not found:
        write_to_file = file.read().replace("\n", "").replace("\t", "").replace("    ", "")
    else:
        is_multiline_string = False
        for line in file.readlines():
            if "`" in line:
                is_multiline_string = not is_multiline_string
            if is_multiline_string:
                write_to_file += line
            else:
                write_to_file += line.replace("\n", "").replace("\t", "").replace("    ", "")
    file.close()
    new_file = open(tmp[0] + ".min." + tmp[1], "w")
    new_file.write(write_to_file)
    new_file.close()