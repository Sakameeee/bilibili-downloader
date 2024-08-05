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
  description: "在第三季中，桃夭追着百妖谱的气息一路来到京师，为了养活一僧一蛇一狐，她在京师的大户人家“司府”担任喂马杂役。尽管自称只救妖不救人，但当面对暗刀、百知、风果、孰湖、枫生、趸鱼、镇水等妖怪和事中人时，桃夭心怀善念，解铃还愿。这一季充满了情感与冒险，展现出一系列感人至深的治愈故事~",
  index: 12,
  date: "2024",
  score: "9.6分",
  formats: ["高清 1080P+", "高清 1080P", "高清 720P"],
  episodes: [{
    cid: "301428697",
    title: "Lecture 1 - Introduction",
    video_urls: ["https://xy111x225x218x142xy.mcdn.bilivideo.cn:4483/upgcxcode/97/86/301428697/301428697-1-30032.m4s?e=ig8euxZM2rNcNbdlhoNvNC8BqJIzNbfqXBvEqxTEto8BTrNvN0GvT90W5JZMkX_YN0MvXg8gNEV4NC8xNEV4N03eN0B5tZlqNxTEto8BTrNvNeZVuJ10Kj_g2UB02J0mN0B5tZlqNCNEto8BTrNvNC7MTX502C8f2jmMQJ6mqF2fka1mqx6gqj0eN0B599M=&uipk=5&nbs=1&deadline=1722621726&gen=playurlv2&os=mcdn&oi=1035876564&trid=0000db23ebfe93334781b190a097b9646038u&mid=0&platform=pc&og=hw&upsig=d37e6b34f01330c2e3e0126f28868c9b&uparams=e,uipk,nbs,deadline,gen,os,oi,trid,mid,platform,og&mcdnid=50004915&bvc=vod&nettype=0&orderid=0,3&buvid=4CB663D6-6208-6C0E-4392-4C7DCEAB3E5E94322infoc&build=0&f=u_0_0&agrr=0&bw=49880&logo=A0020000", "https://xy111x225x218x142xy.mcdn.bilivideo.cn:4483/upgcxcode/97/86/301428697/301428697-1-30011.m4s?e=ig8euxZM2rNcNbdlhoNvNC8BqJIzNbfqXBvEqxTEto8BTrNvN0GvT90W5JZMkX_YN0MvXg8gNEV4NC8xNEV4N03eN0B5tZlqNxTEto8BTrNvNeZVuJ10Kj_g2UB02J0mN0B5tZlqNCNEto8BTrNvNC7MTX502C8f2jmMQJ6mqF2fka1mqx6gqj0eN0B599M=&uipk=5&nbs=1&deadline=1722621726&gen=playurlv2&os=mcdn&oi=1035876564&trid=0000db23ebfe93334781b190a097b9646038u&mid=0&platform=pc&og=hw&upsig=43409cf50f79c1a58fa6f504b8396ea4&uparams=e,uipk,nbs,deadline,gen,os,oi,trid,mid,platform,og&mcdnid=50004915&bvc=vod&nettype=0&orderid=0,3&buvid=4CB663D6-6208-6C0E-4392-4C7DCEAB3E5E94322infoc&build=0&f=u_0_0&agrr=0&bw=10573&logo=A0020000"],
    audio_url: "https://xy111x225x218x142xy.mcdn.bilivideo.cn:4483/upgcxcode/97/86/301428697/301428697-1-30280.m4s?e=ig8euxZM2rNcNbdlhoNvNC8BqJIzNbfqXBvEqxTEto8BTrNvN0GvT90W5JZMkX_YN0MvXg8gNEV4NC8xNEV4N03eN0B5tZlqNxTEto8BTrNvNeZVuJ10Kj_g2UB02J0mN0B5tZlqNCNEto8BTrNvNC7MTX502C8f2jmMQJ6mqF2fka1mqx6gqj0eN0B599M=&uipk=5&nbs=1&deadline=1722621726&gen=playurlv2&os=mcdn&oi=1035876564&trid=0000db23ebfe93334781b190a097b9646038u&mid=0&platform=pc&og=hw&upsig=0fe2d0a79cadffcf846f4ef0c8eface2&uparams=e,uipk,nbs,deadline,gen,os,oi,trid,mid,platform,og&mcdnid=50004915&bvc=vod&nettype=0&orderid=0,3&buvid=4CB663D6-6208-6C0E-4392-4C7DCEAB3E5E94322infoc&build=0&f=u_0_0&agrr=0&bw=16193&logo=A0020000",
    duration: 4749,
  }, {
    cid: "301428697",
    title: "Lecture 1 - Introduction",
    video_urls: ["https://xy111x225x218x142xy.mcdn.bilivideo.cn:4483/upgcxcode/97/86/301428697/301428697-1-30032.m4s?e=ig8euxZM2rNcNbdlhoNvNC8BqJIzNbfqXBvEqxTEto8BTrNvN0GvT90W5JZMkX_YN0MvXg8gNEV4NC8xNEV4N03eN0B5tZlqNxTEto8BTrNvNeZVuJ10Kj_g2UB02J0mN0B5tZlqNCNEto8BTrNvNC7MTX502C8f2jmMQJ6mqF2fka1mqx6gqj0eN0B599M=&uipk=5&nbs=1&deadline=1722621726&gen=playurlv2&os=mcdn&oi=1035876564&trid=0000db23ebfe93334781b190a097b9646038u&mid=0&platform=pc&og=hw&upsig=d37e6b34f01330c2e3e0126f28868c9b&uparams=e,uipk,nbs,deadline,gen,os,oi,trid,mid,platform,og&mcdnid=50004915&bvc=vod&nettype=0&orderid=0,3&buvid=4CB663D6-6208-6C0E-4392-4C7DCEAB3E5E94322infoc&build=0&f=u_0_0&agrr=0&bw=49880&logo=A0020000", "https://xy111x225x218x142xy.mcdn.bilivideo.cn:4483/upgcxcode/97/86/301428697/301428697-1-30011.m4s?e=ig8euxZM2rNcNbdlhoNvNC8BqJIzNbfqXBvEqxTEto8BTrNvN0GvT90W5JZMkX_YN0MvXg8gNEV4NC8xNEV4N03eN0B5tZlqNxTEto8BTrNvNeZVuJ10Kj_g2UB02J0mN0B5tZlqNCNEto8BTrNvNC7MTX502C8f2jmMQJ6mqF2fka1mqx6gqj0eN0B599M=&uipk=5&nbs=1&deadline=1722621726&gen=playurlv2&os=mcdn&oi=1035876564&trid=0000db23ebfe93334781b190a097b9646038u&mid=0&platform=pc&og=hw&upsig=43409cf50f79c1a58fa6f504b8396ea4&uparams=e,uipk,nbs,deadline,gen,os,oi,trid,mid,platform,og&mcdnid=50004915&bvc=vod&nettype=0&orderid=0,3&buvid=4CB663D6-6208-6C0E-4392-4C7DCEAB3E5E94322infoc&build=0&f=u_0_0&agrr=0&bw=10573&logo=A0020000"],
    audio_url: "https://xy111x225x218x142xy.mcdn.bilivideo.cn:4483/upgcxcode/97/86/301428697/301428697-1-30280.m4s?e=ig8euxZM2rNcNbdlhoNvNC8BqJIzNbfqXBvEqxTEto8BTrNvN0GvT90W5JZMkX_YN0MvXg8gNEV4NC8xNEV4N03eN0B5tZlqNxTEto8BTrNvNeZVuJ10Kj_g2UB02J0mN0B5tZlqNCNEto8BTrNvNC7MTX502C8f2jmMQJ6mqF2fka1mqx6gqj0eN0B599M=&uipk=5&nbs=1&deadline=1722621726&gen=playurlv2&os=mcdn&oi=1035876564&trid=0000db23ebfe93334781b190a097b9646038u&mid=0&platform=pc&og=hw&upsig=0fe2d0a79cadffcf846f4ef0c8eface2&uparams=e,uipk,nbs,deadline,gen,os,oi,trid,mid,platform,og&mcdnid=50004915&bvc=vod&nettype=0&orderid=0,3&buvid=4CB663D6-6208-6C0E-4392-4C7DCEAB3E5E94322infoc&build=0&f=u_0_0&agrr=0&bw=16193&logo=A0020000",
    duration: 4749,
  }, {
    cid: "301428697",
    title: "Lecture 1 - Introduction",
    video_urls: ["https://xy111x225x218x142xy.mcdn.bilivideo.cn:4483/upgcxcode/97/86/301428697/301428697-1-30032.m4s?e=ig8euxZM2rNcNbdlhoNvNC8BqJIzNbfqXBvEqxTEto8BTrNvN0GvT90W5JZMkX_YN0MvXg8gNEV4NC8xNEV4N03eN0B5tZlqNxTEto8BTrNvNeZVuJ10Kj_g2UB02J0mN0B5tZlqNCNEto8BTrNvNC7MTX502C8f2jmMQJ6mqF2fka1mqx6gqj0eN0B599M=&uipk=5&nbs=1&deadline=1722621726&gen=playurlv2&os=mcdn&oi=1035876564&trid=0000db23ebfe93334781b190a097b9646038u&mid=0&platform=pc&og=hw&upsig=d37e6b34f01330c2e3e0126f28868c9b&uparams=e,uipk,nbs,deadline,gen,os,oi,trid,mid,platform,og&mcdnid=50004915&bvc=vod&nettype=0&orderid=0,3&buvid=4CB663D6-6208-6C0E-4392-4C7DCEAB3E5E94322infoc&build=0&f=u_0_0&agrr=0&bw=49880&logo=A0020000", "https://xy111x225x218x142xy.mcdn.bilivideo.cn:4483/upgcxcode/97/86/301428697/301428697-1-30011.m4s?e=ig8euxZM2rNcNbdlhoNvNC8BqJIzNbfqXBvEqxTEto8BTrNvN0GvT90W5JZMkX_YN0MvXg8gNEV4NC8xNEV4N03eN0B5tZlqNxTEto8BTrNvNeZVuJ10Kj_g2UB02J0mN0B5tZlqNCNEto8BTrNvNC7MTX502C8f2jmMQJ6mqF2fka1mqx6gqj0eN0B599M=&uipk=5&nbs=1&deadline=1722621726&gen=playurlv2&os=mcdn&oi=1035876564&trid=0000db23ebfe93334781b190a097b9646038u&mid=0&platform=pc&og=hw&upsig=43409cf50f79c1a58fa6f504b8396ea4&uparams=e,uipk,nbs,deadline,gen,os,oi,trid,mid,platform,og&mcdnid=50004915&bvc=vod&nettype=0&orderid=0,3&buvid=4CB663D6-6208-6C0E-4392-4C7DCEAB3E5E94322infoc&build=0&f=u_0_0&agrr=0&bw=10573&logo=A0020000"],
    audio_url: "https://xy111x225x218x142xy.mcdn.bilivideo.cn:4483/upgcxcode/97/86/301428697/301428697-1-30280.m4s?e=ig8euxZM2rNcNbdlhoNvNC8BqJIzNbfqXBvEqxTEto8BTrNvN0GvT90W5JZMkX_YN0MvXg8gNEV4NC8xNEV4N03eN0B5tZlqNxTEto8BTrNvNeZVuJ10Kj_g2UB02J0mN0B5tZlqNCNEto8BTrNvNC7MTX502C8f2jmMQJ6mqF2fka1mqx6gqj0eN0B599M=&uipk=5&nbs=1&deadline=1722621726&gen=playurlv2&os=mcdn&oi=1035876564&trid=0000db23ebfe93334781b190a097b9646038u&mid=0&platform=pc&og=hw&upsig=0fe2d0a79cadffcf846f4ef0c8eface2&uparams=e,uipk,nbs,deadline,gen,os,oi,trid,mid,platform,og&mcdnid=50004915&bvc=vod&nettype=0&orderid=0,3&buvid=4CB663D6-6208-6C0E-4392-4C7DCEAB3E5E94322infoc&build=0&f=u_0_0&agrr=0&bw=16193&logo=A0020000",
    duration: 4749,
  }, {
    cid: "301428697",
    title: "Lecture 1 - Introduction",
    video_urls: ["https://xy111x225x218x142xy.mcdn.bilivideo.cn:4483/upgcxcode/97/86/301428697/301428697-1-30032.m4s?e=ig8euxZM2rNcNbdlhoNvNC8BqJIzNbfqXBvEqxTEto8BTrNvN0GvT90W5JZMkX_YN0MvXg8gNEV4NC8xNEV4N03eN0B5tZlqNxTEto8BTrNvNeZVuJ10Kj_g2UB02J0mN0B5tZlqNCNEto8BTrNvNC7MTX502C8f2jmMQJ6mqF2fka1mqx6gqj0eN0B599M=&uipk=5&nbs=1&deadline=1722621726&gen=playurlv2&os=mcdn&oi=1035876564&trid=0000db23ebfe93334781b190a097b9646038u&mid=0&platform=pc&og=hw&upsig=d37e6b34f01330c2e3e0126f28868c9b&uparams=e,uipk,nbs,deadline,gen,os,oi,trid,mid,platform,og&mcdnid=50004915&bvc=vod&nettype=0&orderid=0,3&buvid=4CB663D6-6208-6C0E-4392-4C7DCEAB3E5E94322infoc&build=0&f=u_0_0&agrr=0&bw=49880&logo=A0020000", "https://xy111x225x218x142xy.mcdn.bilivideo.cn:4483/upgcxcode/97/86/301428697/301428697-1-30011.m4s?e=ig8euxZM2rNcNbdlhoNvNC8BqJIzNbfqXBvEqxTEto8BTrNvN0GvT90W5JZMkX_YN0MvXg8gNEV4NC8xNEV4N03eN0B5tZlqNxTEto8BTrNvNeZVuJ10Kj_g2UB02J0mN0B5tZlqNCNEto8BTrNvNC7MTX502C8f2jmMQJ6mqF2fka1mqx6gqj0eN0B599M=&uipk=5&nbs=1&deadline=1722621726&gen=playurlv2&os=mcdn&oi=1035876564&trid=0000db23ebfe93334781b190a097b9646038u&mid=0&platform=pc&og=hw&upsig=43409cf50f79c1a58fa6f504b8396ea4&uparams=e,uipk,nbs,deadline,gen,os,oi,trid,mid,platform,og&mcdnid=50004915&bvc=vod&nettype=0&orderid=0,3&buvid=4CB663D6-6208-6C0E-4392-4C7DCEAB3E5E94322infoc&build=0&f=u_0_0&agrr=0&bw=10573&logo=A0020000"],
    audio_url: "https://xy111x225x218x142xy.mcdn.bilivideo.cn:4483/upgcxcode/97/86/301428697/301428697-1-30280.m4s?e=ig8euxZM2rNcNbdlhoNvNC8BqJIzNbfqXBvEqxTEto8BTrNvN0GvT90W5JZMkX_YN0MvXg8gNEV4NC8xNEV4N03eN0B5tZlqNxTEto8BTrNvNeZVuJ10Kj_g2UB02J0mN0B5tZlqNCNEto8BTrNvNC7MTX502C8f2jmMQJ6mqF2fka1mqx6gqj0eN0B599M=&uipk=5&nbs=1&deadline=1722621726&gen=playurlv2&os=mcdn&oi=1035876564&trid=0000db23ebfe93334781b190a097b9646038u&mid=0&platform=pc&og=hw&upsig=0fe2d0a79cadffcf846f4ef0c8eface2&uparams=e,uipk,nbs,deadline,gen,os,oi,trid,mid,platform,og&mcdnid=50004915&bvc=vod&nettype=0&orderid=0,3&buvid=4CB663D6-6208-6C0E-4392-4C7DCEAB3E5E94322infoc&build=0&f=u_0_0&agrr=0&bw=16193&logo=A0020000",
    duration: 4749,
  }, {
    cid: "301428697",
    title: "Lecture 1 - Introduction",
    video_urls: ["https://xy111x225x218x142xy.mcdn.bilivideo.cn:4483/upgcxcode/97/86/301428697/301428697-1-30032.m4s?e=ig8euxZM2rNcNbdlhoNvNC8BqJIzNbfqXBvEqxTEto8BTrNvN0GvT90W5JZMkX_YN0MvXg8gNEV4NC8xNEV4N03eN0B5tZlqNxTEto8BTrNvNeZVuJ10Kj_g2UB02J0mN0B5tZlqNCNEto8BTrNvNC7MTX502C8f2jmMQJ6mqF2fka1mqx6gqj0eN0B599M=&uipk=5&nbs=1&deadline=1722621726&gen=playurlv2&os=mcdn&oi=1035876564&trid=0000db23ebfe93334781b190a097b9646038u&mid=0&platform=pc&og=hw&upsig=d37e6b34f01330c2e3e0126f28868c9b&uparams=e,uipk,nbs,deadline,gen,os,oi,trid,mid,platform,og&mcdnid=50004915&bvc=vod&nettype=0&orderid=0,3&buvid=4CB663D6-6208-6C0E-4392-4C7DCEAB3E5E94322infoc&build=0&f=u_0_0&agrr=0&bw=49880&logo=A0020000", "https://xy111x225x218x142xy.mcdn.bilivideo.cn:4483/upgcxcode/97/86/301428697/301428697-1-30011.m4s?e=ig8euxZM2rNcNbdlhoNvNC8BqJIzNbfqXBvEqxTEto8BTrNvN0GvT90W5JZMkX_YN0MvXg8gNEV4NC8xNEV4N03eN0B5tZlqNxTEto8BTrNvNeZVuJ10Kj_g2UB02J0mN0B5tZlqNCNEto8BTrNvNC7MTX502C8f2jmMQJ6mqF2fka1mqx6gqj0eN0B599M=&uipk=5&nbs=1&deadline=1722621726&gen=playurlv2&os=mcdn&oi=1035876564&trid=0000db23ebfe93334781b190a097b9646038u&mid=0&platform=pc&og=hw&upsig=43409cf50f79c1a58fa6f504b8396ea4&uparams=e,uipk,nbs,deadline,gen,os,oi,trid,mid,platform,og&mcdnid=50004915&bvc=vod&nettype=0&orderid=0,3&buvid=4CB663D6-6208-6C0E-4392-4C7DCEAB3E5E94322infoc&build=0&f=u_0_0&agrr=0&bw=10573&logo=A0020000"],
    audio_url: "https://xy111x225x218x142xy.mcdn.bilivideo.cn:4483/upgcxcode/97/86/301428697/301428697-1-30280.m4s?e=ig8euxZM2rNcNbdlhoNvNC8BqJIzNbfqXBvEqxTEto8BTrNvN0GvT90W5JZMkX_YN0MvXg8gNEV4NC8xNEV4N03eN0B5tZlqNxTEto8BTrNvNeZVuJ10Kj_g2UB02J0mN0B5tZlqNCNEto8BTrNvNC7MTX502C8f2jmMQJ6mqF2fka1mqx6gqj0eN0B599M=&uipk=5&nbs=1&deadline=1722621726&gen=playurlv2&os=mcdn&oi=1035876564&trid=0000db23ebfe93334781b190a097b9646038u&mid=0&platform=pc&og=hw&upsig=0fe2d0a79cadffcf846f4ef0c8eface2&uparams=e,uipk,nbs,deadline,gen,os,oi,trid,mid,platform,og&mcdnid=50004915&bvc=vod&nettype=0&orderid=0,3&buvid=4CB663D6-6208-6C0E-4392-4C7DCEAB3E5E94322infoc&build=0&f=u_0_0&agrr=0&bw=16193&logo=A0020000",
    duration: 4749,
  }, {
    cid: "301428697",
    title: "Lecture 1 - Introduction",
    video_urls: ["https://xy111x225x218x142xy.mcdn.bilivideo.cn:4483/upgcxcode/97/86/301428697/301428697-1-30032.m4s?e=ig8euxZM2rNcNbdlhoNvNC8BqJIzNbfqXBvEqxTEto8BTrNvN0GvT90W5JZMkX_YN0MvXg8gNEV4NC8xNEV4N03eN0B5tZlqNxTEto8BTrNvNeZVuJ10Kj_g2UB02J0mN0B5tZlqNCNEto8BTrNvNC7MTX502C8f2jmMQJ6mqF2fka1mqx6gqj0eN0B599M=&uipk=5&nbs=1&deadline=1722621726&gen=playurlv2&os=mcdn&oi=1035876564&trid=0000db23ebfe93334781b190a097b9646038u&mid=0&platform=pc&og=hw&upsig=d37e6b34f01330c2e3e0126f28868c9b&uparams=e,uipk,nbs,deadline,gen,os,oi,trid,mid,platform,og&mcdnid=50004915&bvc=vod&nettype=0&orderid=0,3&buvid=4CB663D6-6208-6C0E-4392-4C7DCEAB3E5E94322infoc&build=0&f=u_0_0&agrr=0&bw=49880&logo=A0020000", "https://xy111x225x218x142xy.mcdn.bilivideo.cn:4483/upgcxcode/97/86/301428697/301428697-1-30011.m4s?e=ig8euxZM2rNcNbdlhoNvNC8BqJIzNbfqXBvEqxTEto8BTrNvN0GvT90W5JZMkX_YN0MvXg8gNEV4NC8xNEV4N03eN0B5tZlqNxTEto8BTrNvNeZVuJ10Kj_g2UB02J0mN0B5tZlqNCNEto8BTrNvNC7MTX502C8f2jmMQJ6mqF2fka1mqx6gqj0eN0B599M=&uipk=5&nbs=1&deadline=1722621726&gen=playurlv2&os=mcdn&oi=1035876564&trid=0000db23ebfe93334781b190a097b9646038u&mid=0&platform=pc&og=hw&upsig=43409cf50f79c1a58fa6f504b8396ea4&uparams=e,uipk,nbs,deadline,gen,os,oi,trid,mid,platform,og&mcdnid=50004915&bvc=vod&nettype=0&orderid=0,3&buvid=4CB663D6-6208-6C0E-4392-4C7DCEAB3E5E94322infoc&build=0&f=u_0_0&agrr=0&bw=10573&logo=A0020000"],
    audio_url: "https://xy111x225x218x142xy.mcdn.bilivideo.cn:4483/upgcxcode/97/86/301428697/301428697-1-30280.m4s?e=ig8euxZM2rNcNbdlhoNvNC8BqJIzNbfqXBvEqxTEto8BTrNvN0GvT90W5JZMkX_YN0MvXg8gNEV4NC8xNEV4N03eN0B5tZlqNxTEto8BTrNvNeZVuJ10Kj_g2UB02J0mN0B5tZlqNCNEto8BTrNvNC7MTX502C8f2jmMQJ6mqF2fka1mqx6gqj0eN0B599M=&uipk=5&nbs=1&deadline=1722621726&gen=playurlv2&os=mcdn&oi=1035876564&trid=0000db23ebfe93334781b190a097b9646038u&mid=0&platform=pc&og=hw&upsig=0fe2d0a79cadffcf846f4ef0c8eface2&uparams=e,uipk,nbs,deadline,gen,os,oi,trid,mid,platform,og&mcdnid=50004915&bvc=vod&nettype=0&orderid=0,3&buvid=4CB663D6-6208-6C0E-4392-4C7DCEAB3E5E94322infoc&build=0&f=u_0_0&agrr=0&bw=16193&logo=A0020000",
    duration: 4749,
  }, {
    cid: "301428697",
    title: "Lecture 1 - Introduction",
    video_urls: ["https://xy111x225x218x142xy.mcdn.bilivideo.cn:4483/upgcxcode/97/86/301428697/301428697-1-30032.m4s?e=ig8euxZM2rNcNbdlhoNvNC8BqJIzNbfqXBvEqxTEto8BTrNvN0GvT90W5JZMkX_YN0MvXg8gNEV4NC8xNEV4N03eN0B5tZlqNxTEto8BTrNvNeZVuJ10Kj_g2UB02J0mN0B5tZlqNCNEto8BTrNvNC7MTX502C8f2jmMQJ6mqF2fka1mqx6gqj0eN0B599M=&uipk=5&nbs=1&deadline=1722621726&gen=playurlv2&os=mcdn&oi=1035876564&trid=0000db23ebfe93334781b190a097b9646038u&mid=0&platform=pc&og=hw&upsig=d37e6b34f01330c2e3e0126f28868c9b&uparams=e,uipk,nbs,deadline,gen,os,oi,trid,mid,platform,og&mcdnid=50004915&bvc=vod&nettype=0&orderid=0,3&buvid=4CB663D6-6208-6C0E-4392-4C7DCEAB3E5E94322infoc&build=0&f=u_0_0&agrr=0&bw=49880&logo=A0020000", "https://xy111x225x218x142xy.mcdn.bilivideo.cn:4483/upgcxcode/97/86/301428697/301428697-1-30011.m4s?e=ig8euxZM2rNcNbdlhoNvNC8BqJIzNbfqXBvEqxTEto8BTrNvN0GvT90W5JZMkX_YN0MvXg8gNEV4NC8xNEV4N03eN0B5tZlqNxTEto8BTrNvNeZVuJ10Kj_g2UB02J0mN0B5tZlqNCNEto8BTrNvNC7MTX502C8f2jmMQJ6mqF2fka1mqx6gqj0eN0B599M=&uipk=5&nbs=1&deadline=1722621726&gen=playurlv2&os=mcdn&oi=1035876564&trid=0000db23ebfe93334781b190a097b9646038u&mid=0&platform=pc&og=hw&upsig=43409cf50f79c1a58fa6f504b8396ea4&uparams=e,uipk,nbs,deadline,gen,os,oi,trid,mid,platform,og&mcdnid=50004915&bvc=vod&nettype=0&orderid=0,3&buvid=4CB663D6-6208-6C0E-4392-4C7DCEAB3E5E94322infoc&build=0&f=u_0_0&agrr=0&bw=10573&logo=A0020000"],
    audio_url: "https://xy111x225x218x142xy.mcdn.bilivideo.cn:4483/upgcxcode/97/86/301428697/301428697-1-30280.m4s?e=ig8euxZM2rNcNbdlhoNvNC8BqJIzNbfqXBvEqxTEto8BTrNvN0GvT90W5JZMkX_YN0MvXg8gNEV4NC8xNEV4N03eN0B5tZlqNxTEto8BTrNvNeZVuJ10Kj_g2UB02J0mN0B5tZlqNCNEto8BTrNvNC7MTX502C8f2jmMQJ6mqF2fka1mqx6gqj0eN0B599M=&uipk=5&nbs=1&deadline=1722621726&gen=playurlv2&os=mcdn&oi=1035876564&trid=0000db23ebfe93334781b190a097b9646038u&mid=0&platform=pc&og=hw&upsig=0fe2d0a79cadffcf846f4ef0c8eface2&uparams=e,uipk,nbs,deadline,gen,os,oi,trid,mid,platform,og&mcdnid=50004915&bvc=vod&nettype=0&orderid=0,3&buvid=4CB663D6-6208-6C0E-4392-4C7DCEAB3E5E94322infoc&build=0&f=u_0_0&agrr=0&bw=16193&logo=A0020000",
    duration: 4749,
  }, {
    cid: "301428697",
    title: "Lecture 1 - Introduction",
    video_urls: ["https://xy111x225x218x142xy.mcdn.bilivideo.cn:4483/upgcxcode/97/86/301428697/301428697-1-30032.m4s?e=ig8euxZM2rNcNbdlhoNvNC8BqJIzNbfqXBvEqxTEto8BTrNvN0GvT90W5JZMkX_YN0MvXg8gNEV4NC8xNEV4N03eN0B5tZlqNxTEto8BTrNvNeZVuJ10Kj_g2UB02J0mN0B5tZlqNCNEto8BTrNvNC7MTX502C8f2jmMQJ6mqF2fka1mqx6gqj0eN0B599M=&uipk=5&nbs=1&deadline=1722621726&gen=playurlv2&os=mcdn&oi=1035876564&trid=0000db23ebfe93334781b190a097b9646038u&mid=0&platform=pc&og=hw&upsig=d37e6b34f01330c2e3e0126f28868c9b&uparams=e,uipk,nbs,deadline,gen,os,oi,trid,mid,platform,og&mcdnid=50004915&bvc=vod&nettype=0&orderid=0,3&buvid=4CB663D6-6208-6C0E-4392-4C7DCEAB3E5E94322infoc&build=0&f=u_0_0&agrr=0&bw=49880&logo=A0020000", "https://xy111x225x218x142xy.mcdn.bilivideo.cn:4483/upgcxcode/97/86/301428697/301428697-1-30011.m4s?e=ig8euxZM2rNcNbdlhoNvNC8BqJIzNbfqXBvEqxTEto8BTrNvN0GvT90W5JZMkX_YN0MvXg8gNEV4NC8xNEV4N03eN0B5tZlqNxTEto8BTrNvNeZVuJ10Kj_g2UB02J0mN0B5tZlqNCNEto8BTrNvNC7MTX502C8f2jmMQJ6mqF2fka1mqx6gqj0eN0B599M=&uipk=5&nbs=1&deadline=1722621726&gen=playurlv2&os=mcdn&oi=1035876564&trid=0000db23ebfe93334781b190a097b9646038u&mid=0&platform=pc&og=hw&upsig=43409cf50f79c1a58fa6f504b8396ea4&uparams=e,uipk,nbs,deadline,gen,os,oi,trid,mid,platform,og&mcdnid=50004915&bvc=vod&nettype=0&orderid=0,3&buvid=4CB663D6-6208-6C0E-4392-4C7DCEAB3E5E94322infoc&build=0&f=u_0_0&agrr=0&bw=10573&logo=A0020000"],
    audio_url: "https://xy111x225x218x142xy.mcdn.bilivideo.cn:4483/upgcxcode/97/86/301428697/301428697-1-30280.m4s?e=ig8euxZM2rNcNbdlhoNvNC8BqJIzNbfqXBvEqxTEto8BTrNvN0GvT90W5JZMkX_YN0MvXg8gNEV4NC8xNEV4N03eN0B5tZlqNxTEto8BTrNvNeZVuJ10Kj_g2UB02J0mN0B5tZlqNCNEto8BTrNvNC7MTX502C8f2jmMQJ6mqF2fka1mqx6gqj0eN0B599M=&uipk=5&nbs=1&deadline=1722621726&gen=playurlv2&os=mcdn&oi=1035876564&trid=0000db23ebfe93334781b190a097b9646038u&mid=0&platform=pc&og=hw&upsig=0fe2d0a79cadffcf846f4ef0c8eface2&uparams=e,uipk,nbs,deadline,gen,os,oi,trid,mid,platform,og&mcdnid=50004915&bvc=vod&nettype=0&orderid=0,3&buvid=4CB663D6-6208-6C0E-4392-4C7DCEAB3E5E94322infoc&build=0&f=u_0_0&agrr=0&bw=16193&logo=A0020000",
    duration: 4749,
  }, {
    cid: "301428697",
    title: "Lecture 1 - Introduction",
    video_urls: ["https://xy111x225x218x142xy.mcdn.bilivideo.cn:4483/upgcxcode/97/86/301428697/301428697-1-30032.m4s?e=ig8euxZM2rNcNbdlhoNvNC8BqJIzNbfqXBvEqxTEto8BTrNvN0GvT90W5JZMkX_YN0MvXg8gNEV4NC8xNEV4N03eN0B5tZlqNxTEto8BTrNvNeZVuJ10Kj_g2UB02J0mN0B5tZlqNCNEto8BTrNvNC7MTX502C8f2jmMQJ6mqF2fka1mqx6gqj0eN0B599M=&uipk=5&nbs=1&deadline=1722621726&gen=playurlv2&os=mcdn&oi=1035876564&trid=0000db23ebfe93334781b190a097b9646038u&mid=0&platform=pc&og=hw&upsig=d37e6b34f01330c2e3e0126f28868c9b&uparams=e,uipk,nbs,deadline,gen,os,oi,trid,mid,platform,og&mcdnid=50004915&bvc=vod&nettype=0&orderid=0,3&buvid=4CB663D6-6208-6C0E-4392-4C7DCEAB3E5E94322infoc&build=0&f=u_0_0&agrr=0&bw=49880&logo=A0020000", "https://xy111x225x218x142xy.mcdn.bilivideo.cn:4483/upgcxcode/97/86/301428697/301428697-1-30011.m4s?e=ig8euxZM2rNcNbdlhoNvNC8BqJIzNbfqXBvEqxTEto8BTrNvN0GvT90W5JZMkX_YN0MvXg8gNEV4NC8xNEV4N03eN0B5tZlqNxTEto8BTrNvNeZVuJ10Kj_g2UB02J0mN0B5tZlqNCNEto8BTrNvNC7MTX502C8f2jmMQJ6mqF2fka1mqx6gqj0eN0B599M=&uipk=5&nbs=1&deadline=1722621726&gen=playurlv2&os=mcdn&oi=1035876564&trid=0000db23ebfe93334781b190a097b9646038u&mid=0&platform=pc&og=hw&upsig=43409cf50f79c1a58fa6f504b8396ea4&uparams=e,uipk,nbs,deadline,gen,os,oi,trid,mid,platform,og&mcdnid=50004915&bvc=vod&nettype=0&orderid=0,3&buvid=4CB663D6-6208-6C0E-4392-4C7DCEAB3E5E94322infoc&build=0&f=u_0_0&agrr=0&bw=10573&logo=A0020000"],
    audio_url: "https://xy111x225x218x142xy.mcdn.bilivideo.cn:4483/upgcxcode/97/86/301428697/301428697-1-30280.m4s?e=ig8euxZM2rNcNbdlhoNvNC8BqJIzNbfqXBvEqxTEto8BTrNvN0GvT90W5JZMkX_YN0MvXg8gNEV4NC8xNEV4N03eN0B5tZlqNxTEto8BTrNvNeZVuJ10Kj_g2UB02J0mN0B5tZlqNCNEto8BTrNvNC7MTX502C8f2jmMQJ6mqF2fka1mqx6gqj0eN0B599M=&uipk=5&nbs=1&deadline=1722621726&gen=playurlv2&os=mcdn&oi=1035876564&trid=0000db23ebfe93334781b190a097b9646038u&mid=0&platform=pc&og=hw&upsig=0fe2d0a79cadffcf846f4ef0c8eface2&uparams=e,uipk,nbs,deadline,gen,os,oi,trid,mid,platform,og&mcdnid=50004915&bvc=vod&nettype=0&orderid=0,3&buvid=4CB663D6-6208-6C0E-4392-4C7DCEAB3E5E94322infoc&build=0&f=u_0_0&agrr=0&bw=16193&logo=A0020000",
    duration: 4749,
  }, {
    cid: "301428697",
    title: "Lecture 1 - Introduction",
    video_urls: ["https://xy111x225x218x142xy.mcdn.bilivideo.cn:4483/upgcxcode/97/86/301428697/301428697-1-30032.m4s?e=ig8euxZM2rNcNbdlhoNvNC8BqJIzNbfqXBvEqxTEto8BTrNvN0GvT90W5JZMkX_YN0MvXg8gNEV4NC8xNEV4N03eN0B5tZlqNxTEto8BTrNvNeZVuJ10Kj_g2UB02J0mN0B5tZlqNCNEto8BTrNvNC7MTX502C8f2jmMQJ6mqF2fka1mqx6gqj0eN0B599M=&uipk=5&nbs=1&deadline=1722621726&gen=playurlv2&os=mcdn&oi=1035876564&trid=0000db23ebfe93334781b190a097b9646038u&mid=0&platform=pc&og=hw&upsig=d37e6b34f01330c2e3e0126f28868c9b&uparams=e,uipk,nbs,deadline,gen,os,oi,trid,mid,platform,og&mcdnid=50004915&bvc=vod&nettype=0&orderid=0,3&buvid=4CB663D6-6208-6C0E-4392-4C7DCEAB3E5E94322infoc&build=0&f=u_0_0&agrr=0&bw=49880&logo=A0020000", "https://xy111x225x218x142xy.mcdn.bilivideo.cn:4483/upgcxcode/97/86/301428697/301428697-1-30011.m4s?e=ig8euxZM2rNcNbdlhoNvNC8BqJIzNbfqXBvEqxTEto8BTrNvN0GvT90W5JZMkX_YN0MvXg8gNEV4NC8xNEV4N03eN0B5tZlqNxTEto8BTrNvNeZVuJ10Kj_g2UB02J0mN0B5tZlqNCNEto8BTrNvNC7MTX502C8f2jmMQJ6mqF2fka1mqx6gqj0eN0B599M=&uipk=5&nbs=1&deadline=1722621726&gen=playurlv2&os=mcdn&oi=1035876564&trid=0000db23ebfe93334781b190a097b9646038u&mid=0&platform=pc&og=hw&upsig=43409cf50f79c1a58fa6f504b8396ea4&uparams=e,uipk,nbs,deadline,gen,os,oi,trid,mid,platform,og&mcdnid=50004915&bvc=vod&nettype=0&orderid=0,3&buvid=4CB663D6-6208-6C0E-4392-4C7DCEAB3E5E94322infoc&build=0&f=u_0_0&agrr=0&bw=10573&logo=A0020000"],
    audio_url: "https://xy111x225x218x142xy.mcdn.bilivideo.cn:4483/upgcxcode/97/86/301428697/301428697-1-30280.m4s?e=ig8euxZM2rNcNbdlhoNvNC8BqJIzNbfqXBvEqxTEto8BTrNvN0GvT90W5JZMkX_YN0MvXg8gNEV4NC8xNEV4N03eN0B5tZlqNxTEto8BTrNvNeZVuJ10Kj_g2UB02J0mN0B5tZlqNCNEto8BTrNvNC7MTX502C8f2jmMQJ6mqF2fka1mqx6gqj0eN0B599M=&uipk=5&nbs=1&deadline=1722621726&gen=playurlv2&os=mcdn&oi=1035876564&trid=0000db23ebfe93334781b190a097b9646038u&mid=0&platform=pc&og=hw&upsig=0fe2d0a79cadffcf846f4ef0c8eface2&uparams=e,uipk,nbs,deadline,gen,os,oi,trid,mid,platform,og&mcdnid=50004915&bvc=vod&nettype=0&orderid=0,3&buvid=4CB663D6-6208-6C0E-4392-4C7DCEAB3E5E94322infoc&build=0&f=u_0_0&agrr=0&bw=16193&logo=A0020000",
    duration: 4749,
  }, {
    cid: "301428697",
    title: "Lecture 1 - Introduction",
    video_urls: ["https://xy111x225x218x142xy.mcdn.bilivideo.cn:4483/upgcxcode/97/86/301428697/301428697-1-30032.m4s?e=ig8euxZM2rNcNbdlhoNvNC8BqJIzNbfqXBvEqxTEto8BTrNvN0GvT90W5JZMkX_YN0MvXg8gNEV4NC8xNEV4N03eN0B5tZlqNxTEto8BTrNvNeZVuJ10Kj_g2UB02J0mN0B5tZlqNCNEto8BTrNvNC7MTX502C8f2jmMQJ6mqF2fka1mqx6gqj0eN0B599M=&uipk=5&nbs=1&deadline=1722621726&gen=playurlv2&os=mcdn&oi=1035876564&trid=0000db23ebfe93334781b190a097b9646038u&mid=0&platform=pc&og=hw&upsig=d37e6b34f01330c2e3e0126f28868c9b&uparams=e,uipk,nbs,deadline,gen,os,oi,trid,mid,platform,og&mcdnid=50004915&bvc=vod&nettype=0&orderid=0,3&buvid=4CB663D6-6208-6C0E-4392-4C7DCEAB3E5E94322infoc&build=0&f=u_0_0&agrr=0&bw=49880&logo=A0020000", "https://xy111x225x218x142xy.mcdn.bilivideo.cn:4483/upgcxcode/97/86/301428697/301428697-1-30011.m4s?e=ig8euxZM2rNcNbdlhoNvNC8BqJIzNbfqXBvEqxTEto8BTrNvN0GvT90W5JZMkX_YN0MvXg8gNEV4NC8xNEV4N03eN0B5tZlqNxTEto8BTrNvNeZVuJ10Kj_g2UB02J0mN0B5tZlqNCNEto8BTrNvNC7MTX502C8f2jmMQJ6mqF2fka1mqx6gqj0eN0B599M=&uipk=5&nbs=1&deadline=1722621726&gen=playurlv2&os=mcdn&oi=1035876564&trid=0000db23ebfe93334781b190a097b9646038u&mid=0&platform=pc&og=hw&upsig=43409cf50f79c1a58fa6f504b8396ea4&uparams=e,uipk,nbs,deadline,gen,os,oi,trid,mid,platform,og&mcdnid=50004915&bvc=vod&nettype=0&orderid=0,3&buvid=4CB663D6-6208-6C0E-4392-4C7DCEAB3E5E94322infoc&build=0&f=u_0_0&agrr=0&bw=10573&logo=A0020000"],
    audio_url: "https://xy111x225x218x142xy.mcdn.bilivideo.cn:4483/upgcxcode/97/86/301428697/301428697-1-30280.m4s?e=ig8euxZM2rNcNbdlhoNvNC8BqJIzNbfqXBvEqxTEto8BTrNvN0GvT90W5JZMkX_YN0MvXg8gNEV4NC8xNEV4N03eN0B5tZlqNxTEto8BTrNvNeZVuJ10Kj_g2UB02J0mN0B5tZlqNCNEto8BTrNvNC7MTX502C8f2jmMQJ6mqF2fka1mqx6gqj0eN0B599M=&uipk=5&nbs=1&deadline=1722621726&gen=playurlv2&os=mcdn&oi=1035876564&trid=0000db23ebfe93334781b190a097b9646038u&mid=0&platform=pc&og=hw&upsig=0fe2d0a79cadffcf846f4ef0c8eface2&uparams=e,uipk,nbs,deadline,gen,os,oi,trid,mid,platform,og&mcdnid=50004915&bvc=vod&nettype=0&orderid=0,3&buvid=4CB663D6-6208-6C0E-4392-4C7DCEAB3E5E94322infoc&build=0&f=u_0_0&agrr=0&bw=16193&logo=A0020000",
    duration: 4749,
  }, {
    cid: "301428697",
    title: "Lecture 1 - Introduction",
    video_urls: ["https://xy111x225x218x142xy.mcdn.bilivideo.cn:4483/upgcxcode/97/86/301428697/301428697-1-30032.m4s?e=ig8euxZM2rNcNbdlhoNvNC8BqJIzNbfqXBvEqxTEto8BTrNvN0GvT90W5JZMkX_YN0MvXg8gNEV4NC8xNEV4N03eN0B5tZlqNxTEto8BTrNvNeZVuJ10Kj_g2UB02J0mN0B5tZlqNCNEto8BTrNvNC7MTX502C8f2jmMQJ6mqF2fka1mqx6gqj0eN0B599M=&uipk=5&nbs=1&deadline=1722621726&gen=playurlv2&os=mcdn&oi=1035876564&trid=0000db23ebfe93334781b190a097b9646038u&mid=0&platform=pc&og=hw&upsig=d37e6b34f01330c2e3e0126f28868c9b&uparams=e,uipk,nbs,deadline,gen,os,oi,trid,mid,platform,og&mcdnid=50004915&bvc=vod&nettype=0&orderid=0,3&buvid=4CB663D6-6208-6C0E-4392-4C7DCEAB3E5E94322infoc&build=0&f=u_0_0&agrr=0&bw=49880&logo=A0020000", "https://xy111x225x218x142xy.mcdn.bilivideo.cn:4483/upgcxcode/97/86/301428697/301428697-1-30011.m4s?e=ig8euxZM2rNcNbdlhoNvNC8BqJIzNbfqXBvEqxTEto8BTrNvN0GvT90W5JZMkX_YN0MvXg8gNEV4NC8xNEV4N03eN0B5tZlqNxTEto8BTrNvNeZVuJ10Kj_g2UB02J0mN0B5tZlqNCNEto8BTrNvNC7MTX502C8f2jmMQJ6mqF2fka1mqx6gqj0eN0B599M=&uipk=5&nbs=1&deadline=1722621726&gen=playurlv2&os=mcdn&oi=1035876564&trid=0000db23ebfe93334781b190a097b9646038u&mid=0&platform=pc&og=hw&upsig=43409cf50f79c1a58fa6f504b8396ea4&uparams=e,uipk,nbs,deadline,gen,os,oi,trid,mid,platform,og&mcdnid=50004915&bvc=vod&nettype=0&orderid=0,3&buvid=4CB663D6-6208-6C0E-4392-4C7DCEAB3E5E94322infoc&build=0&f=u_0_0&agrr=0&bw=10573&logo=A0020000"],
    audio_url: "https://xy111x225x218x142xy.mcdn.bilivideo.cn:4483/upgcxcode/97/86/301428697/301428697-1-30280.m4s?e=ig8euxZM2rNcNbdlhoNvNC8BqJIzNbfqXBvEqxTEto8BTrNvN0GvT90W5JZMkX_YN0MvXg8gNEV4NC8xNEV4N03eN0B5tZlqNxTEto8BTrNvNeZVuJ10Kj_g2UB02J0mN0B5tZlqNCNEto8BTrNvNC7MTX502C8f2jmMQJ6mqF2fka1mqx6gqj0eN0B599M=&uipk=5&nbs=1&deadline=1722621726&gen=playurlv2&os=mcdn&oi=1035876564&trid=0000db23ebfe93334781b190a097b9646038u&mid=0&platform=pc&og=hw&upsig=0fe2d0a79cadffcf846f4ef0c8eface2&uparams=e,uipk,nbs,deadline,gen,os,oi,trid,mid,platform,og&mcdnid=50004915&bvc=vod&nettype=0&orderid=0,3&buvid=4CB663D6-6208-6C0E-4392-4C7DCEAB3E5E94322infoc&build=0&f=u_0_0&agrr=0&bw=16193&logo=A0020000",
    duration: 4749,
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
  value.value = animeInfo.value.formats[0];
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
        <el-page-header :icon="ArrowLeft" @back="goBack" title="返回" style="border-bottom: 2px solid var(--el-color-primary-light-7);
  padding-bottom: 5px">
          <template #content>
            <el-button>下载</el-button>
          </template>
        </el-page-header>
      </el-header>
      <el-main>
        <div class="content">
          <div class="left">
            <span
                style="margin: 5px 0 5px 0; font-size: 16px; height: 25px; font-weight: bold; color: var(--el-color-info-dark-2)">{{
                animeInfo.title
              }}</span>
            <el-image
                fit="cover"
                style="width: 220px; height: 300px; border-radius: 10px"
                :src="animeInfo.cover"
            >
              <template #error>
                <div class="image-slot">
                  <el-icon>
                    <icon-picture/>
                  </el-icon>
                </div>
              </template>
            </el-image>
            <div style="display: flex; align-items: center; margin: 5px 0 5px 0; font-size: small">
              <span>{{ animeInfo.date }}</span>
              <el-divider direction="vertical" border-style="none"/>
              <span style="font-size: small; margin-left: 10px">{{
                  animeInfo.play.substring(0, animeInfo.play.indexOf("弹幕") + 2)
                }}</span>
            </div>
          </div>
          <div class="right">
            <span
                style="margin-bottom: 30px;
                background-color: var(--el-color-info-light-9);
                border-radius: 15px; padding: 5px; height: 45%"
            >
              {{ animeInfo.description }}
            </span>
            <div style="display: flex; justify-content: space-between; margin-bottom: 30px; background-color: var(--el-color-primary-light-9); border-radius: 10px">
              <el-checkbox-group v-model="checkboxGroup1" style="height: 100px; overflow-y: scroll;">
                <el-checkbox-button style="margin: 5px" v-for="(episode, index) in animeInfo.episodes" :key="index"
                                    :value="index">
                  {{ index + 1 }}
                </el-checkbox-button>
              </el-checkbox-group>
            </div>
            <el-segmented v-model="value" :options="options"/>
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
  width: 40%;
  height: 100%;
  display: flex;
  flex-direction: column;
  align-items: center;
  margin-right: 30px;
  justify-content: space-between;
  background-color: var(--el-color-info-light-9);
  border-radius: 15px;
}

.right {
  width: 55%;
  height: 100%;
  display: flex;
  flex-direction: column;
  justify-content: space-between;
}

.content {
  display: flex;
  width: 100%;
  height: 80%;
  justify-content: space-between;
  margin: 0 auto;
}

.download-video {
  height: 100%;
  width: 100%;
}
</style>