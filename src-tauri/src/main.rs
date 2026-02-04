#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use std::fs;
use std::process::Command;
use std::path::PathBuf;
use tauri::command;
use serde::Serialize;
use regex::Regex;

#[command]
fn compile_latex(latex_code: String) -> Result<Vec<u8>, Vec<CompileError>> {
    println!("Frontend requested compilation...");

    // 1. 确定临时文件路径
    // 在 macOS 上通常是 /tmp/
    let mut temp_dir = std::env::temp_dir();
    temp_dir.push("tauri_latex_build");

    // 确保目录存在
    if !temp_dir.exists() {
        fs::create_dir(&temp_dir).map_err(|e| {
            vec![CompileError {
                line: 0,
                message: e.to_string(),
                severity: "error".to_string(),
            }]
        })?;
    }

    let tex_file_path = temp_dir.join("input.tex");
    let pdf_file_path = temp_dir.join("input.pdf");

    // 2. 将前端传来的代码写入 input.tex
    fs::write(&tex_file_path, &latex_code).map_err(|e| {
        vec![CompileError {
            line: 0,
            message: format!("无法写入文件: {}", e),
            severity: "error".to_string(),
        }]
    })?;

    // 3. 调用系统安装的 `tectonic` 命令
    // 相当于在终端执行: tectonic input.tex
    let output = Command::new("tectonic")
        .arg(&tex_file_path)
        .current_dir(&temp_dir) // 在临时目录执行
        .output()
        .map_err(|e| {
            vec![CompileError {
                line: 0,
                message: format!("无法调用 Tectonic 命令: {}. 请确保你运行了 'brew install tectonic'", e),
                severity: "error".to_string(),
            }]
        })?;

    if !output.status.success() {
        let stdout = String::from_utf8_lossy(&output.stdout);
        let stderr = String::from_utf8_lossy(&output.stderr);
        let log = format!("{}\n{}", stdout, stderr);
        let msg_re = Regex::new(r"^error:\s*(.*)$").unwrap();
        let line_re = Regex::new(r"^l\.(\d+)").unwrap();
        let mut current_message: Option<String> = None;
        let mut errors = Vec::new();

        for line in log.lines() {
            let trimmed = line.trim();
            if let Some(caps) = msg_re.captures(trimmed) {
                current_message = Some(caps[1].trim().to_string());
                continue;
            }
            if let Some(caps) = line_re.captures(trimmed) {
                let line_number = caps
                    .get(1)
                    .and_then(|value| value.as_str().parse::<u32>().ok())
                    .unwrap_or(0);
                let message = current_message.take().unwrap_or_else(|| "Compilation error".to_string());
                errors.push(CompileError {
                    line: line_number,
                    message,
                    severity: "error".to_string(),
                });
            }
        }

        if errors.is_empty() {
            errors.push(CompileError {
                line: 0,
                message: log.trim().to_string(),
                severity: "error".to_string(),
            });
        }

        return Err(errors);
    }

    // 4. 读取生成的 PDF 文件
    if pdf_file_path.exists() {
        let pdf_data = fs::read(&pdf_file_path).map_err(|e| {
            vec![CompileError {
                line: 0,
                message: e.to_string(),
                severity: "error".to_string(),
            }]
        })?;
        println!("Success! PDF size: {} bytes", pdf_data.len());
        Ok(pdf_data)
    } else {
        Err(vec![CompileError {
            line: 0,
            message: "编译看似成功，但未找到生成的 PDF 文件".to_string(),
            severity: "error".to_string(),
        }])
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

#[derive(Serialize)]
struct CompileError {
    line: u32,
    message: String,
    severity: String,
}

#[command]
fn list_files(root_path: String) -> Result<Vec<FileEntry>, String> {
    let root = PathBuf::from(root_path);
    let mut entries = Vec::new();

    let read_dir = fs::read_dir(&root).map_err(|e| format!("无法读取目录: {}", e))?;

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
    }

    entries.sort_by(|a, b| {
        if a.is_dir == b.is_dir {
            a.name.cmp(&b.name)
        } else {
            if a.is_dir { std::cmp::Ordering::Less } else { std::cmp::Ordering::Greater }
        }
    });

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
