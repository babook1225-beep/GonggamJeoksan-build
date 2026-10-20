# 공감 적산 — Windows exe 빌드 전용

오프라인 적산 프로그램 '공감 적산'(단일 HTML)을 Tauri로 포장해 Windows exe를 만드는 빌드 전용 저장소입니다.
원본 소스·문서는 비공개 저장소에 있고, 여기에는 빌드에 필요한 것만 있습니다.

- `outputs/gonggam-offline/` — 빌드된 단일 HTML
- `desktop/` — Tauri v2 프로젝트
- `.github/workflows/windows-exe.yml` — main에 올리면 Windows 설치 파일을 빌드해 Actions 'Artifacts'에 올림. `v*` 태그면 Releases에도 첨부.
