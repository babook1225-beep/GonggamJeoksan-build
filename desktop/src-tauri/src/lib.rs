//! 공감 적산 데스크톱 포장. 화면은 오프라인 단일 HTML 그대로 쓰고,
//! 브라우저가 못 하는 일(지정한 경로에 저장·열기·만든 파일 열기)만 아래 명령으로 제공한다.
use base64::{engine::general_purpose::STANDARD, Engine};
use std::path::PathBuf;
use tauri_plugin_opener::OpenerExt;

/// 파일 저장. 같은 폴더의 임시 파일에 먼저 쓴 뒤 바꿔치기해서, 저장 도중 꺼져도 기존 파일이 깨지지 않게 한다.
#[tauri::command]
fn write_file(path: String, data: String) -> Result<(), String> {
    let bytes = STANDARD.decode(data).map_err(|e| format!("데이터 형식 오류: {e}"))?;
    let target = PathBuf::from(&path);
    let mut tmp = target.clone().into_os_string();
    tmp.push(".gonggam-tmp");
    let tmp = PathBuf::from(tmp);
    std::fs::write(&tmp, &bytes).map_err(|e| format!("저장 실패({path}): {e}"))?;
    std::fs::rename(&tmp, &target).map_err(|e| {
        let _ = std::fs::remove_file(&tmp);
        format!("저장 실패({path}): {e}")
    })
}

/// 파일 읽기(base64로 돌려준다).
#[tauri::command]
fn read_file(path: String) -> Result<String, String> {
    std::fs::read(&path)
        .map(|b| STANDARD.encode(b))
        .map_err(|e| format!("열기 실패({path}): {e}"))
}

/// 만든 파일을 기본 프로그램(엑셀 등)으로 연다.
#[tauri::command]
fn open_path(app: tauri::AppHandle, path: String) -> Result<(), String> {
    app.opener()
        .open_path(path, None::<&str>)
        .map_err(|e| e.to_string())
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .plugin(tauri_plugin_dialog::init())
        .plugin(tauri_plugin_opener::init())
        .invoke_handler(tauri::generate_handler![write_file, read_file, open_path])
        .run(tauri::generate_context!())
        .expect("공감 적산을 시작하지 못했습니다");
}
