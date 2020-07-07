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

use std::sync::Arc;
use std::sync::RwLock;

use gtk::*;
use horrorshow::helper::doctype;
use horrorshow::Raw;
use pulldown_cmark::html;
use pulldown_cmark::Parser;
use sourceview::*;
use webkit2gtk::*;

use crate::metadata::Metadata;

use super::utils::*;

const CSS: &[u8] = b"textview { font-family: Source Code Pro; font-size: 11pt; }";

pub struct Body {
    pub paned: Paned,
    pub editor: Editor,
    pub preview: WebView,
}

impl Body {
    pub fn new() -> Body {
        let paned = Paned::new(Orientation::Horizontal);
        let editor = Editor::new();
        let preview = WebView::new();

        paned.pack1(&editor.scrolled_win, true, true);
        paned.pack2(&preview, true, true);

        editor.scrolled_win.set_size_request(100, -1);
        preview.set_size_request(100, -1);
        preview.load_html(&render_default(), None);

        Body {
            paned,
            editor,
            preview,
        }
    }

    pub fn editor_changed(&self, current: Arc<RwLock<Option<Metadata>>>, save_btn: Button) {
        let preview = self.preview.clone();
        self.editor.buff.connect_changed(move |buff| {
            if let Some(markdown_str) = get_content(&buff) {
                if markdown_str.is_empty() {
                    preview.load_html(&render_default(), None);
                } else {
                    preview.load_html(&render(&markdown_str), None);
                }

                if let Some(ref current) = *current.read().unwrap() {
                    save_btn.set_sensitive(!current.is_same_as(markdown_str.as_bytes()));
                }
            }
        });
    }
}

pub struct Editor {
    pub scrolled_win: ScrolledWindow,
    pub buff: Buffer,
    pub view: View,
}

impl Editor {
    fn new() -> Self {
        let scrolled_win = ScrolledWindowBuilder::new().build();

        let buff = BufferBuilder::new()
            .highlight_syntax(true)
            .highlight_matching_brackets(true)
            .language(&LanguageManager::new().get_language("markdown").unwrap())
            .build();

        let view = View::new_with_buffer(&buff);
        scrolled_win.add(&view);

        // view settings
        view.set_show_line_numbers(true);
        view.set_monospace(true);
        view.set_left_margin(10);
        view.set_auto_indent(true);
        view.set_insert_spaces_instead_of_tabs(true);
        view.set_indent_width(4);
        view.set_highlight_current_line(true);
        view.set_smart_backspace(true);
        view.set_wrap_mode(WrapMode::Word);

        let css = CssProvider::new();
        css.load_from_data(CSS).expect("load css error");
        view.get_style_context()
            .add_provider(&css, STYLE_PROVIDER_PRIORITY_APPLICATION);

        Editor {
            scrolled_win,
            view,
            buff,
        }
    }
}

fn render_default() -> String {
    format!(
        "{}",
        html!(
            : doctype::HTML;
            html {
                head {
                }
                body {
                   : Raw(&markdown_to_html("Edit left, Live Preview Here."));
                }
            }
        )
    )
}

fn render(s: &str) -> String {
    format!(
        "{}",
        html!(
            : doctype::HTML;
            html {
                head {
                    link(rel="stylesheet", href="https://cdnjs.cloudflare.com/ajax/libs/highlight.js/10.1.1/styles/github.min.css") {}
                    script(src="https://cdnjs.cloudflare.com/ajax/libs/highlight.js/10.1.1/highlight.min.js") {}
                    script(src="https://cdnjs.cloudflare.com/ajax/libs/highlight.js/10.1.1/languages/rust.min.js") {}
                    script {
                        : Raw("hljs.initHighlightingOnLoad()")
                    }
                    style {
                        : "body { width: 90%; margin: 0 auto }";
                        : "img { max-width: 90% }"
                    }
                }
                body {
                    : Raw(&markdown_to_html(s));
                }
            }
        )
    )
}

fn markdown_to_html(s: &str) -> String {
    let parser = Parser::new(&s);
    let mut result = String::new();
    html::push_html(&mut result, parser);
    result
}
