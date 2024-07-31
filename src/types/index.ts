export interface Response<T = any> {
  status: "ok" | "err";
  data: T;
  err: string;
}

export interface BiliConfig {
  savePath: string;
  agent: "Mozilla/5.0 (Windows NT 10.0; Win64; x64) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/126.0.0.0 Safari/537.36 Edg/126.0.0.0";
  cookie: string;
}

export interface Video {
  cid: string;
  title: string;
  videoUrls: string[];
  audioUrl: string;
  videoBaseRange: string;
  audioBaseRange: string;
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
  videoUrls: string[];
  audioUrl: string;
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