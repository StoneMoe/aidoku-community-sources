/*	Created by reference to https://github.com/tachiyomiorg/tachiyomi-extensions/tree/master/src/zh/manhuagui
 *	All credit goes to their outstanding work.
 */
#![no_std]
extern crate alloc;

use alloc::string::ToString;
use aidoku::{
    error::Result,
    prelude::*,
    std::net::{HttpMethod, Request},
    std::{json, String, Vec},
    Chapter, DeepLink, Filter, FilterType, Manga, MangaContentRating, MangaPageResult, MangaStatus,
    MangaViewer, Page,
};
use aidoku::std::html::Node;
use crate::parser::manga_list;

mod parser;
mod helper;

// Server Endpoints
const BASE_URL: &str = "https://tw.manhuagui.com";
const IMAGE_SERVERS: [&str; 2] = ["https://i.hamreus.com", "https://cf.hamreus.com"];

// 排序
const FILTER_SORT: [&str; 4] = ["view", "", "update", "rate"];
// 地区
const FILTER_AREA: [&str; 7] = ["", "japan", "hongkong", "other", "europe", "china", "korea"];
// 剧情
const FILTER_GENRE: [&str; 39] = [
    "",
    "rexue",
    "maoxian",
    "mohuan",
    "shengui",
    "gaoxiao",
    "mengxi",
    "aiqing",
    "kehuan",
    "mofa",
    "gedou",
    "wuxia",
    "jizhan",
    "zhanzheng",
    "jingji",
    "tiyu",
    "xiaoyuan",
    "shenghuo",
    "lizhi",
    "lishi",
    "weiniang",
    "zhainan",
    "funv",
    "danmei",
    "baihe",
    "hougong",
    "zhiyu",
    "meishi",
    "tuili",
    "xuanyi",
    "kongbu",
    "sige",
    "zhichang",
    "zhentan",
    "shehui",
    "yinyue",
    "wudao",
    "zazhi",
    "heidao",
];
// 受众
const FILTER_READER: [&str; 6] = ["", "shaonv", "shaonian", "qingnian", "ertong", "tongyong"];
// 年份
const FILTER_PUBLISH_DATE: [&str; 16] = [
    "", "2020", "2019", "2018", "2017", "2016", "2015", "2014", "2013", "2012", "2011", "2010",
    "200x", "199x", "198x", "197x",
];
// 首字母
const FILTER_FIRST_LETTER: [&str; 28] = [
    "", "a", "b", "c", "d", "e", "f", "g", "h", "i", "j", "k", "l", "m", "n", "o", "p", "q", "r",
    "s", "t", "u", "v", "w", "x", "y", "z", "0-9",
];
// 进度
const FILTER_STATUS: [&str; 3] = ["", "lianzai", "wanjie"];

fn fetch_manga_list(filters: Vec<Filter>, page: i32) -> Result<manga_list::Resp> {
    let mut query: String = String::new();
    let mut filters_list: Vec<&str> = Vec::new();
    let mut sort_opt: String = String::new();

    // prepare filter
    for filter in filters {
        match filter.kind {
            FilterType::Title => {
                query = filter.value.as_string()?.read();
            }
            FilterType::Select => {
                let index = filter.value.as_int()? as usize;
                let filter_value = match filter.name.as_str() {
                    "地区" => FILTER_AREA[index],
                    "剧情" => FILTER_GENRE[index],
                    "受众" => FILTER_READER[index],
                    "年份" => FILTER_PUBLISH_DATE[index],
                    "首字母" => FILTER_FIRST_LETTER[index],
                    "进度" => FILTER_STATUS[index],
                    _ => continue,
                };

                if filter_value != "" {
                    filters_list.push(filter_value);
                }
            }
            FilterType::Sort => {
                let value = match filter.value.as_object() {
                    Ok(value) => value,
                    Err(_) => continue,
                };
                let sort_index = value.get("index").as_int()? as usize;
                sort_opt = FILTER_SORT[sort_index].to_string();
            }
            _ => continue,
        }
    }

    return if !query.is_empty() {
        let url = format!(
            "{}/s/{}_p{}.html",
            BASE_URL,
            &helper::encode_uri(&query),
            page
        );
        println!("zh-manhuagui - search - {}", url);
        manga_list::parse(helper::get(&url).html(), manga_list::ParseMode::Search)
    } else {
        // Example: https://www.manhuagui.com/list/japan_maoxian_qingnian_2020_b/update_p1.html
        //                                        /$params                      /$sort_p$page.html
        let url = format!(
            "{}/list/{}/{}_p{}.html",
            BASE_URL,
            filters_list.join("_"),
            if sort_opt.is_empty() { "index" } else { &sort_opt },
            page
        );
        println!("zh-manhuagui - filter - {}", url);
        manga_list::parse(helper::get(&url).html(), manga_list::ParseMode::Filtered)
    };
}

#[get_manga_list]
pub fn get_manga_list(filters: Vec<Filter>, page: i32) -> Result<MangaPageResult> {
    let list_resp = fetch_manga_list(filters, page)?;
    Ok(MangaPageResult {
        manga: list_resp.mangas,
        has_more: list_resp.has_more,
    })
}

#[get_manga_details]
fn get_manga_details(id: String) -> Result<Manga> {
    todo!()
    // let url = format!(
    //     "{}/comic/detail/{}?channel=android&version=3.0.0&timestamp={}",
    //     V4_API_URL,
    //     &id,
    //     aidoku::std::current_date() as i64
    // );
    // let data = None;
    // return Ok(Manga {
    //     id: id.clone(),
    //     cover: data.cover,
    //     title: data.title,
    //     author: data
    //         .authors
    //         .iter()
    //         .map(|s| s.tag_name.clone())
    //         .collect::<Vec<String>>()
    //         .join(", "),
    //     artist: String::new(),
    //     description: data.description,
    //     url: format!("{}/info/{}.html", BASE_URL, id),
    //     categories: data.types.iter().map(|s| s.tag_name.clone()).collect(),
    //     status: match data.status[0].tag_name.as_str() {
    //         "连载中" => MangaStatus::Ongoing,
    //         "已完结" => MangaStatus::Completed,
    //         _ => MangaStatus::Unknown,
    //     },
    //     nsfw: MangaContentRating::Safe,
    //     viewer: match data.direction {
    //         0 => MangaViewer::Rtl, // Maybe? Can't find evidence.
    //         1 => MangaViewer::Ltr,
    //         2 => MangaViewer::Scroll,
    //         _ => MangaViewer::Default,
    //     },
    // });
}

#[get_chapter_list]
fn get_chapter_list(id: String) -> Result<Vec<Chapter>> {
    todo!()
    // let url = format!(
    //     "{}/comic/detail/{}?channel=android&version=3.0.0&timestamp={}",
    //     V4_API_URL,
    //     &id,
    //     aidoku::std::current_date() as i64
    // );
    //
    // let pb = helper::decode(&helper::get(&url).string());
    //
    // let mut chapters = Vec::new();
    //
    // if pb.errno == 0 && !pb.data.as_ref().unwrap().chapters.is_empty() {
    //     let pb_data = pb.data.unwrap();
    //     let mut volume = 0;
    //     let has_multi_chapter = pb_data.chapters.len() >= 2;
    //     for chapter_list in pb_data.chapters {
    //         volume += 1;
    //         let len = chapter_list.data.len();
    //         for (index, chapter) in chapter_list.data.into_iter().enumerate() {
    //             chapters.push(Chapter {
    //                 id: format!("{}/{}", pb_data.id, chapter.chapter_id),
    //                 title: format!("{}: {}", chapter_list.title, chapter.chapter_title),
    //                 volume: if has_multi_chapter {
    //                     volume as f32
    //                 } else {
    //                     -1.0
    //                 },
    //                 chapter: (len - index) as f32,
    //                 date_updated: chapter.updatetime as f64,
    //                 scanlator: String::new(),
    //                 url: String::new(),
    //                 lang: String::from("zh"),
    //             });
    //         }
    //     }
    // } else {
    //     let url = format!("{}/dynamic/comicinfo/{}.json", API_URL, id);
    //     let req = helper::get(&url);
    //
    //     let list = req
    //         .json()
    //         .as_object()?
    //         .get("data")
    //         .as_object()?
    //         .get("list")
    //         .clone()
    //         .as_array()?;
    //
    //     let len = list.len();
    //     for (index, chapter) in list.enumerate() {
    //         let data = chapter.as_object()?;
    //
    //         chapters.push(Chapter {
    //             id: format!("{}/{}", id, data.get("id").as_string()?.read()),
    //             title: data.get("chapter_name").as_string()?.read(),
    //             volume: -1.0,
    //             chapter: (len - index) as f32,
    //             date_updated: data.get("updatetime").as_int()? as f64,
    //             scanlator: String::new(),
    //             url: String::new(),
    //             lang: String::from("zh"),
    //         });
    //     }
    // }
    // Ok(chapters)
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
    //         0 | 1 => req.as_object()?.get("page_url").clone().as_array().ok(),
    //         2 => req
    //             .as_object()?
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
    //             format!("{}/{}/{}/{}.jpg", IMAGE_SMALL_URL, initial, id, index)
    //         } else {
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

// Doesn't work
#[modify_image_request]
fn modify_image_request(request: Request) {
    request
        .header("Referer", &*format!("{}/", BASE_URL))
        .header("User-Agent",
                "Mozilla/5.0 (Linux; Android 10) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/88.0.4324.93 Mobile Safari/537.36 Aidoku/1.0");
}

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
    //         Some(str) => break String::from(str.strip_suffix(".html").unwrap_or_default()),
    //         _ => index += 1,
    //     }
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
