#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use std::fs;
use std::process::Command;
use std::path::PathBuf;
use tauri::command;
use serde::Serialize;

#[command]
fn compile_latex(latex_code: String) -> Result<Vec<u8>, String> {
    println!("Frontend requested compilation...");

    // 1. 确定临时文件路径
    // 在 macOS 上通常是 /tmp/
    let mut temp_dir = std::env::temp_dir();
    temp_dir.push("tauri_latex_build");

    // 确保目录存在
    if !temp_dir.exists() {
        fs::create_dir(&temp_dir).map_err(|e| e.to_string())?;
    }

    let tex_file_path = temp_dir.join("input.tex");
    let pdf_file_path = temp_dir.join("input.pdf");

    // 2. 将前端传来的代码写入 input.tex
    fs::write(&tex_file_path, &latex_code).map_err(|e| format!("无法写入文件: {}", e))?;

    // 3. 调用系统安装的 `tectonic` 命令
    // 相当于在终端执行: tectonic input.tex
    let output = Command::new("tectonic")
        .arg(&tex_file_path)
        .current_dir(&temp_dir) // 在临时目录执行
        .output()
        .map_err(|e| format!("无法调用 Tectonic 命令: {}. 请确保你运行了 'brew install tectonic'", e))?;

    if !output.status.success() {
        // 如果编译失败，返回错误日志
        let stderr = String::from_utf8_lossy(&output.stderr);
        return Err(format!("编译错误:\n{}", stderr));
    }

    // 4. 读取生成的 PDF 文件
    if pdf_file_path.exists() {
        let pdf_data = fs::read(&pdf_file_path).map_err(|e| e.to_string())?;
        println!("Success! PDF size: {} bytes", pdf_data.len());
        Ok(pdf_data)
    } else {
        Err("编译看似成功，但未找到生成的 PDF 文件".to_string())
    }
}

#[command]
fn save_file(path: String, content: String) -> Result<(), String> {
    fs::write(&path, content).map_err(|e| format!("无法写入文件: {}", e))
}

#[command]
fn read_file(path: String) -> Result<String, String> {
    fs::read_to_string(&path).map_err(|e| format!("无法读取文件: {}", e))
}

#[derive(Serialize)]
struct FileEntry {
    name: String,
    path: String,
    is_dir: bool,
}

#[command]
fn list_files(root_path: String) -> Result<Vec<FileEntry>, String> {
    fn walk_dir(path: &PathBuf, entries: &mut Vec<FileEntry>) -> Result<(), String> {
        let read_dir = fs::read_dir(path).map_err(|e| format!("无法读取目录: {}", e))?;
        for entry in read_dir {
            let entry = entry.map_err(|e| format!("无法读取目录项: {}", e))?;
            let entry_path = entry.path();
            let is_dir = entry_path.is_dir();
            let name = entry
                .file_name()
                .to_string_lossy()
                .to_string();
            entries.push(FileEntry {
                name,
                path: entry_path.to_string_lossy().to_string(),
                is_dir,
            });
            if is_dir {
                walk_dir(&entry_path, entries)?;
            }
        }
        Ok(())
    }

    let root = PathBuf::from(root_path);
    let mut entries = Vec::new();
    walk_dir(&root, &mut entries)?;
    Ok(entries)
}

fn main() {
    tauri::Builder::default()
        .plugin(tauri_plugin_dialog::init())
        .invoke_handler(tauri::generate_handler![
            compile_latex,
            save_file,
            read_file,
            list_files
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
