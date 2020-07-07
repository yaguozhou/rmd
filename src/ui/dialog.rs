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

use std::path::PathBuf;

use gtk::*;

pub struct OpenDialog(FileChooserDialog);

impl OpenDialog {
    pub fn new() -> Self {
        let file_chooser = FileChooserDialogBuilder::new()
            .action(FileChooserAction::Open)
            .show_hidden(true)
            .build();
        file_chooser.add_button("Cancel", ResponseType::Cancel.into());
        file_chooser.add_button("Open", ResponseType::Ok.into());
        OpenDialog(file_chooser)
    }

    pub fn run(&self) -> Option<PathBuf> {
        if self.0.run() == ResponseType::Ok.into() {
            self.0.get_filename()
        } else {
            None
        }
    }
}

pub struct SaveDialog(FileChooserDialog);

impl SaveDialog {
    pub fn new() -> Self {
        let save_chooser = FileChooserDialogBuilder::new()
            .action(FileChooserAction::Save)
            .show_hidden(true)
            .build();
        save_chooser.add_button("Cancel", ResponseType::Cancel.into());
        save_chooser.add_button("Save", ResponseType::Ok.into());
        SaveDialog(save_chooser)
    }

    pub fn run(&self) -> Option<PathBuf> {
        if self.0.run() == ResponseType::Ok.into() {
            self.0.get_filename()
        } else {
            None
        }
    }
}

impl Drop for OpenDialog {
    fn drop(&mut self) {
        self.0.destroy();
    }
}

impl Drop for SaveDialog {
    fn drop(&mut self) {
        self.0.destroy();
    }
}
