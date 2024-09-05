# frontend-minify reduce the size of the files of frontend applications
# Copyright (C) 2024 john-malloc

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
import os

def get_params():
    excluded_files = []
    excluded_licenses = []
    exclude_license_start = 0
    exclude_extreme_start = 0

    for i, arg in enumerate(sys.argv):
        if arg == "--exclude-license":
            exclude_license_start = i
        if arg == "--exclude-extreme":
            exclude_extreme_start = i
    
    if exclude_license_start > exclude_extreme_start:
        excluded_files = sys.argv[exclude_extreme_start + 1: exclude_license_start]
        excluded_licenses = sys.argv[exclude_license_start + 1: len(sys.argv)]
    if exclude_license_start <= exclude_extreme_start:
        excluded_licenses = sys.argv[exclude_license_start + 1: exclude_extreme_start]
        excluded_files = sys.argv[exclude_extreme_start + 1: len(sys.argv)]

    return(excluded_files, excluded_licenses)

excluded_files, excluded_licenses = get_params()

files_path = []
files_path.extend(list(Path(".").rglob("**/*.[hH][tT][mM][lL]")))
files_path.extend(list(Path(".").rglob("**/*.[cC][sS][sS]")))
files_path.extend(list(Path(".").rglob("**/*.[jJ][sS]")))

for file_path in files_path:
    if "min" in str(file_path):
        continue

    excluded_file_bool = False
    for excluded_file in excluded_files:
        if excluded_file == str(file_path):
            excluded_file_bool = True
            break
    
    license_lines_to_skip = 0
    for i in range(0, len(excluded_licenses), 2):
        if excluded_licenses[i] == str(file_path):
            license_lines_to_skip = int(excluded_licenses[i + 1])
            break

    tmp = str(file_path).split(".")
    new_file_path = tmp[0] + ".min." + tmp[1]
    file = open(file_path, "r")
    write_to_file = ""

    copy = True
    for i, line in enumerate(file):
        if i + 1 <= license_lines_to_skip:
            write_to_file += line
            continue

        # HTML comment
        if "<!--" in line and "-->" in line:
            continue
        if "<!--" in line:
            copy = False
        if "-->" in line:
            copy = True
        
        # CSS comment or multiline JS comment
        if "/*" in line and "*/" in line:
            continue
        if "<!--" in line:
            copy = False
        if "-->" in line:
            copy = True

        # single line JS comment
        if "//" in line:
            continue

        # if file is not --exclude-extreme and 
        # is in multiline string, copy without minify
        if excluded_file_bool:
            if "`" in line:
                copy = not copy
            if not copy:
                write_to_file += line

        if copy:
            write_to_file += line.replace("\n", "").replace("\t", "").replace("    ", "")
            
    file.close()
    new_file = open(new_file_path, "w")
    new_file.write(write_to_file)
    new_file.close()

    old_file_size = os.path.getsize(file_path) 
    new_file_size = os.path.getsize(new_file_path)
    print(str(old_file_size - new_file_size) + " bytes saved")
    print("file size reduced by " + str(int((1 - (new_file_size / old_file_size)) * 100)) + "%")