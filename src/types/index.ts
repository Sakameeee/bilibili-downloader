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
  cid: string;
  title: string;
  video_urls: string[];
  audio_url: string;
  video_base_range: string;
  audio_base_range: string;
  cover: string;
  duration: number;
  author: string;
  sizes: string;
  play: string;
  formats: string[];
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
  types: string[];
  cover: string;
  play: string;
  description: string;
  index: number;
  date: Date;
  score: number;
  episodes: Episode[];
  formats: string[];
}