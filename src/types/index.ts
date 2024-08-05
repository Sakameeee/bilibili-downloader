export interface Response<T = any> {
  status: "ok" | "err";
  data: T;
  err: string;
}

export interface BiliConfig {
  save_path: string;
  agent: string;
  cookie: string;
}

export interface Video {
  bvid: string;
  cid: string;
  title: string;
  description: string;
  cover: string;
  duration: number;
  author: string;
  author_avatar: string;
  count: number;
  danmaku: string;
  play: string;
  formats: string[];
  episodes: Episode[];
}

export interface Episode {
  epId: string;
  cid: string;
  title: string;
  video_urls: string[];
  audio_url: string;
  duration: number;
  cover: string;
  play: string;
  danmaku: string;
  sizes: string[];
}

export interface Anime {
  title: string;
  types: string;
  cover: string;
  play: string;
  description: string;
  index: number;
  date: string;
  score: string;
  episodes: Episode[];
  formats: string[];
}

export interface Download {
  id: number;
  url: string;
  file_name: string;
  file_path: string;
  referer: string;
  total_size: number;
  downloaded_size: number;
  status: string;
  added_date: string;
  last_updated_date: string;
}