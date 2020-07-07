/*
 * Copyright (c) 2020 Yaguo Zhou
 * rmd is licensed under Mulan PSL v2.
 * You can use this software according to the terms and conditions of the Mulan PSL v2.
 * You may obtain a copy of Mulan PSL v2 at:
 *          http://license.coscl.org.cn/MulanPSL2
 * THIS SOFTWARE IS PROVIDED ON AN "AS IS" BASIS, WITHOUT WARRANTIES OF ANY KIND,
 * EITHER EXPRESS OR IMPLIED, INCLUDING BUT NOT LIMITED TO NON-INFRINGEMENT,
 * MERCHANTABILITY OR FIT FOR A PARTICULAR PURPOSE.
 * See the Mulan PSL v2 for more details.
 */

#[macro_use]
extern crate clap;
#[macro_use]
extern crate horrorshow;

use std::path::Path;

use clap::Arg;

use ui::App;

mod metadata;
mod ui;

fn main() {
    let matches = clap::App::new(crate_name!())
        .version(crate_version!())
        .author(crate_authors!())
        .about(crate_description!())
        .arg(
            Arg::with_name("md_file")
                .required(false)
                .help("markdown file to open"),
        )
        .get_matches();

    match matches.value_of("md_file") {
        Some(md_file) => match Path::new(md_file).canonicalize() {
            Ok(path_buf) => App::new(Some(path_buf)).go(),
            _ => {
                eprintln!("Incorrect path: {}", md_file);
                App::new(None).go();
            }
        },
        None => {
            App::new(None).go();
        }
    }
}
