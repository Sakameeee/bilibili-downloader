use scraper::{Html, Selector};
use serde_json::Value;
use reqwest::header::{COOKIE, HeaderValue, USER_AGENT};
use serde::{Deserialize, Serialize};
use crate::{Agent, Cookie};

const BANGUMI_LIST_URL: &str = "https://api.bilibili.com/pgc/view/web/ep/list?ep_id=";
const BANGUMI_PLAY_URL: &str = "https://www.bilibili.com/bangumi/play/";

#[derive(Debug, Serialize, Deserialize, Default)]
pub struct Anime {
    types: String,
    score: String,
    cover: String,
    title: String,
    play: String,
    description: String,
    date: String,
    formats: Vec<String>,
    episodes: Vec<Episode>,
    count: usize,
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

pub async fn get_anime_info(ep_id: &str) -> Result<Anime, String> {
    let client = reqwest::Client::new();
    let mut headers = reqwest::header::HeaderMap::new();
    headers.insert(USER_AGENT, HeaderValue::from_static(Agent));
    headers.insert(COOKIE, HeaderValue::from_static(Cookie));

    let surf_client = surf::client();

    let bangumi_play_url = format!("{}{}", BANGUMI_PLAY_URL, ep_id);
    let bangumi_list_url = format!("{}{}", BANGUMI_LIST_URL, ep_id.replace("ep", ""));

    // 发送带 Cookie 的请求
    let mut html = surf_client
        .get(&bangumi_play_url)
        .header("Cookie", Cookie)
        .header("User-Agent", Agent)
        .await.unwrap()
        .body_string()
        .await.unwrap();

    let document = Html::parse_document(&html);

    let mut anime = Anime {
        types: String::new(),
        score: String::new(),
        cover: String::new(),
        title: String::new(),
        play: String::new(),
        description: String::new(),
        date: String::new(),
        formats: Vec::new(),
        episodes: Vec::new(),
        count: 0,
    };

    let pattern = regex::Regex::new(r#"<script id="__NEXT_DATA__" type="application/json">([^<]*)</script>"#).unwrap();
    if let Some(caps) = pattern.captures(&html) {
        if let Some(matched) = caps.get(1) {
            let json: Value = serde_json::from_str(matched.as_str()).unwrap();

            let animates = &json["props"]["pageProps"]["dehydratedState"]["queries"][0]["state"]["data"]["result"]["video_info"];
            let selector = Selector::parse("div[class=\"mediainfo_mediaDesc__jjRiB\"] span").unwrap();

            anime.types = document.select(&selector).nth(0).map(|s| s.text().collect()).unwrap_or_default();
            anime.score = document.select(&Selector::parse("div[class=\"mediainfo_score__SQ_KG\"]").unwrap())
                .next().map(|e| e.text().collect()).unwrap_or_default();
            anime.cover = document.select(&Selector::parse("img[class=\"image_ogv_weslie_common_image__Rg7Xm\"]").unwrap())
                .next().map(|e| e.value().attr("src").unwrap_or_default().to_string()).unwrap_or_default();
            anime.title = document.select(&Selector::parse("meta[property=\"og:title\"]").unwrap())
                .next().map(|e| e.value().attr("content").unwrap_or_default().to_string()).unwrap_or_default();
            anime.play = document.select(&Selector::parse("div[class=\"mediainfo_mediaDesc__jjRiB\"]").unwrap())
                .next().map(|e| e.text().collect()).unwrap_or_default();
            anime.description = document.select(&Selector::parse("meta[name=\"description\"]").unwrap())
                .next().map(|e| e.value().attr("content").unwrap_or_default().to_string()).unwrap_or_default();
            anime.date = document.select(&selector).nth(1).map(|s| s.text().collect()).unwrap_or_default();

            for format in animates["support_formats"].as_array().unwrap_or(&Vec::new()) {
                anime.formats.push(format.get("description").unwrap().to_string());
            }

            let response1 = client.get(&bangumi_list_url).headers(headers.clone()).send().await.unwrap();
            let result: Value = serde_json::from_str(&response1.text().await.unwrap()).unwrap();
            let ep_list = result["result"]["episodes"].as_array().unwrap();

            let mut episodes = Vec::new();
            for ep in ep_list {
                if ep["badge"].as_str().unwrap_or_default() == "\"预告\"" {
                    continue;
                }

                let mut episode = Episode {
                    bvid: ep["bvid"].as_str().unwrap_or_default().to_string(),
                    ep_id: ep["ep_id"].as_i64().map(|num| num.to_string()).unwrap_or_default().to_string(),
                    cover: ep["cover"].as_str().unwrap_or_default().to_string(),
                    duration: ep["duration"].as_i64().map(|num| num.to_string()).unwrap_or_default().parse().unwrap_or_default(),
                    title: ep["long_title"].as_str().unwrap_or_default().to_string(),
                    play: ep["stat_for_unity"]["vt"]["text"].as_str().unwrap_or_default().to_string(),
                    danmaku: ep["stat_for_unity"]["danmaku"]["text"].as_str().unwrap_or_default().to_string(),
                    cid: ep["cid"].as_i64().map(|num| num.to_string()).unwrap_or_default().to_string(),
                    sizes: Vec::new(),
                    video_urls: Vec::new(),
                    audio_url: String::new(),
                };

                let mut html1 = surf_client
                    .get(&&format!("{}ep{}", BANGUMI_PLAY_URL, episode.ep_id))
                    .header("Cookie", Cookie)
                    .header("User-Agent", Agent)
                    .await.unwrap()
                    .body_string()
                    .await.unwrap();
                if let Some(caps1) = pattern.captures(&html1) {
                    if let Some(matched1) = caps1.get(1) {
                        let json1: Value = serde_json::from_str(matched1.as_str()).unwrap();
                        let videos = &json1["props"]["pageProps"]["dehydratedState"]["queries"][0]["state"]["data"]["result"]["video_info"]["dash"]["video"];
                        let audios = &json1["props"]["pageProps"]["dehydratedState"]["queries"][0]["state"]["data"]["result"]["video_info"]["dash"]["audio"];

                        for video in videos.as_array().unwrap_or(&Vec::new()) {
                            episode.video_urls.push(video["base_url"].as_str().unwrap_or_default().to_string());
                            episode.sizes.push(video["size"].as_i64().unwrap_or_default().to_string());
                        }

                        for audio in audios.as_array().unwrap_or(&Vec::new()) {
                            episode.audio_url = audio["base_url"].as_str().unwrap_or_default().to_string();
                            break;
                        }
                    }
                }

                episodes.push(episode);
            }

            anime.episodes = episodes;
            anime.count = anime.episodes.len();
        }
    }

    Ok(anime)
}
