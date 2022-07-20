use aidoku::prelude::println;
use aidoku::std::html::Node;
use aidoku::{error::Result, Manga, MangaContentRating, MangaStatus, MangaViewer};
use alloc::string::{String, ToString};
use alloc::vec::Vec;

pub fn parse(manga_id: String, manga_url: String, html: Node) -> Result<Manga> {
	Ok(Manga {
		id: manga_id.clone(),
		cover: {
			let html = html.select("p.hcover > img");
			let html = html.first();
			html.attr("abs:src").read()
		},
		title: {
			let html = html.select("div.book-title > h1:nth-child(1)");
			let html = html.first();
			html.text().read()
		},
		author: {
			let html = html.select("span:contains(漫画作者) > a , span:contains(漫畫作者) > a");
			let html = html.array(); // todo: multiple author need array join
			let mut authors = Vec::new();
			for author in html {
				authors.push(author.as_node().text().read().trim().to_string())
			}
			authors.join(", ")
		},
		artist: String::new(),
		description: {
			let html = html.select("div#intro-all");
			html.text().read()
		},
		url: manga_url,
		categories: {
			let mut c = Vec::new();
			c.push(
				html.select("span:contains(漫画剧情) > a , span:contains(漫畫劇情) > a")
					.text()
					.read(),
			);
			c
		},
		status: match {
			let html = html.select("div.book-detail > ul.detail-list > li.status > span > span");
			let html = html.first();
			html.text().read().as_str()
		} {
			"连载中" => MangaStatus::Ongoing,
			"已完结" => MangaStatus::Completed,
			"連載中" => MangaStatus::Ongoing,
			"已完結" => MangaStatus::Completed,
			_ => MangaStatus::Unknown,
		},
		nsfw: match {
			let html = html.select(".chapter-tip-18");
			let r18_warning = html.array();
			r18_warning.len() != 0
		} {
			true => MangaContentRating::Nsfw,
			false => MangaContentRating::Safe,
		},
		viewer: MangaViewer::Default,
	})
}
