use clap::Parser;
use glob::glob;
use rayon::prelude::*;
use roxmltree::Document;
use std::{fs, path::PathBuf};

#[derive(clap::ValueEnum, Clone, Debug)]
enum SearchMode {
    MediaID,
    Guid,
    ShortID,
}

#[derive(Parser)]
#[command(
    name = "find-id",
    author = "xmimu <1101588023@qq.com>",
    version = "0.1.0",
    about = "在 *.wwu 文件中查找匹配的 MediaID、GUID 或 ShortID"
)]
struct Cli {
    /// 要查找的 ID 字符串（可部分匹配，不区分大小写）
    id: String,

    /// 要搜索的文件夹路径（包含 .wproj 文件）
    #[arg(value_parser = is_path_valid)]
    path: PathBuf,

    #[arg(long, short, value_enum, default_value = "guid")]
    mode: SearchMode,
}

#[derive(Debug)]
struct MatchInfo {
    tag: String,
    name: String,
    id: String,
    short_id: String,
    media_id: String,
    language: String,
    audio_file: String,
}

fn is_path_valid(path: &str) -> Result<PathBuf, String> {
    let path = PathBuf::from(path);

    if !path.is_dir() {
        return Err(format!(
            "Path '{}' is not a valid directory",
            path.display()
        ));
    }

    let has_wproj = path
        .read_dir()
        .map_err(|e| format!("Failed to read directory '{}': {}", path.display(), e))?
        .filter_map(Result::ok)
        .map(|entry| entry.path())
        .any(|p| p.is_file() && p.extension().map_or(false, |ext| ext == "wproj"));

    if has_wproj {
        Ok(path)
    } else {
        Err(format!(
            "No '.wproj' files found in directory '{}'",
            path.display()
        ))
    }
}

fn find_id(query: &str, path: &str, mode: &SearchMode) -> Vec<MatchInfo> {
    let query = query.to_lowercase();
    let pattern = format!("{}/**/*.wwu", path);
    let entries: Vec<PathBuf> = glob(&pattern)
        .expect("Failed to read glob pattern")
        .filter_map(Result::ok)
        .collect();

    let results: Vec<MatchInfo> = entries
        .par_iter()
        .flat_map_iter(|p| {
            let contents = match fs::read_to_string(p) {
                Ok(c) => c,
                Err(e) => {
                    eprintln!("读取文件失败：{} {}", p.display(), e);
                    return Vec::new();
                }
            };
            match mode {
                SearchMode::MediaID => search_media_id(&query, &contents),
                SearchMode::Guid => search_guid(&query, &contents),
                SearchMode::ShortID => search_short_id(&query, &contents),
            }
        })
        .collect();

    results
}

fn search_media_id(query: &str, contents: &str) -> Vec<MatchInfo> {
    let doc = Document::parse(contents).unwrap();
    let mut results = Vec::new();

    for node in doc.descendants().filter(|n| n.has_tag_name("MediaID")) {
        let id = node.attribute("ID").unwrap_or("?");
        if id.to_lowercase().contains(query) {
            let parent = node.parent_element().unwrap().parent_element().unwrap();

            results.push(MatchInfo {
                tag: parent.tag_name().name().to_string(),
                name: parent.attribute("Name").unwrap_or("?").to_string(),
                id: parent.attribute("ID").unwrap_or("?").to_string(),
                short_id: "?".to_string(),
                media_id: id.to_string(),
                language: parent
                    .children()
                    .find(|n| n.tag_name().name().contains("Language"))
                    .and_then(|n| n.text())
                    .unwrap_or("?")
                    .to_string(),
                audio_file: parent
                    .children()
                    .find(|n| n.tag_name().name().contains("AudioFile"))
                    .and_then(|n| n.text())
                    .unwrap_or("?")
                    .to_string(),
            });
        }
    }
    results
}

fn search_guid(query: &str, contents: &str) -> Vec<MatchInfo> {
    let doc = Document::parse(contents).unwrap();
    let mut results = Vec::new();

    for node in doc.descendants().filter(|n| n.has_attribute("ID")) {
        let id = node.attribute("ID").unwrap_or("?");
        if id.to_lowercase().contains(query) {
            results.push(MatchInfo {
                tag: node.tag_name().name().to_string(),
                name: node.attribute("Name").unwrap_or("?").to_string(),
                id: id.to_string(),
                short_id: node.attribute("ShortID").unwrap_or("?").to_string(),
                media_id: "".to_string(),
                language: "".to_string(),
                audio_file: "".to_string(),
            });
        }
    }
    results
}

fn search_short_id(query: &str, contents: &str) -> Vec<MatchInfo> {
    let doc = Document::parse(contents).unwrap();
    let mut results = Vec::new();

    for node in doc.descendants().filter(|n| n.has_attribute("ShortID")) {
        let short_id = node.attribute("ShortID").unwrap_or("?");
        if short_id.to_lowercase().contains(query) {
            results.push(MatchInfo {
                tag: node.tag_name().name().to_string(),
                name: node.attribute("Name").unwrap_or("?").to_string(),
                id: node.attribute("ID").unwrap_or("?").to_string(),
                short_id: short_id.to_string(),
                media_id: "".to_string(),
                language: "".to_string(),
                audio_file: "".to_string(),
            });
        }
    }
    results
}

fn main() {
    let args = Cli::parse();

    let results = find_id(&args.id, args.path.to_str().unwrap(), &args.mode);

    match &args.mode {
        SearchMode::MediaID => {
            for r in &results {
                println!(
                    "MediaID: {} | Tag: {} | Name: {} | ID: {} | Language: {} | AudioFile: {}",
                    r.media_id, r.tag, r.name, r.id, r.language, r.audio_file
                );
            }
        }
        _ => {
            for r in &results {
                println!(
                    "Tag: {} | Name: {} | ID: {} | ShortID: {}",
                    r.tag, r.name, r.id, r.short_id
                );
            }
        }
    }

    println!("共匹配到 {} 条结果", results.len());
}
