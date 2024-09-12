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

#[derive(Clone)]
pub struct LicenseCmd {
    pub file_name: String,
    pub num_line: usize,
}

pub struct CmdArgs {
    pub no_extreme: Vec<String>,
    pub license_lines: Vec<LicenseCmd>,
}

pub fn cmd() -> CmdArgs {
    let cmd = clap::command!()
        .about("about")
        .arg(
            clap::Arg::new("no_extreme")
                .num_args(1..)
                .long("no-extreme")
                .aliases(["noextreme"])
                .help("no-extreme help"),
        )
        .arg(
            clap::Arg::new("license_lines")
                .num_args(1..)
                .long("license-lines")
                .aliases(["licenselines"])
                .help("license-lines help"),
        )
        .get_matches();

    let mut no_ext: Vec<String> = Vec::new();
    let no_ext_val_ref: clap::parser::ValuesRef<'_, String> =
        cmd.get_many::<String>("no_extreme").unwrap_or_default();
    for x in no_ext_val_ref {
        no_ext.push(x.to_string());
    }

    let mut lic_lin: Vec<LicenseCmd> = Vec::new();
    let lic_lin_val_ref: clap::parser::ValuesRef<'_, String> =
        cmd.get_many::<String>("license_lines").unwrap_or_default();
    let mut tmp = LicenseCmd {
        file_name: String::new(),
        num_line: 0,
    };
    for (i, x) in lic_lin_val_ref.enumerate() {
        if i % 2 == 0 {
            tmp.file_name = x.to_string();
        } else {
            tmp.num_line = x.trim().parse().expect("Failed on casting");
            lic_lin.push(tmp.clone());
        }
    }

    return CmdArgs {
        no_extreme: no_ext,
        license_lines: lic_lin,
    };
}
