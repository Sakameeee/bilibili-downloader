<script setup lang="ts">

import {ArrowLeft} from "@element-plus/icons-vue";

import {useRoute} from "vue-router";
import {Anime} from "../types";
import {onMounted, ref} from "vue";

const loading = ref(false);
const value = ref();
const radio = ref(3);
const checkboxGroup1 = ref([0])
const route = useRoute();
const animeInfo = ref<Anime>(<Anime>{
  title: "百妖谱·司府篇",
  types: "小说改 / 古风 / 玄幻 ·",
  cover: "//i0.hdslb.com/bfs/bangumi/image/2c0dcb30da71753588407f74961daa2222418528.png",
  play: "7464.3万播放 · 11.4万弹幕 · 753.4万系列追番 小说改 / 古风 / 玄幻 · 2024 · 已完结, 全12话 · BV1LZ42117ET 声优：桃夭：赵双 柳公子：曹云图 磨牙：常蓉珊 司静渊：皇贞季 司狂澜：藤新 苗管家：刘琮 陆夫人：蔡娜 陆澄：涂小鸦 百知：朔小兔 司父：李铫 虫虫：朔小兔 许承怀：归什 莲歆：戈昕宇 孰湖哥哥：乐小彤 孰湖弟弟：来晓风 邱晚来：曹一茜 枫生：刘蕊 沈枫：刘蕊 沈明善：言灵 明善父亲：任景行 趸鱼：乐小彤 黑袍少女（桃夭）：赵双 镇水：郭政建 明善母亲：蔺婵婵",
  description: "在第三季中，桃夭追着百妖谱的气息一路来到京师，为了养活一僧一蛇一狐，她在京师的大户人家“司府”担任喂马杂役。尽管自称只救妖不救人，但当面对暗刀、百知、风果、孰湖、枫生、趸鱼、镇水等妖怪和事中人时，桃夭心怀善念，解铃还愿。这一季充满了情感与冒险，展现出一系列感人至深的治愈故事~在第三季中，桃夭追着百妖谱的气息一路来到京师，为了养活一僧一蛇一狐，她在京师的大户人家“司府”担任喂马杂役。尽管自称只救妖不救人，但当面对暗刀、百知、风果、孰湖、枫生、趸鱼、镇水等妖怪和事中人时，桃夭心怀善念，解铃还愿。这一季充满了情感与冒险，展现出一系列感人至深的治愈故事~在第三季中，桃夭追着百妖谱的气息一路来到京师，为了养活一僧一蛇一狐，她在京师的大户人家“司府”担任喂马杂役。尽管自称只救妖不救人，但当面对暗刀、百知、风果、孰湖、枫生、趸鱼、镇水等妖怪和事中人时，桃夭心怀善念，解铃还愿。这一季充满了情感与冒险，展现出一系列感人至深的治愈故事~",
  index: 12,
  date: "2024",
  score: "9.6分",
  formats: ["高清 1080P+", "高清 1080P", "高清 720P"],
  episodes: [{
    cid: "301428697",
    title: "故人归乡",
    video_urls: ["https://xy111x225x218x142xy.mcdn.bilivideo.cn:4483/upgcxcode/97/86/301428697/301428697-1-30032.m4s?e=ig8euxZM2rNcNbdlhoNvNC8BqJIzNbfqXBvEqxTEto8BTrNvN0GvT90W5JZMkX_YN0MvXg8gNEV4NC8xNEV4N03eN0B5tZlqNxTEto8BTrNvNeZVuJ10Kj_g2UB02J0mN0B5tZlqNCNEto8BTrNvNC7MTX502C8f2jmMQJ6mqF2fka1mqx6gqj0eN0B599M=&uipk=5&nbs=1&deadline=1722621726&gen=playurlv2&os=mcdn&oi=1035876564&trid=0000db23ebfe93334781b190a097b9646038u&mid=0&platform=pc&og=hw&upsig=d37e6b34f01330c2e3e0126f28868c9b&uparams=e,uipk,nbs,deadline,gen,os,oi,trid,mid,platform,og&mcdnid=50004915&bvc=vod&nettype=0&orderid=0,3&buvid=4CB663D6-6208-6C0E-4392-4C7DCEAB3E5E94322infoc&build=0&f=u_0_0&agrr=0&bw=49880&logo=A0020000", "https://xy111x225x218x142xy.mcdn.bilivideo.cn:4483/upgcxcode/97/86/301428697/301428697-1-30011.m4s?e=ig8euxZM2rNcNbdlhoNvNC8BqJIzNbfqXBvEqxTEto8BTrNvN0GvT90W5JZMkX_YN0MvXg8gNEV4NC8xNEV4N03eN0B5tZlqNxTEto8BTrNvNeZVuJ10Kj_g2UB02J0mN0B5tZlqNCNEto8BTrNvNC7MTX502C8f2jmMQJ6mqF2fka1mqx6gqj0eN0B599M=&uipk=5&nbs=1&deadline=1722621726&gen=playurlv2&os=mcdn&oi=1035876564&trid=0000db23ebfe93334781b190a097b9646038u&mid=0&platform=pc&og=hw&upsig=43409cf50f79c1a58fa6f504b8396ea4&uparams=e,uipk,nbs,deadline,gen,os,oi,trid,mid,platform,og&mcdnid=50004915&bvc=vod&nettype=0&orderid=0,3&buvid=4CB663D6-6208-6C0E-4392-4C7DCEAB3E5E94322infoc&build=0&f=u_0_0&agrr=0&bw=10573&logo=A0020000"],
    audio_url: "https://xy111x225x218x142xy.mcdn.bilivideo.cn:4483/upgcxcode/97/86/301428697/301428697-1-30280.m4s?e=ig8euxZM2rNcNbdlhoNvNC8BqJIzNbfqXBvEqxTEto8BTrNvN0GvT90W5JZMkX_YN0MvXg8gNEV4NC8xNEV4N03eN0B5tZlqNxTEto8BTrNvNeZVuJ10Kj_g2UB02J0mN0B5tZlqNCNEto8BTrNvNC7MTX502C8f2jmMQJ6mqF2fka1mqx6gqj0eN0B599M=&uipk=5&nbs=1&deadline=1722621726&gen=playurlv2&os=mcdn&oi=1035876564&trid=0000db23ebfe93334781b190a097b9646038u&mid=0&platform=pc&og=hw&upsig=0fe2d0a79cadffcf846f4ef0c8eface2&uparams=e,uipk,nbs,deadline,gen,os,oi,trid,mid,platform,og&mcdnid=50004915&bvc=vod&nettype=0&orderid=0,3&buvid=4CB663D6-6208-6C0E-4392-4C7DCEAB3E5E94322infoc&build=0&f=u_0_0&agrr=0&bw=16193&logo=A0020000",
    duration: 1388999,
    cover: "http://i0.hdslb.com/bfs/archive/3ae2646d694b61f8eb1f651fa873b0c722134e1e.jpg",
  }, {
    cid: "301428697",
    title: "暗刀障目",
    video_urls: ["https://xy111x225x218x142xy.mcdn.bilivideo.cn:4483/upgcxcode/97/86/301428697/301428697-1-30032.m4s?e=ig8euxZM2rNcNbdlhoNvNC8BqJIzNbfqXBvEqxTEto8BTrNvN0GvT90W5JZMkX_YN0MvXg8gNEV4NC8xNEV4N03eN0B5tZlqNxTEto8BTrNvNeZVuJ10Kj_g2UB02J0mN0B5tZlqNCNEto8BTrNvNC7MTX502C8f2jmMQJ6mqF2fka1mqx6gqj0eN0B599M=&uipk=5&nbs=1&deadline=1722621726&gen=playurlv2&os=mcdn&oi=1035876564&trid=0000db23ebfe93334781b190a097b9646038u&mid=0&platform=pc&og=hw&upsig=d37e6b34f01330c2e3e0126f28868c9b&uparams=e,uipk,nbs,deadline,gen,os,oi,trid,mid,platform,og&mcdnid=50004915&bvc=vod&nettype=0&orderid=0,3&buvid=4CB663D6-6208-6C0E-4392-4C7DCEAB3E5E94322infoc&build=0&f=u_0_0&agrr=0&bw=49880&logo=A0020000", "https://xy111x225x218x142xy.mcdn.bilivideo.cn:4483/upgcxcode/97/86/301428697/301428697-1-30011.m4s?e=ig8euxZM2rNcNbdlhoNvNC8BqJIzNbfqXBvEqxTEto8BTrNvN0GvT90W5JZMkX_YN0MvXg8gNEV4NC8xNEV4N03eN0B5tZlqNxTEto8BTrNvNeZVuJ10Kj_g2UB02J0mN0B5tZlqNCNEto8BTrNvNC7MTX502C8f2jmMQJ6mqF2fka1mqx6gqj0eN0B599M=&uipk=5&nbs=1&deadline=1722621726&gen=playurlv2&os=mcdn&oi=1035876564&trid=0000db23ebfe93334781b190a097b9646038u&mid=0&platform=pc&og=hw&upsig=43409cf50f79c1a58fa6f504b8396ea4&uparams=e,uipk,nbs,deadline,gen,os,oi,trid,mid,platform,og&mcdnid=50004915&bvc=vod&nettype=0&orderid=0,3&buvid=4CB663D6-6208-6C0E-4392-4C7DCEAB3E5E94322infoc&build=0&f=u_0_0&agrr=0&bw=10573&logo=A0020000"],
    audio_url: "https://xy111x225x218x142xy.mcdn.bilivideo.cn:4483/upgcxcode/97/86/301428697/301428697-1-30280.m4s?e=ig8euxZM2rNcNbdlhoNvNC8BqJIzNbfqXBvEqxTEto8BTrNvN0GvT90W5JZMkX_YN0MvXg8gNEV4NC8xNEV4N03eN0B5tZlqNxTEto8BTrNvNeZVuJ10Kj_g2UB02J0mN0B5tZlqNCNEto8BTrNvNC7MTX502C8f2jmMQJ6mqF2fka1mqx6gqj0eN0B599M=&uipk=5&nbs=1&deadline=1722621726&gen=playurlv2&os=mcdn&oi=1035876564&trid=0000db23ebfe93334781b190a097b9646038u&mid=0&platform=pc&og=hw&upsig=0fe2d0a79cadffcf846f4ef0c8eface2&uparams=e,uipk,nbs,deadline,gen,os,oi,trid,mid,platform,og&mcdnid=50004915&bvc=vod&nettype=0&orderid=0,3&buvid=4CB663D6-6208-6C0E-4392-4C7DCEAB3E5E94322infoc&build=0&f=u_0_0&agrr=0&bw=16193&logo=A0020000",
    duration: 36000,
    cover: "http://i0.hdslb.com/bfs/archive/ee03abe54f5e32999cadc12183ea4dc4a48fd0b3.jpg"
  }, {
    cid: "301428697",
    title: "心魔丛生",
    video_urls: ["https://xy111x225x218x142xy.mcdn.bilivideo.cn:4483/upgcxcode/97/86/301428697/301428697-1-30032.m4s?e=ig8euxZM2rNcNbdlhoNvNC8BqJIzNbfqXBvEqxTEto8BTrNvN0GvT90W5JZMkX_YN0MvXg8gNEV4NC8xNEV4N03eN0B5tZlqNxTEto8BTrNvNeZVuJ10Kj_g2UB02J0mN0B5tZlqNCNEto8BTrNvNC7MTX502C8f2jmMQJ6mqF2fka1mqx6gqj0eN0B599M=&uipk=5&nbs=1&deadline=1722621726&gen=playurlv2&os=mcdn&oi=1035876564&trid=0000db23ebfe93334781b190a097b9646038u&mid=0&platform=pc&og=hw&upsig=d37e6b34f01330c2e3e0126f28868c9b&uparams=e,uipk,nbs,deadline,gen,os,oi,trid,mid,platform,og&mcdnid=50004915&bvc=vod&nettype=0&orderid=0,3&buvid=4CB663D6-6208-6C0E-4392-4C7DCEAB3E5E94322infoc&build=0&f=u_0_0&agrr=0&bw=49880&logo=A0020000", "https://xy111x225x218x142xy.mcdn.bilivideo.cn:4483/upgcxcode/97/86/301428697/301428697-1-30011.m4s?e=ig8euxZM2rNcNbdlhoNvNC8BqJIzNbfqXBvEqxTEto8BTrNvN0GvT90W5JZMkX_YN0MvXg8gNEV4NC8xNEV4N03eN0B5tZlqNxTEto8BTrNvNeZVuJ10Kj_g2UB02J0mN0B5tZlqNCNEto8BTrNvNC7MTX502C8f2jmMQJ6mqF2fka1mqx6gqj0eN0B599M=&uipk=5&nbs=1&deadline=1722621726&gen=playurlv2&os=mcdn&oi=1035876564&trid=0000db23ebfe93334781b190a097b9646038u&mid=0&platform=pc&og=hw&upsig=43409cf50f79c1a58fa6f504b8396ea4&uparams=e,uipk,nbs,deadline,gen,os,oi,trid,mid,platform,og&mcdnid=50004915&bvc=vod&nettype=0&orderid=0,3&buvid=4CB663D6-6208-6C0E-4392-4C7DCEAB3E5E94322infoc&build=0&f=u_0_0&agrr=0&bw=10573&logo=A0020000"],
    audio_url: "https://xy111x225x218x142xy.mcdn.bilivideo.cn:4483/upgcxcode/97/86/301428697/301428697-1-30280.m4s?e=ig8euxZM2rNcNbdlhoNvNC8BqJIzNbfqXBvEqxTEto8BTrNvN0GvT90W5JZMkX_YN0MvXg8gNEV4NC8xNEV4N03eN0B5tZlqNxTEto8BTrNvNeZVuJ10Kj_g2UB02J0mN0B5tZlqNCNEto8BTrNvNC7MTX502C8f2jmMQJ6mqF2fka1mqx6gqj0eN0B599M=&uipk=5&nbs=1&deadline=1722621726&gen=playurlv2&os=mcdn&oi=1035876564&trid=0000db23ebfe93334781b190a097b9646038u&mid=0&platform=pc&og=hw&upsig=0fe2d0a79cadffcf846f4ef0c8eface2&uparams=e,uipk,nbs,deadline,gen,os,oi,trid,mid,platform,og&mcdnid=50004915&bvc=vod&nettype=0&orderid=0,3&buvid=4CB663D6-6208-6C0E-4392-4C7DCEAB3E5E94322infoc&build=0&f=u_0_0&agrr=0&bw=16193&logo=A0020000",
    duration: 36000,
    cover: "http://i0.hdslb.com/bfs/archive/769fb4a8d3be2252f4b5db497090768cd86399ac.jpg",
  }, {
    cid: "301428697",
    title: "百知现形",
    video_urls: ["https://xy111x225x218x142xy.mcdn.bilivideo.cn:4483/upgcxcode/97/86/301428697/301428697-1-30032.m4s?e=ig8euxZM2rNcNbdlhoNvNC8BqJIzNbfqXBvEqxTEto8BTrNvN0GvT90W5JZMkX_YN0MvXg8gNEV4NC8xNEV4N03eN0B5tZlqNxTEto8BTrNvNeZVuJ10Kj_g2UB02J0mN0B5tZlqNCNEto8BTrNvNC7MTX502C8f2jmMQJ6mqF2fka1mqx6gqj0eN0B599M=&uipk=5&nbs=1&deadline=1722621726&gen=playurlv2&os=mcdn&oi=1035876564&trid=0000db23ebfe93334781b190a097b9646038u&mid=0&platform=pc&og=hw&upsig=d37e6b34f01330c2e3e0126f28868c9b&uparams=e,uipk,nbs,deadline,gen,os,oi,trid,mid,platform,og&mcdnid=50004915&bvc=vod&nettype=0&orderid=0,3&buvid=4CB663D6-6208-6C0E-4392-4C7DCEAB3E5E94322infoc&build=0&f=u_0_0&agrr=0&bw=49880&logo=A0020000", "https://xy111x225x218x142xy.mcdn.bilivideo.cn:4483/upgcxcode/97/86/301428697/301428697-1-30011.m4s?e=ig8euxZM2rNcNbdlhoNvNC8BqJIzNbfqXBvEqxTEto8BTrNvN0GvT90W5JZMkX_YN0MvXg8gNEV4NC8xNEV4N03eN0B5tZlqNxTEto8BTrNvNeZVuJ10Kj_g2UB02J0mN0B5tZlqNCNEto8BTrNvNC7MTX502C8f2jmMQJ6mqF2fka1mqx6gqj0eN0B599M=&uipk=5&nbs=1&deadline=1722621726&gen=playurlv2&os=mcdn&oi=1035876564&trid=0000db23ebfe93334781b190a097b9646038u&mid=0&platform=pc&og=hw&upsig=43409cf50f79c1a58fa6f504b8396ea4&uparams=e,uipk,nbs,deadline,gen,os,oi,trid,mid,platform,og&mcdnid=50004915&bvc=vod&nettype=0&orderid=0,3&buvid=4CB663D6-6208-6C0E-4392-4C7DCEAB3E5E94322infoc&build=0&f=u_0_0&agrr=0&bw=10573&logo=A0020000"],
    audio_url: "https://xy111x225x218x142xy.mcdn.bilivideo.cn:4483/upgcxcode/97/86/301428697/301428697-1-30280.m4s?e=ig8euxZM2rNcNbdlhoNvNC8BqJIzNbfqXBvEqxTEto8BTrNvN0GvT90W5JZMkX_YN0MvXg8gNEV4NC8xNEV4N03eN0B5tZlqNxTEto8BTrNvNeZVuJ10Kj_g2UB02J0mN0B5tZlqNCNEto8BTrNvNC7MTX502C8f2jmMQJ6mqF2fka1mqx6gqj0eN0B599M=&uipk=5&nbs=1&deadline=1722621726&gen=playurlv2&os=mcdn&oi=1035876564&trid=0000db23ebfe93334781b190a097b9646038u&mid=0&platform=pc&og=hw&upsig=0fe2d0a79cadffcf846f4ef0c8eface2&uparams=e,uipk,nbs,deadline,gen,os,oi,trid,mid,platform,og&mcdnid=50004915&bvc=vod&nettype=0&orderid=0,3&buvid=4CB663D6-6208-6C0E-4392-4C7DCEAB3E5E94322infoc&build=0&f=u_0_0&agrr=0&bw=16193&logo=A0020000",
    duration: 36000,
    cover: "http://i0.hdslb.com/bfs/archive/8be70cfe1699dd85dffd2696d60a481cc51b4b5d.png",
  }, {
    cid: "301428697",
    title: "风果篡忆",
    video_urls: ["https://xy111x225x218x142xy.mcdn.bilivideo.cn:4483/upgcxcode/97/86/301428697/301428697-1-30032.m4s?e=ig8euxZM2rNcNbdlhoNvNC8BqJIzNbfqXBvEqxTEto8BTrNvN0GvT90W5JZMkX_YN0MvXg8gNEV4NC8xNEV4N03eN0B5tZlqNxTEto8BTrNvNeZVuJ10Kj_g2UB02J0mN0B5tZlqNCNEto8BTrNvNC7MTX502C8f2jmMQJ6mqF2fka1mqx6gqj0eN0B599M=&uipk=5&nbs=1&deadline=1722621726&gen=playurlv2&os=mcdn&oi=1035876564&trid=0000db23ebfe93334781b190a097b9646038u&mid=0&platform=pc&og=hw&upsig=d37e6b34f01330c2e3e0126f28868c9b&uparams=e,uipk,nbs,deadline,gen,os,oi,trid,mid,platform,og&mcdnid=50004915&bvc=vod&nettype=0&orderid=0,3&buvid=4CB663D6-6208-6C0E-4392-4C7DCEAB3E5E94322infoc&build=0&f=u_0_0&agrr=0&bw=49880&logo=A0020000", "https://xy111x225x218x142xy.mcdn.bilivideo.cn:4483/upgcxcode/97/86/301428697/301428697-1-30011.m4s?e=ig8euxZM2rNcNbdlhoNvNC8BqJIzNbfqXBvEqxTEto8BTrNvN0GvT90W5JZMkX_YN0MvXg8gNEV4NC8xNEV4N03eN0B5tZlqNxTEto8BTrNvNeZVuJ10Kj_g2UB02J0mN0B5tZlqNCNEto8BTrNvNC7MTX502C8f2jmMQJ6mqF2fka1mqx6gqj0eN0B599M=&uipk=5&nbs=1&deadline=1722621726&gen=playurlv2&os=mcdn&oi=1035876564&trid=0000db23ebfe93334781b190a097b9646038u&mid=0&platform=pc&og=hw&upsig=43409cf50f79c1a58fa6f504b8396ea4&uparams=e,uipk,nbs,deadline,gen,os,oi,trid,mid,platform,og&mcdnid=50004915&bvc=vod&nettype=0&orderid=0,3&buvid=4CB663D6-6208-6C0E-4392-4C7DCEAB3E5E94322infoc&build=0&f=u_0_0&agrr=0&bw=10573&logo=A0020000"],
    audio_url: "https://xy111x225x218x142xy.mcdn.bilivideo.cn:4483/upgcxcode/97/86/301428697/301428697-1-30280.m4s?e=ig8euxZM2rNcNbdlhoNvNC8BqJIzNbfqXBvEqxTEto8BTrNvN0GvT90W5JZMkX_YN0MvXg8gNEV4NC8xNEV4N03eN0B5tZlqNxTEto8BTrNvNeZVuJ10Kj_g2UB02J0mN0B5tZlqNCNEto8BTrNvNC7MTX502C8f2jmMQJ6mqF2fka1mqx6gqj0eN0B599M=&uipk=5&nbs=1&deadline=1722621726&gen=playurlv2&os=mcdn&oi=1035876564&trid=0000db23ebfe93334781b190a097b9646038u&mid=0&platform=pc&og=hw&upsig=0fe2d0a79cadffcf846f4ef0c8eface2&uparams=e,uipk,nbs,deadline,gen,os,oi,trid,mid,platform,og&mcdnid=50004915&bvc=vod&nettype=0&orderid=0,3&buvid=4CB663D6-6208-6C0E-4392-4C7DCEAB3E5E94322infoc&build=0&f=u_0_0&agrr=0&bw=16193&logo=A0020000",
    duration: 36000,
    cover: "http://i0.hdslb.com/bfs/archive/8eb10a8b98800d8ae25e21ed4d4f6dc17d0ba2d5.png"
  }, {
    cid: "301428697",
    title: "自欺梦醒",
    video_urls: ["https://xy111x225x218x142xy.mcdn.bilivideo.cn:4483/upgcxcode/97/86/301428697/301428697-1-30032.m4s?e=ig8euxZM2rNcNbdlhoNvNC8BqJIzNbfqXBvEqxTEto8BTrNvN0GvT90W5JZMkX_YN0MvXg8gNEV4NC8xNEV4N03eN0B5tZlqNxTEto8BTrNvNeZVuJ10Kj_g2UB02J0mN0B5tZlqNCNEto8BTrNvNC7MTX502C8f2jmMQJ6mqF2fka1mqx6gqj0eN0B599M=&uipk=5&nbs=1&deadline=1722621726&gen=playurlv2&os=mcdn&oi=1035876564&trid=0000db23ebfe93334781b190a097b9646038u&mid=0&platform=pc&og=hw&upsig=d37e6b34f01330c2e3e0126f28868c9b&uparams=e,uipk,nbs,deadline,gen,os,oi,trid,mid,platform,og&mcdnid=50004915&bvc=vod&nettype=0&orderid=0,3&buvid=4CB663D6-6208-6C0E-4392-4C7DCEAB3E5E94322infoc&build=0&f=u_0_0&agrr=0&bw=49880&logo=A0020000", "https://xy111x225x218x142xy.mcdn.bilivideo.cn:4483/upgcxcode/97/86/301428697/301428697-1-30011.m4s?e=ig8euxZM2rNcNbdlhoNvNC8BqJIzNbfqXBvEqxTEto8BTrNvN0GvT90W5JZMkX_YN0MvXg8gNEV4NC8xNEV4N03eN0B5tZlqNxTEto8BTrNvNeZVuJ10Kj_g2UB02J0mN0B5tZlqNCNEto8BTrNvNC7MTX502C8f2jmMQJ6mqF2fka1mqx6gqj0eN0B599M=&uipk=5&nbs=1&deadline=1722621726&gen=playurlv2&os=mcdn&oi=1035876564&trid=0000db23ebfe93334781b190a097b9646038u&mid=0&platform=pc&og=hw&upsig=43409cf50f79c1a58fa6f504b8396ea4&uparams=e,uipk,nbs,deadline,gen,os,oi,trid,mid,platform,og&mcdnid=50004915&bvc=vod&nettype=0&orderid=0,3&buvid=4CB663D6-6208-6C0E-4392-4C7DCEAB3E5E94322infoc&build=0&f=u_0_0&agrr=0&bw=10573&logo=A0020000"],
    audio_url: "https://xy111x225x218x142xy.mcdn.bilivideo.cn:4483/upgcxcode/97/86/301428697/301428697-1-30280.m4s?e=ig8euxZM2rNcNbdlhoNvNC8BqJIzNbfqXBvEqxTEto8BTrNvN0GvT90W5JZMkX_YN0MvXg8gNEV4NC8xNEV4N03eN0B5tZlqNxTEto8BTrNvNeZVuJ10Kj_g2UB02J0mN0B5tZlqNCNEto8BTrNvNC7MTX502C8f2jmMQJ6mqF2fka1mqx6gqj0eN0B599M=&uipk=5&nbs=1&deadline=1722621726&gen=playurlv2&os=mcdn&oi=1035876564&trid=0000db23ebfe93334781b190a097b9646038u&mid=0&platform=pc&og=hw&upsig=0fe2d0a79cadffcf846f4ef0c8eface2&uparams=e,uipk,nbs,deadline,gen,os,oi,trid,mid,platform,og&mcdnid=50004915&bvc=vod&nettype=0&orderid=0,3&buvid=4CB663D6-6208-6C0E-4392-4C7DCEAB3E5E94322infoc&build=0&f=u_0_0&agrr=0&bw=16193&logo=A0020000",
    duration: 36000,
    cover: "https://i0.hdslb.com/bfs/bangumi/image/0641f137019b77df1feda59043985c77549322f0.png"
  }, {
    cid: "301428697",
    title: "孰湖双生",
    video_urls: ["https://xy111x225x218x142xy.mcdn.bilivideo.cn:4483/upgcxcode/97/86/301428697/301428697-1-30032.m4s?e=ig8euxZM2rNcNbdlhoNvNC8BqJIzNbfqXBvEqxTEto8BTrNvN0GvT90W5JZMkX_YN0MvXg8gNEV4NC8xNEV4N03eN0B5tZlqNxTEto8BTrNvNeZVuJ10Kj_g2UB02J0mN0B5tZlqNCNEto8BTrNvNC7MTX502C8f2jmMQJ6mqF2fka1mqx6gqj0eN0B599M=&uipk=5&nbs=1&deadline=1722621726&gen=playurlv2&os=mcdn&oi=1035876564&trid=0000db23ebfe93334781b190a097b9646038u&mid=0&platform=pc&og=hw&upsig=d37e6b34f01330c2e3e0126f28868c9b&uparams=e,uipk,nbs,deadline,gen,os,oi,trid,mid,platform,og&mcdnid=50004915&bvc=vod&nettype=0&orderid=0,3&buvid=4CB663D6-6208-6C0E-4392-4C7DCEAB3E5E94322infoc&build=0&f=u_0_0&agrr=0&bw=49880&logo=A0020000", "https://xy111x225x218x142xy.mcdn.bilivideo.cn:4483/upgcxcode/97/86/301428697/301428697-1-30011.m4s?e=ig8euxZM2rNcNbdlhoNvNC8BqJIzNbfqXBvEqxTEto8BTrNvN0GvT90W5JZMkX_YN0MvXg8gNEV4NC8xNEV4N03eN0B5tZlqNxTEto8BTrNvNeZVuJ10Kj_g2UB02J0mN0B5tZlqNCNEto8BTrNvNC7MTX502C8f2jmMQJ6mqF2fka1mqx6gqj0eN0B599M=&uipk=5&nbs=1&deadline=1722621726&gen=playurlv2&os=mcdn&oi=1035876564&trid=0000db23ebfe93334781b190a097b9646038u&mid=0&platform=pc&og=hw&upsig=43409cf50f79c1a58fa6f504b8396ea4&uparams=e,uipk,nbs,deadline,gen,os,oi,trid,mid,platform,og&mcdnid=50004915&bvc=vod&nettype=0&orderid=0,3&buvid=4CB663D6-6208-6C0E-4392-4C7DCEAB3E5E94322infoc&build=0&f=u_0_0&agrr=0&bw=10573&logo=A0020000"],
    audio_url: "https://xy111x225x218x142xy.mcdn.bilivideo.cn:4483/upgcxcode/97/86/301428697/301428697-1-30280.m4s?e=ig8euxZM2rNcNbdlhoNvNC8BqJIzNbfqXBvEqxTEto8BTrNvN0GvT90W5JZMkX_YN0MvXg8gNEV4NC8xNEV4N03eN0B5tZlqNxTEto8BTrNvNeZVuJ10Kj_g2UB02J0mN0B5tZlqNCNEto8BTrNvNC7MTX502C8f2jmMQJ6mqF2fka1mqx6gqj0eN0B599M=&uipk=5&nbs=1&deadline=1722621726&gen=playurlv2&os=mcdn&oi=1035876564&trid=0000db23ebfe93334781b190a097b9646038u&mid=0&platform=pc&og=hw&upsig=0fe2d0a79cadffcf846f4ef0c8eface2&uparams=e,uipk,nbs,deadline,gen,os,oi,trid,mid,platform,og&mcdnid=50004915&bvc=vod&nettype=0&orderid=0,3&buvid=4CB663D6-6208-6C0E-4392-4C7DCEAB3E5E94322infoc&build=0&f=u_0_0&agrr=0&bw=16193&logo=A0020000",
    duration: 36000,
    cover: "http://i0.hdslb.com/bfs/archive/5adb111a55151544a7f3adb4b4f371a9b8cac3e2.png"
  }, {
    cid: "301428697",
    title: "手足相赎",
    video_urls: ["https://xy111x225x218x142xy.mcdn.bilivideo.cn:4483/upgcxcode/97/86/301428697/301428697-1-30032.m4s?e=ig8euxZM2rNcNbdlhoNvNC8BqJIzNbfqXBvEqxTEto8BTrNvN0GvT90W5JZMkX_YN0MvXg8gNEV4NC8xNEV4N03eN0B5tZlqNxTEto8BTrNvNeZVuJ10Kj_g2UB02J0mN0B5tZlqNCNEto8BTrNvNC7MTX502C8f2jmMQJ6mqF2fka1mqx6gqj0eN0B599M=&uipk=5&nbs=1&deadline=1722621726&gen=playurlv2&os=mcdn&oi=1035876564&trid=0000db23ebfe93334781b190a097b9646038u&mid=0&platform=pc&og=hw&upsig=d37e6b34f01330c2e3e0126f28868c9b&uparams=e,uipk,nbs,deadline,gen,os,oi,trid,mid,platform,og&mcdnid=50004915&bvc=vod&nettype=0&orderid=0,3&buvid=4CB663D6-6208-6C0E-4392-4C7DCEAB3E5E94322infoc&build=0&f=u_0_0&agrr=0&bw=49880&logo=A0020000", "https://xy111x225x218x142xy.mcdn.bilivideo.cn:4483/upgcxcode/97/86/301428697/301428697-1-30011.m4s?e=ig8euxZM2rNcNbdlhoNvNC8BqJIzNbfqXBvEqxTEto8BTrNvN0GvT90W5JZMkX_YN0MvXg8gNEV4NC8xNEV4N03eN0B5tZlqNxTEto8BTrNvNeZVuJ10Kj_g2UB02J0mN0B5tZlqNCNEto8BTrNvNC7MTX502C8f2jmMQJ6mqF2fka1mqx6gqj0eN0B599M=&uipk=5&nbs=1&deadline=1722621726&gen=playurlv2&os=mcdn&oi=1035876564&trid=0000db23ebfe93334781b190a097b9646038u&mid=0&platform=pc&og=hw&upsig=43409cf50f79c1a58fa6f504b8396ea4&uparams=e,uipk,nbs,deadline,gen,os,oi,trid,mid,platform,og&mcdnid=50004915&bvc=vod&nettype=0&orderid=0,3&buvid=4CB663D6-6208-6C0E-4392-4C7DCEAB3E5E94322infoc&build=0&f=u_0_0&agrr=0&bw=10573&logo=A0020000"],
    audio_url: "https://xy111x225x218x142xy.mcdn.bilivideo.cn:4483/upgcxcode/97/86/301428697/301428697-1-30280.m4s?e=ig8euxZM2rNcNbdlhoNvNC8BqJIzNbfqXBvEqxTEto8BTrNvN0GvT90W5JZMkX_YN0MvXg8gNEV4NC8xNEV4N03eN0B5tZlqNxTEto8BTrNvNeZVuJ10Kj_g2UB02J0mN0B5tZlqNCNEto8BTrNvNC7MTX502C8f2jmMQJ6mqF2fka1mqx6gqj0eN0B599M=&uipk=5&nbs=1&deadline=1722621726&gen=playurlv2&os=mcdn&oi=1035876564&trid=0000db23ebfe93334781b190a097b9646038u&mid=0&platform=pc&og=hw&upsig=0fe2d0a79cadffcf846f4ef0c8eface2&uparams=e,uipk,nbs,deadline,gen,os,oi,trid,mid,platform,og&mcdnid=50004915&bvc=vod&nettype=0&orderid=0,3&buvid=4CB663D6-6208-6C0E-4392-4C7DCEAB3E5E94322infoc&build=0&f=u_0_0&agrr=0&bw=16193&logo=A0020000",
    duration: 36000,
    cover: "http://i0.hdslb.com/bfs/archive/39eb28d10afd395acc4bdda5ebd92f13bbd64f22.png"
  }, {
    cid: "301428697",
    title: "手足相赎",
    video_urls: ["https://xy111x225x218x142xy.mcdn.bilivideo.cn:4483/upgcxcode/97/86/301428697/301428697-1-30032.m4s?e=ig8euxZM2rNcNbdlhoNvNC8BqJIzNbfqXBvEqxTEto8BTrNvN0GvT90W5JZMkX_YN0MvXg8gNEV4NC8xNEV4N03eN0B5tZlqNxTEto8BTrNvNeZVuJ10Kj_g2UB02J0mN0B5tZlqNCNEto8BTrNvNC7MTX502C8f2jmMQJ6mqF2fka1mqx6gqj0eN0B599M=&uipk=5&nbs=1&deadline=1722621726&gen=playurlv2&os=mcdn&oi=1035876564&trid=0000db23ebfe93334781b190a097b9646038u&mid=0&platform=pc&og=hw&upsig=d37e6b34f01330c2e3e0126f28868c9b&uparams=e,uipk,nbs,deadline,gen,os,oi,trid,mid,platform,og&mcdnid=50004915&bvc=vod&nettype=0&orderid=0,3&buvid=4CB663D6-6208-6C0E-4392-4C7DCEAB3E5E94322infoc&build=0&f=u_0_0&agrr=0&bw=49880&logo=A0020000", "https://xy111x225x218x142xy.mcdn.bilivideo.cn:4483/upgcxcode/97/86/301428697/301428697-1-30011.m4s?e=ig8euxZM2rNcNbdlhoNvNC8BqJIzNbfqXBvEqxTEto8BTrNvN0GvT90W5JZMkX_YN0MvXg8gNEV4NC8xNEV4N03eN0B5tZlqNxTEto8BTrNvNeZVuJ10Kj_g2UB02J0mN0B5tZlqNCNEto8BTrNvNC7MTX502C8f2jmMQJ6mqF2fka1mqx6gqj0eN0B599M=&uipk=5&nbs=1&deadline=1722621726&gen=playurlv2&os=mcdn&oi=1035876564&trid=0000db23ebfe93334781b190a097b9646038u&mid=0&platform=pc&og=hw&upsig=43409cf50f79c1a58fa6f504b8396ea4&uparams=e,uipk,nbs,deadline,gen,os,oi,trid,mid,platform,og&mcdnid=50004915&bvc=vod&nettype=0&orderid=0,3&buvid=4CB663D6-6208-6C0E-4392-4C7DCEAB3E5E94322infoc&build=0&f=u_0_0&agrr=0&bw=10573&logo=A0020000"],
    audio_url: "https://xy111x225x218x142xy.mcdn.bilivideo.cn:4483/upgcxcode/97/86/301428697/301428697-1-30280.m4s?e=ig8euxZM2rNcNbdlhoNvNC8BqJIzNbfqXBvEqxTEto8BTrNvN0GvT90W5JZMkX_YN0MvXg8gNEV4NC8xNEV4N03eN0B5tZlqNxTEto8BTrNvNeZVuJ10Kj_g2UB02J0mN0B5tZlqNCNEto8BTrNvNC7MTX502C8f2jmMQJ6mqF2fka1mqx6gqj0eN0B599M=&uipk=5&nbs=1&deadline=1722621726&gen=playurlv2&os=mcdn&oi=1035876564&trid=0000db23ebfe93334781b190a097b9646038u&mid=0&platform=pc&og=hw&upsig=0fe2d0a79cadffcf846f4ef0c8eface2&uparams=e,uipk,nbs,deadline,gen,os,oi,trid,mid,platform,og&mcdnid=50004915&bvc=vod&nettype=0&orderid=0,3&buvid=4CB663D6-6208-6C0E-4392-4C7DCEAB3E5E94322infoc&build=0&f=u_0_0&agrr=0&bw=16193&logo=A0020000",
    duration: 36000,
    cover: "http://i0.hdslb.com/bfs/archive/39eb28d10afd395acc4bdda5ebd92f13bbd64f22.png"
  }, {
    cid: "301428697",
    title: "手足相赎",
    video_urls: ["https://xy111x225x218x142xy.mcdn.bilivideo.cn:4483/upgcxcode/97/86/301428697/301428697-1-30032.m4s?e=ig8euxZM2rNcNbdlhoNvNC8BqJIzNbfqXBvEqxTEto8BTrNvN0GvT90W5JZMkX_YN0MvXg8gNEV4NC8xNEV4N03eN0B5tZlqNxTEto8BTrNvNeZVuJ10Kj_g2UB02J0mN0B5tZlqNCNEto8BTrNvNC7MTX502C8f2jmMQJ6mqF2fka1mqx6gqj0eN0B599M=&uipk=5&nbs=1&deadline=1722621726&gen=playurlv2&os=mcdn&oi=1035876564&trid=0000db23ebfe93334781b190a097b9646038u&mid=0&platform=pc&og=hw&upsig=d37e6b34f01330c2e3e0126f28868c9b&uparams=e,uipk,nbs,deadline,gen,os,oi,trid,mid,platform,og&mcdnid=50004915&bvc=vod&nettype=0&orderid=0,3&buvid=4CB663D6-6208-6C0E-4392-4C7DCEAB3E5E94322infoc&build=0&f=u_0_0&agrr=0&bw=49880&logo=A0020000", "https://xy111x225x218x142xy.mcdn.bilivideo.cn:4483/upgcxcode/97/86/301428697/301428697-1-30011.m4s?e=ig8euxZM2rNcNbdlhoNvNC8BqJIzNbfqXBvEqxTEto8BTrNvN0GvT90W5JZMkX_YN0MvXg8gNEV4NC8xNEV4N03eN0B5tZlqNxTEto8BTrNvNeZVuJ10Kj_g2UB02J0mN0B5tZlqNCNEto8BTrNvNC7MTX502C8f2jmMQJ6mqF2fka1mqx6gqj0eN0B599M=&uipk=5&nbs=1&deadline=1722621726&gen=playurlv2&os=mcdn&oi=1035876564&trid=0000db23ebfe93334781b190a097b9646038u&mid=0&platform=pc&og=hw&upsig=43409cf50f79c1a58fa6f504b8396ea4&uparams=e,uipk,nbs,deadline,gen,os,oi,trid,mid,platform,og&mcdnid=50004915&bvc=vod&nettype=0&orderid=0,3&buvid=4CB663D6-6208-6C0E-4392-4C7DCEAB3E5E94322infoc&build=0&f=u_0_0&agrr=0&bw=10573&logo=A0020000"],
    audio_url: "https://xy111x225x218x142xy.mcdn.bilivideo.cn:4483/upgcxcode/97/86/301428697/301428697-1-30280.m4s?e=ig8euxZM2rNcNbdlhoNvNC8BqJIzNbfqXBvEqxTEto8BTrNvN0GvT90W5JZMkX_YN0MvXg8gNEV4NC8xNEV4N03eN0B5tZlqNxTEto8BTrNvNeZVuJ10Kj_g2UB02J0mN0B5tZlqNCNEto8BTrNvNC7MTX502C8f2jmMQJ6mqF2fka1mqx6gqj0eN0B599M=&uipk=5&nbs=1&deadline=1722621726&gen=playurlv2&os=mcdn&oi=1035876564&trid=0000db23ebfe93334781b190a097b9646038u&mid=0&platform=pc&og=hw&upsig=0fe2d0a79cadffcf846f4ef0c8eface2&uparams=e,uipk,nbs,deadline,gen,os,oi,trid,mid,platform,og&mcdnid=50004915&bvc=vod&nettype=0&orderid=0,3&buvid=4CB663D6-6208-6C0E-4392-4C7DCEAB3E5E94322infoc&build=0&f=u_0_0&agrr=0&bw=16193&logo=A0020000",
    duration: 36000,
    cover: "http://i0.hdslb.com/bfs/archive/39eb28d10afd395acc4bdda5ebd92f13bbd64f22.png"
  }, {
    cid: "301428697",
    title: "手足相赎",
    video_urls: ["https://xy111x225x218x142xy.mcdn.bilivideo.cn:4483/upgcxcode/97/86/301428697/301428697-1-30032.m4s?e=ig8euxZM2rNcNbdlhoNvNC8BqJIzNbfqXBvEqxTEto8BTrNvN0GvT90W5JZMkX_YN0MvXg8gNEV4NC8xNEV4N03eN0B5tZlqNxTEto8BTrNvNeZVuJ10Kj_g2UB02J0mN0B5tZlqNCNEto8BTrNvNC7MTX502C8f2jmMQJ6mqF2fka1mqx6gqj0eN0B599M=&uipk=5&nbs=1&deadline=1722621726&gen=playurlv2&os=mcdn&oi=1035876564&trid=0000db23ebfe93334781b190a097b9646038u&mid=0&platform=pc&og=hw&upsig=d37e6b34f01330c2e3e0126f28868c9b&uparams=e,uipk,nbs,deadline,gen,os,oi,trid,mid,platform,og&mcdnid=50004915&bvc=vod&nettype=0&orderid=0,3&buvid=4CB663D6-6208-6C0E-4392-4C7DCEAB3E5E94322infoc&build=0&f=u_0_0&agrr=0&bw=49880&logo=A0020000", "https://xy111x225x218x142xy.mcdn.bilivideo.cn:4483/upgcxcode/97/86/301428697/301428697-1-30011.m4s?e=ig8euxZM2rNcNbdlhoNvNC8BqJIzNbfqXBvEqxTEto8BTrNvN0GvT90W5JZMkX_YN0MvXg8gNEV4NC8xNEV4N03eN0B5tZlqNxTEto8BTrNvNeZVuJ10Kj_g2UB02J0mN0B5tZlqNCNEto8BTrNvNC7MTX502C8f2jmMQJ6mqF2fka1mqx6gqj0eN0B599M=&uipk=5&nbs=1&deadline=1722621726&gen=playurlv2&os=mcdn&oi=1035876564&trid=0000db23ebfe93334781b190a097b9646038u&mid=0&platform=pc&og=hw&upsig=43409cf50f79c1a58fa6f504b8396ea4&uparams=e,uipk,nbs,deadline,gen,os,oi,trid,mid,platform,og&mcdnid=50004915&bvc=vod&nettype=0&orderid=0,3&buvid=4CB663D6-6208-6C0E-4392-4C7DCEAB3E5E94322infoc&build=0&f=u_0_0&agrr=0&bw=10573&logo=A0020000"],
    audio_url: "https://xy111x225x218x142xy.mcdn.bilivideo.cn:4483/upgcxcode/97/86/301428697/301428697-1-30280.m4s?e=ig8euxZM2rNcNbdlhoNvNC8BqJIzNbfqXBvEqxTEto8BTrNvN0GvT90W5JZMkX_YN0MvXg8gNEV4NC8xNEV4N03eN0B5tZlqNxTEto8BTrNvNeZVuJ10Kj_g2UB02J0mN0B5tZlqNCNEto8BTrNvNC7MTX502C8f2jmMQJ6mqF2fka1mqx6gqj0eN0B599M=&uipk=5&nbs=1&deadline=1722621726&gen=playurlv2&os=mcdn&oi=1035876564&trid=0000db23ebfe93334781b190a097b9646038u&mid=0&platform=pc&og=hw&upsig=0fe2d0a79cadffcf846f4ef0c8eface2&uparams=e,uipk,nbs,deadline,gen,os,oi,trid,mid,platform,og&mcdnid=50004915&bvc=vod&nettype=0&orderid=0,3&buvid=4CB663D6-6208-6C0E-4392-4C7DCEAB3E5E94322infoc&build=0&f=u_0_0&agrr=0&bw=16193&logo=A0020000",
    duration: 36000,
    cover: "http://i0.hdslb.com/bfs/archive/39eb28d10afd395acc4bdda5ebd92f13bbd64f22.png"
  }, {
    cid: "301428697",
    title: "手足相赎",
    video_urls: ["https://xy111x225x218x142xy.mcdn.bilivideo.cn:4483/upgcxcode/97/86/301428697/301428697-1-30032.m4s?e=ig8euxZM2rNcNbdlhoNvNC8BqJIzNbfqXBvEqxTEto8BTrNvN0GvT90W5JZMkX_YN0MvXg8gNEV4NC8xNEV4N03eN0B5tZlqNxTEto8BTrNvNeZVuJ10Kj_g2UB02J0mN0B5tZlqNCNEto8BTrNvNC7MTX502C8f2jmMQJ6mqF2fka1mqx6gqj0eN0B599M=&uipk=5&nbs=1&deadline=1722621726&gen=playurlv2&os=mcdn&oi=1035876564&trid=0000db23ebfe93334781b190a097b9646038u&mid=0&platform=pc&og=hw&upsig=d37e6b34f01330c2e3e0126f28868c9b&uparams=e,uipk,nbs,deadline,gen,os,oi,trid,mid,platform,og&mcdnid=50004915&bvc=vod&nettype=0&orderid=0,3&buvid=4CB663D6-6208-6C0E-4392-4C7DCEAB3E5E94322infoc&build=0&f=u_0_0&agrr=0&bw=49880&logo=A0020000", "https://xy111x225x218x142xy.mcdn.bilivideo.cn:4483/upgcxcode/97/86/301428697/301428697-1-30011.m4s?e=ig8euxZM2rNcNbdlhoNvNC8BqJIzNbfqXBvEqxTEto8BTrNvN0GvT90W5JZMkX_YN0MvXg8gNEV4NC8xNEV4N03eN0B5tZlqNxTEto8BTrNvNeZVuJ10Kj_g2UB02J0mN0B5tZlqNCNEto8BTrNvNC7MTX502C8f2jmMQJ6mqF2fka1mqx6gqj0eN0B599M=&uipk=5&nbs=1&deadline=1722621726&gen=playurlv2&os=mcdn&oi=1035876564&trid=0000db23ebfe93334781b190a097b9646038u&mid=0&platform=pc&og=hw&upsig=43409cf50f79c1a58fa6f504b8396ea4&uparams=e,uipk,nbs,deadline,gen,os,oi,trid,mid,platform,og&mcdnid=50004915&bvc=vod&nettype=0&orderid=0,3&buvid=4CB663D6-6208-6C0E-4392-4C7DCEAB3E5E94322infoc&build=0&f=u_0_0&agrr=0&bw=10573&logo=A0020000"],
    audio_url: "https://xy111x225x218x142xy.mcdn.bilivideo.cn:4483/upgcxcode/97/86/301428697/301428697-1-30280.m4s?e=ig8euxZM2rNcNbdlhoNvNC8BqJIzNbfqXBvEqxTEto8BTrNvN0GvT90W5JZMkX_YN0MvXg8gNEV4NC8xNEV4N03eN0B5tZlqNxTEto8BTrNvNeZVuJ10Kj_g2UB02J0mN0B5tZlqNCNEto8BTrNvNC7MTX502C8f2jmMQJ6mqF2fka1mqx6gqj0eN0B599M=&uipk=5&nbs=1&deadline=1722621726&gen=playurlv2&os=mcdn&oi=1035876564&trid=0000db23ebfe93334781b190a097b9646038u&mid=0&platform=pc&og=hw&upsig=0fe2d0a79cadffcf846f4ef0c8eface2&uparams=e,uipk,nbs,deadline,gen,os,oi,trid,mid,platform,og&mcdnid=50004915&bvc=vod&nettype=0&orderid=0,3&buvid=4CB663D6-6208-6C0E-4392-4C7DCEAB3E5E94322infoc&build=0&f=u_0_0&agrr=0&bw=16193&logo=A0020000",
    duration: 36000,
    cover: "http://i0.hdslb.com/bfs/archive/39eb28d10afd395acc4bdda5ebd92f13bbd64f22.png"
  }]
});

const options = ref<string[]>([]);

onMounted(() => {
  loading.value = true;
  setTimeout(() => {
    loading.value = false;
  }, 2000);
  console.log(route.params.bvid);
  options.value = animeInfo.value.formats;
  value.value = 0;
  console.log(radio.value);
})

const goBack = () => {
  console.log('go back');
  history.back();
}
</script>

<template>
  <div class="download-video" v-loading="loading">
    <el-container style="height: 100%">
      <el-header>
        <el-page-header :icon="ArrowLeft" @back="goBack" title="返回" style="border-bottom: 1px solid var(--el-text-color-placeholder);
  padding-bottom: 5px">
          <template #content>
            <el-button>下载</el-button>
          </template>
        </el-page-header>
      </el-header>
      <el-main>
        <div class="content">
          <div class="left">
            <div style="display: flex; width: 100%">
              <div
                  style="display: flex; flex-direction: column; align-items: flex-start; justify-content: space-between; margin-right: 20px">
                <div>
                  <el-text tag="b" size="large">{{ animeInfo.title }}</el-text>
                </div>
                <div>
                  <el-text tag="b">类型:</el-text>
                  <el-text>
                    {{ animeInfo.play.split(" · ")[2].substring(animeInfo.play.split(" · ")[2].indexOf("番") + 1) }}
                  </el-text>
                </div>
                <div>
                  <el-text tag="b">年份:&nbsp;</el-text>
                  <el-text>{{ animeInfo.date }}</el-text>
                </div>
                <div>
                  <el-text tag="b">评分:&nbsp;</el-text>
                  <el-text>{{ animeInfo.score }}</el-text>
                </div>
                <div>
                  <el-text tag="b">集数:&nbsp;</el-text>
                  <el-text>{{ animeInfo.play.split(" · ")[4] }}</el-text>
                </div>
              </div>
              <div>
                <el-image
                    fit="cover"
                    style="width: 160px; height: 200px; border-radius: 10px"
                    :src="animeInfo.cover"
                />
              </div>
            </div>
            <div class="description">
              <el-text tag="b">描述:</el-text>
              <div style="height: 16px"></div>
              <el-text>{{ animeInfo.description }}</el-text>
            </div>
            <div class="data">
              <div class="info">
                <el-text tag="b">播放量:</el-text>
                <el-text>
                  {{ animeInfo.play.split(" · ")[0].substring(0, animeInfo.play.split(" · ")[0].indexOf("万") + 1) }}
                </el-text>
              </div>
              <el-divider direction="vertical" border-style="none"/>
              <el-divider direction="vertical" border-style="none"/>
              <div class="info">
                <el-text tag="b">弹幕量:</el-text>
                <el-text>
                  {{ animeInfo.play.split(" · ")[1].substring(0, animeInfo.play.split(" · ")[1].indexOf("万") + 1) }}
                </el-text>
              </div>
              <el-divider direction="vertical" border-style="none"/>
              <el-divider direction="vertical" border-style="none"/>
              <div class="info">
                <el-text tag="b">系列追番:</el-text>
                <el-text>
                  {{
                    animeInfo.play.split(" · ")[2].substring(0, animeInfo.play.split(" · ")[2].indexOf("番"))
                        .substring(0, animeInfo.play.split(" · ")[2].substring(0, animeInfo.play.split(" · ")[2].indexOf("番")).indexOf("万") + 1)
                  }}
                </el-text>
              </div>
            </div>
          </div>
          <div class="right">
            <div class="episode">
              <el-checkbox-group
                  v-model="checkboxGroup1"
                  style="overflow-y: scroll;"
                  text-color="white"
                  fill="#ff99b3"
              >
                <div v-for="(episode, index) in animeInfo.episodes">
                  <div style="display: flex">
                    <el-image
                        fit="cover"
                        style="width: 100px; height: 60px"
                        :src="episode.cover"
                    />
                    <el-checkbox-button :key="index" :value="index" style="width: 60%; flex: 1">
                      <template #default>
                        <div style="height: 43px; width: 100%">
                          <div style="text-overflow: ellipsis; white-space: nowrap; height: 100%; align-content: center">{{ index + 1 }}.{{ episode.title }}&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;</div>
                        </div>
                      </template>
                    </el-checkbox-button>
                  </div>
                  <div style="height: 10px"></div>
                </div>
              </el-checkbox-group>
            </div>
            <el-radio-group
                v-model="value"
                style="width: 100%"
                text-color="white"
                fill="#ff99b3"
            >
              <div v-for="(format, index) in animeInfo.formats" style="margin: 5px">
                <el-radio-button :value="index" :label="format"/>
              </div>
            </el-radio-group>
          </div>
        </div>
      </el-main>
    </el-container>
  </div>
</template>

<style scoped>
span {
  color: var(--el-color-info-dark-2);
}

.left {
  width: 60%;
  height: 100%;
  display: flex;
  flex-direction: column;
  align-items: center;
  margin-right: 30px;
}

.right {
  width: 40%;
  height: 100%;
  display: flex;
  flex-direction: column;
}

.content {
  display: flex;
  width: 100%;
  height: 100%;
  justify-content: space-between;
  margin: 0 auto;
}

.download-video {
  height: 100%;
  width: 100%;
}

.description {
  margin-top: 30px;
  display: -webkit-box;
  -webkit-box-orient: vertical;
  -webkit-line-clamp: 6;
  overflow: hidden;
  text-overflow: ellipsis;
}

.data {
  margin-top: 30px;
  display: flex;
  width: 100%;
}

.info {
  background-color: var(--el-color-primary-light-9);
  padding: 16px;
  border-radius: 16px;
}

.episode {
  display: flex;
  margin-bottom: 20px;
  background-color: var(--el-color-primary-light-9);
  border-radius: 10px;
  padding: 20px;
  max-height: 70%;
}
</style>