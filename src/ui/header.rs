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

use std::fs::{File, OpenOptions};
use std::io::{Read, Write};
use std::path::PathBuf;
use std::sync::{Arc, RwLock};

use gtk::*;
use sourceview::*;

use crate::metadata::Metadata;

use super::dialog::OpenDialog;
use super::dialog::SaveDialog;
use super::utils::*;

pub struct Header {
    pub header_bar: HeaderBar,
    pub open_btn: Button,
    pub save_btn: Button,
    pub save_as_btn: Button,
}

impl Header {
    pub fn new() -> Header {
        let header_bar = HeaderBarBuilder::new()
            .title(crate_name!())
            .show_close_button(true)
            .build();

        let open_btn = Button::new_with_mnemonic("_Open");
        let save_btn = Button::new_with_mnemonic("_Save");
        let save_as_btn = Button::new_with_mnemonic("Save _As");

        header_bar.pack_start(&open_btn);
        header_bar.pack_start(&save_btn);
        header_bar.pack_start(&save_as_btn);

        Header {
            header_bar,
            open_btn,
            save_btn,
            save_as_btn,
        }
    }

    pub fn open_clicked(&self, current: Arc<RwLock<Option<Metadata>>>, buff: Buffer) {
        let header_bar = self.header_bar.clone();
        self.open_btn
            .connect_clicked(move |_| open(current.clone(), None, &buff, &header_bar));
    }

    pub fn save_event(
        &self,
        current: Arc<RwLock<Option<Metadata>>>,
        buff: Buffer,
        actual_btn: Button,
        save_as: bool,
    ) {
        let header_bar = self.header_bar.clone();
        let save_btn = self.save_btn.clone();
        actual_btn.connect_clicked(move |_| {
            save(current.clone(), &buff, &header_bar, &save_btn, save_as)
        });
    }
}

pub fn open(
    current: Arc<RwLock<Option<Metadata>>>,
    md_file: Option<PathBuf>,
    buff: &Buffer,
    header_bar: &HeaderBar,
) {
    let path_buf;
    {
        if let Some(p) = md_file {
            path_buf = p;
        } else {
            let open_dialog = OpenDialog::new();
            if let Some(new_p) = open_dialog.run() {
                path_buf = new_p;
            } else {
                return;
            }
        }
    }

    if let Ok(mut file) = File::open(&path_buf) {
        let mut file_body = String::new();
        file.read_to_string(&mut file_body).expect("read error.");

        // update header subtitle
        header_bar.set_subtitle(Some(&path_buf.to_string_lossy().as_ref()));
        // store file metadata
        *current.write().unwrap() = Some(Metadata::new(path_buf, file_body.as_bytes()));
        // update editor
        buff.set_text(&file_body);
    }
}

pub fn save(
    current: Arc<RwLock<Option<Metadata>>>,
    buff: &Buffer,
    header_bar: &HeaderBar,
    save_btn: &Button,
    save_as: bool,
) {
    if let Some(text) = get_content(&buff) {
        if !save_as {
            if let Some(ref mut metadata) = *current.write().unwrap() {
                write_to_file(metadata.get_path(), text.as_bytes());
                metadata.set_data(text.as_bytes());
                save_btn.set_sensitive(false);
                return;
            }
        }
        // open dialog when saving an unsaved content OR saving as
        let save_dialog = SaveDialog::new();
        if let Some(new_path) = save_dialog.run() {
            write_to_file(&new_path, text.as_bytes());
            header_bar.set_subtitle(Some(&new_path.to_string_lossy().as_ref()));
            *current.write().unwrap() = Some(Metadata::new(new_path, text.as_bytes()));
            save_btn.set_sensitive(false);
        }
    }
}

fn write_to_file(path_buf: &PathBuf, data: &[u8]) {
    let mut file = OpenOptions::new()
        .write(true)
        .truncate(true)
        .create(true)
        .open(&path_buf)
        .expect(format!("Error opening file {}", &path_buf.to_string_lossy()).as_str());

    file.write_all(data)
        .expect(format!("Error writing file {}", &path_buf.to_string_lossy()).as_str());
}
