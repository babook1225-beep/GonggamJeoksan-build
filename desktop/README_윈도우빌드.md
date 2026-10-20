# 공감 적산 Windows 프로그램(exe) 빌드 — Codex 작업 지시서

이 폴더는 오프라인 HTML(공감적산_오프라인_v0.5.html)을 Tauri v2로 포장한 Windows 데스크톱 프로젝트다.
화면·계산은 HTML 그대로이고, 데스크톱에서만 다음이 추가된다(`offline-src/desktop.js`).
- 작업 파일·엑셀·CSV 저장 시 Windows '다른 이름으로 저장' 창 → 지정한 경로에 실제 저장(임시 파일에 쓴 뒤 교체)
- 작업 파일을 저장/열면 그 파일에 2초 간격으로 자동 저장(구버전 파일을 열었을 때는 원본 보호를 위해 자동 저장 안 함)
- 엑셀 저장 후 바로 열기

## 0. 작업 위치
압축을 푼 `공감적산_v0.5_인계` 폴더에서 PowerShell을 연다. 이하 명령은 `desktop` 폴더 기준.

## 1. 처음 한 번: 개발 도구 설치 (관리자 권한 PowerShell, 20~30분)
```powershell
winget install --id Rustlang.Rustup -e --accept-package-agreements --accept-source-agreements
winget install --id Microsoft.VisualStudio.2022.BuildTools -e --override "--quiet --wait --add Microsoft.VisualStudio.Workload.VCTools --includeRecommended"
winget install --id OpenJS.NodeJS.LTS -e
```
설치 후 PowerShell을 새로 연다. 확인:
```powershell
rustc --version   # 1.80 이상
cargo --version
node --version    # 20 이상
```
WebView2는 Windows 10/11에 기본 설치되어 있다(없으면 설치 파일이 자동으로 받아 설치).

## 2. 빌드
```powershell
cd desktop
npm install
npm run build
```
`npm run build` = `scripts/prepare-dist.mjs`(HTML → dist/index.html, CSP에 Tauri 내부 통신만 허용) + `tauri build`.
처음 빌드는 러스트 라이브러리를 받아 컴파일하느라 5~15분 걸린다.

결과물:
- 설치 파일: `desktop\src-tauri\target\release\bundle\nsis\GonggamJeoksan_0.5.0_x64-setup.exe` (사용자 계정 설치, 관리자 권한 불필요)
- 단독 실행 파일: `desktop\src-tauri\target\release\GonggamJeoksan.exe` (WebView2가 있는 PC에서 바로 실행)

## 3. 확인 (exe 실행 후 직접 클릭)
1. 창 제목 '공감 적산', 내역서 화면이 v0.5 HTML과 같게 보이는지.
2. '작업 파일 저장' → 저장 창 → 바탕화면에 저장 → 창 제목에 파일 이름, 오른쪽 위 '자동 저장: …'.
3. 수량 하나 변경 → 2~3초 뒤 '자동 저장됨' → 프로그램 종료 후 다시 실행 → '작업 열기'로 그 파일 → 바뀐 수량 유지.
4. '엑셀(XLSX) 내보내기' → 저장 → '지금 엑셀로 열까요?' 확인 → 엑셀이 열리고 8개 시트.
5. '단가 가져오기'에서 upload\2026_상반_충청남도_교육청(건축).estx 선택 → '신규 0개 · 3,166개 재사용'.
6. 인터넷을 끈 상태에서도 위가 모두 되는지.

## 4. 문제가 생기면
- `link.exe not found` / `MSVC` 오류: 1단계 Build Tools(C++ 워크로드) 설치 확인 후 PowerShell 재시작.
- 실행 시 'Windows의 PC 보호' 경고: 코드 서명이 없어서 나오는 정상 경고. '추가 정보 → 실행'.
- 저장 창이 안 뜨거나 아무 반응이 없으면: 개발용으로 `npm run dev` 실행 후 창에서 F12(개발자 도구) 콘솔 오류를 그대로 보고.
- 빌드 로그·오류 메시지·확인 결과(1~6번)를 그대로 보고한다. 소스(offline-src, src-tauri)는 임의로 고치지 말고, 고쳐야 하면 무엇을 왜 고쳤는지 diff와 함께 보고한다.

## 구성
| 파일 | 역할 |
|---|---|
| `package.json` | Tauri CLI 2.12.3, `npm run build` |
| `scripts/prepare-dist.mjs` | 최신 outputs HTML → `dist/index.html` (CSP connect-src만 `ipc: http://ipc.localhost`로) |
| `src-tauri/tauri.conf.json` | 창 1440×900(최소 1024×680), NSIS 설치(한국어, 사용자 계정), WebView2 부트스트래퍼 포함 |
| `src-tauri/src/lib.rs` | 명령 `write_file`(임시 파일 후 교체), `read_file`, `open_path` + dialog·opener 플러그인 |
| `src-tauri/capabilities/default.json` | core·dialog·opener 기본 권한 |
| `src-tauri/icons/` | 앱 아이콘(`app-icon.png`에서 `npx tauri icon`으로 생성) |
