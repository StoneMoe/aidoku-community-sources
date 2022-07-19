use alloc::string::{String, ToString};
use alloc::vec::Vec;
use aidoku::{
    prelude::*,
    error::Result, Manga, MangaContentRating, MangaStatus, MangaViewer,
};
use crate::{Node};

extern crate alloc;

pub struct Resp {
    pub(crate) mangas: Vec<Manga>,
    pub(crate) has_more: bool,
}

pub enum ParseMode {
    Search,
    Filtered,
}

pub fn parse(html: Node, mode: ParseMode) -> Result<Resp> {
    let mut manga_arr: Vec<Manga> = Vec::new();
    let mut has_more: bool = true;

    match mode {
        ParseMode::Search => {
            let node = html.select("div.book-result > ul > li");
            let elems = node.array();
            println!("[zh-manhuagui] found {} manga(s)", elems.len());
            for elem in elems {
                manga_arr.push(parse_list_elem(elem.as_node()));
            }
            has_more = manga_arr.len() < 10;
            Ok(Resp { mangas: manga_arr, has_more })
        }
        ParseMode::Filtered => {
            let node = html.select("ul#contList > li");
            let elems = node.array();
            println!("[zh-manhuagui] found {} manga(s)", elems.len());
            for elem in elems {
                manga_arr.push(parse_list_elem(elem.as_node()));
            }
            has_more = manga_arr.len() < 42;
            Ok(Resp { mangas: manga_arr, has_more })
        }
    }
}

fn parse_list_elem(elem: Node) -> Manga {
    let elem = elem.select("a.bcover").first();
    let m = Manga {
        id: elem.attr("href").read()
            .strip_prefix("/comic/").unwrap()
            .strip_suffix("/").unwrap()
            .to_string(),
        cover: {
            let thumbnail_elem = elem.select("img").first();
            if thumbnail_elem.has_attr("src") {
                thumbnail_elem.attr("abs:src").read()
            } else {
                thumbnail_elem.attr("abs:data-src").read()
            }
        },
        title: elem.attr("title").read().trim().to_string(),
        author: "todo".to_string(),
        artist: "todo".to_string(),
        description: "todo".to_string(),
        url: elem.attr("href").read(),
        categories: Vec::new(),
        status: MangaStatus::Unknown,
        nsfw: MangaContentRating::Safe,
        viewer: MangaViewer::Rtl,
    };
    println!("[zh-manhuagui] [{}]{}", m.id.to_string(), m.title);
    return m;
}