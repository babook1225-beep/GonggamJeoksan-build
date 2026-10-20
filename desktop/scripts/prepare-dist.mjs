// 단일 HTML 실행 파일을 Tauri 화면(dist/index.html)으로 복사한다.
// 바꾸는 것은 CSP의 connect-src 하나: Tauri 내부 통신(ipc:, http://ipc.localhost)만 허용. 외부 네트워크는 계속 차단.
import fs from 'node:fs';import path from 'node:path';import {fileURLToPath} from 'node:url';
const here=path.dirname(fileURLToPath(import.meta.url)),root=path.resolve(here,'..','..');
const dir=path.join(root,'outputs','gonggam-offline');
const src=fs.readdirSync(dir).filter(f=>/^공감적산_오프라인_v[\d.]+\.html$/.test(f)).sort((a,b)=>a.localeCompare(b,undefined,{numeric:true})).pop();
if(!src)throw Error('outputs/gonggam-offline에 HTML이 없습니다. 먼저 python offline-src/build.py 를 실행하세요.');
let html=fs.readFileSync(path.join(dir,src),'utf8');
const before="connect-src 'none'";if(!html.includes(before))throw Error('CSP connect-src 항목을 찾지 못했습니다.');
html=html.replace(before,"connect-src ipc: http://ipc.localhost");
fs.mkdirSync(path.join(here,'..','dist'),{recursive:true});fs.writeFileSync(path.join(here,'..','dist','index.html'),html);
console.log('dist/index.html ←',src,(html.length/1e6).toFixed(2)+'MB');
