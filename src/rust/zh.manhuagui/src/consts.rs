// Server Endpoints
pub const BASE_URL: &str = "https://www.manhuagui.com";
const IMAGE_SERVERS: [&str; 2] = ["https://i.hamreus.com", "https://cf.hamreus.com"];

// 排序
pub const FILTER_SORT: [&str; 4] = ["view", "", "update", "rate"];
// 地区
pub const FILTER_AREA: [&str; 7] = ["", "japan", "hongkong", "other", "europe", "china", "korea"];
// 剧情
pub const FILTER_GENRE: [&str; 39] = [
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
pub const FILTER_READER: [&str; 6] = ["", "shaonv", "shaonian", "qingnian", "ertong", "tongyong"];
// 年份
pub const FILTER_PUBLISH_DATE: [&str; 16] = [
	"", "2020", "2019", "2018", "2017", "2016", "2015", "2014", "2013", "2012", "2011", "2010",
	"200x", "199x", "198x", "197x",
];
// 首字母
pub const FILTER_FIRST_LETTER: [&str; 28] = [
	"", "a", "b", "c", "d", "e", "f", "g", "h", "i", "j", "k", "l", "m", "n", "o", "p", "q", "r",
	"s", "t", "u", "v", "w", "x", "y", "z", "0-9",
];
// 进度
pub const FILTER_STATUS: [&str; 3] = ["", "lianzai", "wanjie"];
