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
use std::process::exit;
use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::{Arc, RwLock};

use gdk;
use gdk::enums::key;
use gdk::*;
use gtk::Window;
use gtk::*;

use crate::metadata::Metadata;

use super::body::Body;
use super::header;
use super::header::Header;

pub struct App {
    window: Window,
    header: Header,
    body: Body,
    md_file: Option<PathBuf>,
}

impl App {
    pub fn new(md_file: Option<PathBuf>) -> Self {
        // init gtk
        if gtk::init().is_err() {
            eprintln!("Failed to initialize GTK Application.");
            exit(1);
        }

        let window = WindowBuilder::new().title(crate_name!()).build();
        let header = Header::new();
        let body = Body::new();
        window.set_titlebar(Some(&header.header_bar));
        window.add(&body.paned);
        window.maximize();

        window.connect_delete_event(move |_, _| {
            main_quit();
            Inhibit(false)
        });

        App {
            window,
            header,
            body,
            md_file,
        }
    }

    pub fn go(&self) {
        self.add_events();
        self.window.show_all();
        gtk::main();
    }

    fn add_events(&self) {
        let current = Arc::new(RwLock::new(None));
        let fullscreen = Arc::new(AtomicBool::new(false));

        // open
        self.header
            .open_clicked(current.clone(), self.body.editor.buff.clone());

        // save
        self.header.save_event(
            current.clone(),
            self.body.editor.buff.clone(),
            self.header.save_btn.clone(),
            false,
        );

        // save as
        self.header.save_event(
            current.clone(),
            self.body.editor.buff.clone(),
            self.header.save_as_btn.clone(),
            true,
        );

        // editor
        self.body
            .editor_changed(current.clone(), self.header.save_btn.clone());

        // key binding
        self.key_pressed(current.clone(), fullscreen.clone());

        // load initial md file if provided
        if let Some(path_buf) = &self.md_file {
            let buff = self.body.editor.buff.clone();
            let header_bar = self.header.header_bar.clone();
            header::open(
                current.clone(),
                Some(path_buf.to_path_buf()),
                &buff,
                &header_bar,
            );
        }
    }

    fn key_pressed(&self, current: Arc<RwLock<Option<Metadata>>>, fullscreen: Arc<AtomicBool>) {
        let buff = self.body.editor.buff.clone();
        let header_bar = self.header.header_bar.clone();
        let save_btn = self.header.save_btn.clone();
        self.window.connect_key_press_event(move |window, gdk| {
            match gdk.get_keyval() {
                key::F11 => {
                    if fullscreen.fetch_xor(true, Ordering::SeqCst) {
                        window.unfullscreen();
                    } else {
                        window.fullscreen();
                    }
                }
                // ctrl + s
                key if key == 's' as u32
                    && gdk.get_state().contains(ModifierType::CONTROL_MASK) =>
                {
                    header::save(current.clone(), &buff, &header_bar, &save_btn, false);
                }
                // ctrl + shift + s
                key if key == 'S' as u32
                    && gdk.get_state().contains(ModifierType::CONTROL_MASK) =>
                {
                    header::save(current.clone(), &buff, &header_bar, &save_btn, true);
                }
                // ctrl + o
                key if key == 'o' as u32
                    && gdk.get_state().contains(ModifierType::CONTROL_MASK) =>
                {
                    header::open(current.clone(), None, &buff, &header_bar);
                }
                _ => (),
            }

            Inhibit(false)
        });
    }
}
