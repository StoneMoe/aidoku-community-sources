use crate::Node;
use aidoku::{error::Result, prelude::*, Manga, MangaContentRating, MangaStatus, MangaViewer};
use alloc::string::{String, ToString};
use alloc::vec::Vec;

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
	let has_more: bool;

	match mode {
		ParseMode::Search => {
			let node = html.select("div.book-result > ul > li");
			let elems = node.array(); // workaround: new var to avoid value dropping
			println!("[manhuagui]found {} manga(s)", elems.len());

			for elem in elems {
				manga_arr.push(parse_list_elem(elem.as_node()));
			}
			println!("[manhuagui]loaded {} manga(s)", manga_arr.len());

			has_more = manga_arr.len() >= 10;
			Ok(Resp {
				mangas: manga_arr,
				has_more,
			})
		}
		ParseMode::Filtered => {
			let node = html.select("ul#contList > li");
			let elems = node.array(); // workaround: new var to avoid value dropping
			for elem in elems {
				let node = elem.as_node();
				manga_arr.push(parse_list_elem(node));
			}
			println!("[manhuagui]loaded {} manga(s)", manga_arr.len());
			has_more = manga_arr.len() >= 42;
			Ok(Resp {
				mangas: manga_arr,
				has_more,
			})
		}
	}
}

fn parse_list_elem(elem: Node) -> Manga {
	let elem = elem.select("a.bcover");
	let elem = elem.first(); // workaround: break chaining to save my data
	let m = Manga {
		id: elem
			.attr("href")
			.read()
			.strip_prefix("/comic/")
			.unwrap()
			.strip_suffix("/")
			.unwrap()
			.to_string(),
		cover: {
			let cover_elem = elem.select("img");
			let cover_elem = cover_elem.first(); // workaround: chaining calls cause cancer
			if cover_elem.has_attr("src") {
				cover_elem.attr("abs:src").read()
			} else {
				cover_elem.attr("abs:data-src").read()
			}
		},
		title: elem.attr("title").read().trim().to_string(),
		author: String::new(),
		artist: String::new(),
		description: String::new(),
		url: elem.attr("href").read(),
		categories: Vec::new(),
		status: MangaStatus::Unknown,
		nsfw: MangaContentRating::Safe,
		viewer: MangaViewer::Default,
	};
	return m;
}
