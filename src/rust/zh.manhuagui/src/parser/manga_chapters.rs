use aidoku::prelude::println;
use aidoku::std::html::Node;
use aidoku::std::print;
use aidoku::{error::Result, Chapter};
use alloc::string::{String, ToString};
use alloc::vec::Vec;

pub fn parse(manga_id: String, manga_url: String, html: Node) -> Result<Vec<Chapter>> {
	let mut chap_list = Vec::new();

	let html = html.select("ul > li > a.status0");
	let elems = html.array();
	println!("[manhuagui]found {} chapter(s)", elems.len());

	for elem in elems {
		let elem = elem.as_node();
		chap_list.push(Chapter {
			id: {
				elem.attr("href")
					.read()
					.strip_prefix("/comic/")
					.unwrap()
					.strip_suffix(".html")
					.unwrap()
					.to_string()
			},
			title: elem.attr("title").read().trim().to_string(),
			volume: -1.0,
			chapter: -1.0,
			date_updated: -1.0,
			scanlator: String::new(),
			url: elem.attr("abs:href").read(),
			lang: String::from("zh"),
		});
	}

	println!("[manhuagui]loaded {} chapter(s)", chap_list.len());
	Ok(chap_list)
}
