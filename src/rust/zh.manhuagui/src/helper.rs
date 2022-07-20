use aidoku::prelude::*;
use aidoku::{
	error::Result, std::String, std::Vec, Filter,
	FilterType,
};
use alloc::string::ToString;

use crate::consts::*;
use crate::{manga_list, utils};

pub fn fetch_manga_list(filters: Vec<Filter>, page: i32) -> Result<manga_list::Resp> {
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
			&utils::encode_uri(&query),
			page
		);
		println!("[zh-manhuagui]search fetching: {}", url);
		manga_list::parse(utils::get(&url).html(), manga_list::ParseMode::Search)
	} else {
		// Example: https://www.manhuagui.com/list/japan_maoxian_qingnian_2020_b/update_p1.html
		//                                        /$params
		// /$sort_p$page.html
		let url = format!(
			"{}/list/{}/{}_p{}.html",
			BASE_URL,
			filters_list.join("_"),
			if sort_opt.is_empty() {
				"index"
			} else {
				&sort_opt
			},
			page
		);
		println!("[zh-manhuagui]filter fetching: {}", url);
		manga_list::parse(utils::get(&url).html(), manga_list::ParseMode::Filtered)
	};
}
