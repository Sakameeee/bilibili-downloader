use scraper::{Html, Selector};
use serde::{Deserialize, Serialize};
use serde_json::Value;
use regex::Regex;
use reqwest::header::{COOKIE, HeaderValue, USER_AGENT};
use crate::{Agent, Cookie};

const VIDEO_INFO_URL: &str = "https://api.bilibili.com/x/web-interface/wbi/view?bvid={}";
const VIDEO_PLAY_URL: &str = "https://www.bilibili.com/video/bvid/?p={} ";

#[derive(Debug, Serialize, Deserialize, Default)]
pub struct Video {
    title: String,
    description: String,
    cid: String,
    cover: String,
    bvid: String,
    author: String,
    author_avatar: String,
    count: usize,
    date: String,
    play: String,
    danmaku: String,
    episodes: Vec<Episode>,
    formats: Vec<String>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Episode {
    bvid: String,
    ep_id: String,
    cid: String,
    title: String,
    video_urls: Vec<String>,
    audio_url: String,
    duration: i32,
    cover: String,
    play: String,
    danmaku: String,
    sizes: Vec<String>,
}

pub async fn get_video_info(bvid: &str) -> Result<Video, String> {
    let client = reqwest::Client::new();
    let mut global_headers = reqwest::header::HeaderMap::new();
    global_headers.insert(USER_AGENT, HeaderValue::from_static(Agent));
    global_headers.insert(COOKIE, HeaderValue::from_static(Cookie));

    // 创建一个 Surf 客户端
    let surf_client = surf::client();
    let video_play_url = VIDEO_PLAY_URL.replace("bvid", bvid);
    // 发送带 Cookie 的请求
    let mut response1 = surf_client
        .get(&video_play_url)
        .header("Cookie", Cookie)
        .header("User-Agent", Agent)
        .await.unwrap();

    // 获取响应体
    let html = response1.body_string().await.unwrap();
    let document = Html::parse_document(&html);
    // 获取视频名称和描述
    let meta_description_selector = Selector::parse(r#"meta[name="description"]"#).unwrap();
    let title_selector = Selector::parse("title").unwrap();
    let title_element = document.select(&title_selector).next().ok_or("Title not found").unwrap();
    let title = title_element.text().collect::<String>();
    let description = document
        .select(&meta_description_selector)
        .next()
        .map(|node| node.value().attr("content").unwrap_or(""))
        .unwrap().to_string();

    let mut video = Video {
        title,
        description,
        cid: String::new(),
        cover: String::new(),
        bvid: bvid.to_string(),
        author: String::new(),
        author_avatar: String::new(),
        count: 0,
        date: String::new(),
        play: String::new(),
        danmaku: String::new(),
        episodes: vec![],
        formats: vec![],
    };

    // 请求视频信息 API 接口，拼装信息
    let video_info_url = format!("https://api.bilibili.com/x/web-interface/view?bvid={}", bvid);
    let ret = client.get(&video_info_url).headers(global_headers.clone()).send().await.unwrap();
    let video_info: Value = client.get(&video_info_url).headers(global_headers.clone()).send().await.unwrap().json().await.unwrap();

    if let Some(data) = video_info.get("data") {
        video.cid = data.get("cid").and_then(Value::as_i64)
            .map(|num| num.to_string())
            .unwrap_or_default();
        video.cover = data.get("pic").and_then(Value::as_str).unwrap_or_default().to_string();
        video.author = data.get("owner").and_then(|o| o.get("name")).and_then(Value::as_str).unwrap_or_default().to_string();
        video.author_avatar = data.get("owner").and_then(|o| o.get("face")).and_then(Value::as_str).unwrap_or_default().to_string();
        video.count = data.get("pages").and_then(Value::as_array).map_or(0, |pages| pages.len());
        video.date = data.get("pubdate").and_then(Value::as_i64).map_or(String::new(), |timestamp| {
            let naive = chrono::NaiveDateTime::from_timestamp(timestamp, 0);
            naive.format("%Y-%m-%d").to_string()
        });
        video.play = data.get("stat").and_then(|s| s.get("view")).and_then(Value::as_i64)
            .map(|num| num.to_string())
            .unwrap_or_default();
        video.danmaku = data.get("stat").and_then(|s| s.get("danmaku")).and_then(Value::as_i64)
            .map(|num| num.to_string())
            .unwrap_or_default();

        let mut episodes = Vec::new();
        if let Some(pages) = data.get("pages").and_then(Value::as_array) {
            for page in pages {
                let episode = Episode {
                    bvid: "".to_string(),
                    ep_id: "".to_string(),
                    cid: page.get("cid").and_then(Value::as_i64)
                        .map(|num| num.to_string())
                        .unwrap_or_default(),
                    title: page.get("part").and_then(Value::as_str).unwrap_or_default().to_string(),
                    duration: page.get("duration").and_then(Value::as_i64).unwrap_or_default() as i32,
                    cover: "".to_string(),
                    play: "".to_string(),
                    danmaku: "".to_string(),
                    video_urls: vec![],
                    audio_url: String::new(),
                    sizes: vec![],
                };

                episodes.push(episode);
            }
        }
        video.episodes = episodes;
    }

    // 获取视频格式和链接信息
    for j in 0..video.count {
        let episode = &mut video.episodes[j];
        let mut html = surf_client
            .get(&video_play_url)
            .header("Cookie", Cookie)
            .header("User-Agent", Agent)
            .await.unwrap()
            .body_string()
            .await.unwrap();
        let re = Regex::new(r#"<script>window\.__playinfo__=([^<]*)</script>"#).unwrap();
        if let Some(captures) = re.captures(&html) {
            // 提取匹配的内容
            if let Some(matched) = captures.get(1) {
                let play_info: Value = serde_json::from_str(matched.as_str()).unwrap();
                let data = &play_info["data"];

                // 获取视频链接
                let dash = data.get("dash");
                let videos = dash.and_then(|d| d.get("video")).and_then(Value::as_array).unwrap();
                let mut video_urls = vec![];
                for video in videos {
                    if let Some(base_url) = video.get("baseUrl").and_then(Value::as_str) {
                        video_urls.push(base_url.to_string());
                    }
                }
                episode.video_urls = video_urls;

                // 获取音频链接
                if let Some(audio) = data.get("dash").and_then(|dash| dash.get("audio")).and_then(Value::as_array).and_then(|audios| audios.get(0)) {
                    episode.audio_url = audio.get("baseUrl").and_then(Value::as_str).unwrap_or_default().to_string();
                }

                // 获取格式信息
                if j == 0 {
                    if let Some(formats) = data.get("accept_description").and_then(Value::as_array) {
                        video.formats = formats.iter().filter_map(Value::as_str).map(|s| s.to_string()).collect();
                    }
                }
            }
        } else {
            println!("No match found");
        }
    }

    Ok(video)
}
