pub static CORE_JSON: &str = "core.json";

pub static STATIC_FOLDER: &str = "./static/";

pub static INPUT_FOLDER: &str = "./static/input/";

pub static INPUT_SEARCH_FOLDER: &str = "./static/input/search/";
pub static INPUT_LIVE_FOLDER: &str = "./static/input/live/";

pub static OUTPUT_FOLDER: &str = "./static/output/";

pub static OUTPUT_THUMBNAIL_FOLDER: &str = "./static/output/thumbnail/";

pub static LOGS_FOLDER: &str = "./static/logs/";

pub static core_data:&str = r#"{
  "check": {
    "now": null,
    "task": {
    }
  },
  "ob": {
    "list": []
  },
  "search": {
    "source": [
      {
        "urls": [
          "https://github.com/YueChan/Live",
          "https://github.com/YanG-1989/m3u",
          "https://github.com/fanmingming/live",
          "https://github.com/qwerttvv/Beijing-IPTV",
          "https://github.com/joevess/IPTV",
          "https://github.com/cymz6/AutoIPTV-Hotel",
          "https://github.com/skddyj/iptv",
          "https://github.com/suxuang/myIPTV"
        ],
        "include_files": [],
        "parse_type": "github-home-page"
      },
      {
        "urls": [
          "https://live.zbds.top/tv/iptv6.m3u",
          "https://live.zbds.top/tv/iptv4.m3u",
          "https://raw.githubusercontent.com/jackell777/jackell777.github.io/fa8f1249b67cff645628b6e08fa6f802d430afbb/list.txt",
          "https://raw.githubusercontent.com/sake0116/0305/983fcb9a7ea4cea08a4c177495d34d9ce76db757/2185"
        ],
        "include_files": [],
        "parse_type": "raw-source"
      },
      {
        "urls": [
          "https://github.com/iptv-org/iptv/tree/master/streams"
        ],
        "include_files": [
          "cn.m3u",
          "tw.m3u",
          "hk.m3u"
        ],
        "parse_type": "github-sub-page"
      }
    ],
    "extensions": [
      ".txt",
      ".m3u"
    ],
    "search_list": []
  }
}"#;