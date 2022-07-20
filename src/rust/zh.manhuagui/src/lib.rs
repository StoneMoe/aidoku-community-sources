/*	Created by reference to https://github.com/tachiyomiorg/tachiyomi-extensions/tree/master/src/zh/manhuagui
 *	All credit goes to their outstanding work.
 */
#![no_std]
extern crate alloc;

use crate::consts::BASE_URL;
use crate::helper::{fetch_chapter_list, fetch_manga_detail};
use crate::parser::manga_list;
use aidoku::std::html::Node;
use aidoku::{
	error::Result,
	prelude::*,
	std::net::Request,
	std::{String, Vec},
	Chapter, DeepLink, Filter, Manga, MangaPageResult, Page,
};

mod consts;
mod helper;
mod parser;
mod utils;

#[get_manga_list]
pub fn get_manga_list(filters: Vec<Filter>, page: i32) -> Result<MangaPageResult> {
	let list_resp = helper::fetch_manga_list(filters, page)?;
	Ok(MangaPageResult {
		manga: list_resp.mangas,
		has_more: list_resp.has_more,
	})
}

#[get_manga_details]
fn get_manga_details(id: String) -> Result<Manga> {
	fetch_manga_detail(id)
}

#[get_chapter_list]
fn get_chapter_list(id: String) -> Result<Vec<Chapter>> {
	fetch_chapter_list(id)
}

#[get_page_list]
fn get_page_list(id: String) -> Result<Vec<Page>> {
	todo!()
	// // Not Tested
	// // Maybe only use the first one.
	//
	// let url = [
	//     format!("{}/{}.html", API_PAGELIST_WEBVIEW_URL, &id),
	//     format!(
	//         "{}/chapter/{}.json?channel=android&version=3.0.0&timestamp={}",
	//         V3_API_CHAPTER_URL,
	//         &id,
	//         aidoku::std::current_date() as i64
	//     ),
	//     format!("{}/comic/chapter/{}.html", API_PAGELIST_OLD_URL, &id),
	// ];
	// let mut index = 0;
	// let arr: Vec<String> = loop {
	//     if index > 2 {
	//         break Vec::new();
	//     }
	//
	//     let req = helper::get(&url[index]);
	//
	//     let req = req.json();
	//     let r = match index {
	//         0 | 1 =>
	// req.as_object()?.get("page_url").clone().as_array().ok(),         2 =>
	// req             .as_object()?
	//             .get("chapter")
	//             .as_object()?
	//             .get("page_url")
	//             .clone()
	//             .as_array()
	//             .ok(),
	//         _ => None,
	//     };
	//     match r {
	//         Some(r) => {
	//             // Check if image url valid by having an extension.
	//             let mut rr: Vec<String> = Vec::new();
	//             for it in r {
	//                 let str = it.as_string()?.read();
	//
	//                 if let Some(mat) = str.rfind('.') {
	//                     match &str[mat..str.len()] {
	//                         ".jpg" | ".png" | ".gif" => rr.push(str),
	//                         _ => {}
	//                     }
	//                 }
	//             }
	//             break rr;
	//         }
	//         _ => index += 1,
	//     };
	// };
	//
	// let mut pages = Vec::new();
	//
	// for (index, r) in arr.iter().enumerate() {
	//     let mut image_url = String::from(r.deref());
	//     image_url = image_url
	//         .replace("http:", "https:")
	//         .replace("dmzj1.com", "dmzj.com");
	//
	//     let _thumb_url = {
	//         if !id.is_empty() {
	//             let initial = image_url
	//                 .strip_prefix("https://images.dmzj.com/")
	//                 .unwrap()
	//                 .get(0..1)
	//                 .unwrap();
	//
	//             format!("{}/{}/{}/{}.jpg", IMAGE_SMALL_URL, initial, id,
	// index)         } else {
	//             String::new()
	//         }
	//     };
	//
	//     pages.push(Page {
	//         index: index as i32,
	//         url: helper::encode_uri(&image_url),
	//         base64: String::new(),
	//         text: String::new(),
	//     });
	// }
	//
	// Ok(pages)
}

#[modify_image_request]
fn modify_image_request(request: Request) {}

#[handle_url]
pub fn handle_url(url: String) -> Result<DeepLink> {
	todo!()
	// let prefix = [
	//     "https://m.dmzj.com/info/",
	//     "https://www.dmzj.com/info/",
	//     "https://manhua.dmzj.com/",
	// ];
	//
	// let mut index = 0;
	// let manga_id = loop {
	//     if index > 2 {
	//         break String::new();
	//     }
	//
	//     let r = url.strip_prefix(prefix[index]);
	//     match r {
	//         Some(str) => break
	// String::from(str.strip_suffix(".html").unwrap_or_default()),         _ =>
	// index += 1,     }
	// };
	//
	// if !url.is_empty() && index <= 2 {
	//     let manga = get_manga_details(manga_id)?;
	//
	//     Ok(DeepLink {
	//         manga: Some(manga),
	//         chapter: None,
	//     })
	// } else {
	//     Err(aidoku::error::AidokuError {
	//         reason: aidoku::error::AidokuErrorKind::Unimplemented,
	//     })
	// }
}
