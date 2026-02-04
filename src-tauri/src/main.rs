#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use std::fs;
use std::process::Command;
use tauri::command;
use serde::Serialize;
use regex::Regex;
use std::path::{Path, PathBuf};

#[command]
fn compile_latex(latex_code: String, file_path: Option<String>) -> Result<Vec<u8>, Vec<CompileError>> {
    println!("Frontend requested compilation...");

    // 情况 A: 未保存的新文件 (Untitled)
    // 保持原有逻辑：使用系统临时目录，文件名为 input.tex
    if file_path.is_none() {
        let mut temp_dir = std::env::temp_dir();
        temp_dir.push("tauri_latex_build");
        if !temp_dir.exists() {
            fs::create_dir(&temp_dir).map_err(|e| vec![CompileError::sys(e)])?;
        }
        let tex_file_path = temp_dir.join("input.tex");
        let pdf_file_path = temp_dir.join("input.pdf");

        fs::write(&tex_file_path, &latex_code).map_err(|e| vec![CompileError::sys(e)])?;

        let output = Command::new("tectonic")
            .arg(&tex_file_path)
            .current_dir(&temp_dir)
            .output()
            .map_err(|e| vec![CompileError::sys(e)])?;

        return handle_compilation_result(output, pdf_file_path);
    }

    // 情况 B: 已存在的本地文件
    let path_str = file_path.unwrap();
    let source_path = Path::new(&path_str);
    let parent_dir = source_path.parent().unwrap_or(Path::new("."));

    // 1. 获取文件名 (如 "main.tex" -> stem 是 "main")
    let file_stem = source_path.file_stem()
        .ok_or_else(|| vec![CompileError::simple("无法获取文件名")])?
        .to_string_lossy();

    // 2. 创建 AuxiliaryFiles 目录
    let aux_dir = parent_dir.join("AuxiliaryFiles");
    if !aux_dir.exists() {
        fs::create_dir_all(&aux_dir).map_err(|e| vec![CompileError::sys(e)])?;
    }

    // 3. 【关键】保存当前编辑器内容到源文件
    // Tectonic 需要读取磁盘上的文件，所以我们必须先保存
    fs::write(source_path, &latex_code).map_err(|e| vec![CompileError::sys(e)])?;

    // 4. 执行编译
    // 运行命令：tectonic -o <AuxDir> --keep-intermediates --synctex <SourceFile>
    // 注意：源文件不在 AuxDir 里，而在父目录。Tectonic 会自动处理。
    println!("Compiling {:?} to output dir {:?}", source_path, aux_dir);

    let output = Command::new("tectonic")
        .arg("-o")
        .arg(&aux_dir)
        .arg("--keep-intermediates") // 保留中间文件
        .arg("--synctex")            // 生成 synctex
        .arg(source_path)            // 输入文件
        .output()
        .map_err(|e| vec![CompileError::sys(e)])?;

    // 5. 结果处理
    // PDF 会生成在 aux_dir 下，名字是 <file_stem>.pdf
    let pdf_filename = format!("{}.pdf", file_stem);
    let pdf_file_path = aux_dir.join(&pdf_filename);

    handle_compilation_result(output, pdf_file_path)
}

// 辅助函数：统一处理 Tectonic 输出和错误解析
fn handle_compilation_result(output: std::process::Output, pdf_path: PathBuf) -> Result<Vec<u8>, Vec<CompileError>> {
    if !output.status.success() {
        let stdout = String::from_utf8_lossy(&output.stdout);
        let stderr = String::from_utf8_lossy(&output.stderr);
        let log = format!("{}\n{}", stdout, stderr);

        // 简单的错误解析逻辑
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
                let line_number = caps.get(1).and_then(|v| v.as_str().parse::<u32>().ok()).unwrap_or(0);
                let message = current_message.take().unwrap_or_else(|| "Compilation error".to_string());
                errors.push(CompileError { line: line_number, message, severity: "error".to_string() });
            }
        }
        if errors.is_empty() {
            errors.push(CompileError::simple(log.trim()));
        }
        return Err(errors);
    }

    if pdf_path.exists() {
        let pdf_data = fs::read(&pdf_path).map_err(|e| vec![CompileError::sys(e)])?;
        Ok(pdf_data)
    } else {
        Err(vec![CompileError::simple("编译成功但未找到生成的 PDF 文件")])
    }
}

// 扩展 CompileError 方便构建
impl CompileError {
    fn simple(msg: impl Into<String>) -> Self {
        Self { line: 0, message: msg.into(), severity: "error".to_string() }
    }
    fn sys(e: std::io::Error) -> Self {
        Self { line: 0, message: e.to_string(), severity: "error".to_string() }
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
