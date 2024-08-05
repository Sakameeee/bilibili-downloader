<script setup lang="ts">
import {onMounted, ref} from "vue";
import {ArrowLeft} from "@element-plus/icons-vue";
import Divider from "../components/Divider.vue";
import {useRoute} from "vue-router";
import {Video} from "../types";

const loading = ref(false);
const value = ref();
const radio = ref(3);
const checkboxGroup1 = ref([0])
const route = useRoute();
const videoInfo = ref<Video>(<Video>{
  duration: 0,
  bvid: "BV16f4y1z7kn",
  cid: "301428697",
  title: "MIT 6.824 2021 分布式系统 [中英文字幕]_哔哩哔哩_bilibili",
  description: "是MIT 6.824 2021 分布式系统 [中英文字幕]的第NaN集视频，该合集共计22集，视频收藏或关注UP主，及时了解更多相关视频内容。",
  cover: "http://i1.hdslb.com/bfs/archive/f9a6c834d4dff5d349d451900ec9724302b36135.jpg",
  author: "mayf09",
  author_avatar: "https://i0.hdslb.com/bfs/face/341af0339a1eee3e4b518f03de2ae5481d7dc474.jpg",
  count: 22,
  danmaku: "269",
  play: "52834",
  formats: ["高清 1080P", "高清 720P", "清晰 480P", "流畅 360P"],
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
  options.value = videoInfo.value.formats;
  value.value = videoInfo.value.formats[0];
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
                videoInfo.title
              }}</span>
            <el-image
                fit="cover"
                style="width: 300px; height: 170px"
                :src="videoInfo.cover">
              <template #error>
                <div class="image-slot">
                  <el-icon>
                    <icon-picture/>
                  </el-icon>
                </div>
              </template>
            </el-image>
            <div style="display: flex; align-items: center; margin: 5px 0 5px 0; font-size: small">
              <span style="font-size: small; margin-left: 10px">{{
                  videoInfo.description
                }}</span>
            </div>
          </div>
          <div class="right">
            <div style="display: flex; align-items: center; margin-bottom: 20px; background-color: var(--el-color-info-light-9); padding: 10px; border-radius: 30px">
              <el-avatar :size="40" src="https://empty">
                <img
                    :src="videoInfo.author_avatar"
                />
              </el-avatar>
              <el-divider direction="vertical" border-style="none"/>
              <span>{{ videoInfo.author }}</span>
              <span style="font-size: small; margin-left: 10px">播放数：{{ videoInfo.play }}</span>
              <span style="font-size: small; margin-left: 10px">弹幕：{{ videoInfo.danmaku }}</span>
            </div>
            <div style="display: flex; justify-content: space-between; margin-bottom: 30px; background-color: var(--el-color-primary-light-9); border-radius: 10px; padding: 5px">
              <el-checkbox-group v-model="checkboxGroup1" style="width: 200px; height: 100px; overflow-y: scroll;">
                <el-checkbox-button style="margin: 5px" v-for="(episode, index) in videoInfo.episodes" :key="index"
                                    :value="index">
                  {{ index + 1 }}
                </el-checkbox-button>
              </el-checkbox-group>
              <el-radio-group v-model="radio" size="small" style="width: 250px">
                <el-radio :value="3" border style="width: 120px">下载音视频</el-radio>
                <divider/>
                <el-radio :value="6" border style="width: 120px">下载视频</el-radio>
                <divider/>
                <el-radio :value="9" border style="width: 120px">下载音频</el-radio>
              </el-radio-group>
            </div>
            <el-segmented v-model="value" :options="options"/>
          </div>
        </div>
      </el-main>
    </el-container>
  </div>
</template>

<style scoped>
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