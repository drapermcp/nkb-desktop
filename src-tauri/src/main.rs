// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use std::fs;
use std::path::{Path, PathBuf};
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
struct Node {
    id: String,
    title: String,
    content: String,
    path: String,
    classification: Option<String>,
}

#[derive(Debug, Serialize, Deserialize)]
struct Relationship {
    source: String,
    target: String,
    #[serde(rename = "type")]
    rel_type: String,
    strength: Option<f64>,
    metadata: Option<serde_json::Value>,
}

#[derive(Debug, Serialize, Deserialize)]
struct RelationshipFile {
    domain: Option<String>,
    total_relationships: Option<usize>,
    relationships: Vec<Relationship>,
}

// Get NKB path from environment or use default
fn get_nkb_path() -> Result<PathBuf, String> {
    // Try environment variable first
    if let Ok(path) = std::env::var("NKB_PATH") {
        let path = PathBuf::from(path);
        if path.exists() {
            return Ok(path);
        }
    }
    
    // Default path (Mac)
    let default_path = PathBuf::from("/Users/danraper/DRLibrary/DOCKER2/nkb-ani/neural-knowledge-base");
    if default_path.exists() {
        return Ok(default_path);
    }
    
    // Fallback: user's home directory
    let home = dirs::home_dir().ok_or("Could not find home directory")?;
    let fallback = home.join(".nkb-browser");
    
    Ok(fallback)
}

// Parse markdown frontmatter (simplified)
fn parse_markdown(content: &str) -> Result<(std::collections::HashMap<String, String>, String), String> {
    let mut frontmatter = std::collections::HashMap::new();
    
    if !content.starts_with("---") {
        return Ok((frontmatter, content.to_string()));
    }
    
    let parts: Vec<&str> = content.splitn(3, "---").collect();
    if parts.len() < 3 {
        return Ok((frontmatter, content.to_string()));
    }
    
    // Parse frontmatter (simplified - you'd want a proper YAML parser)
    let frontmatter_text = parts[1];
    for line in frontmatter_text.lines() {
        if let Some((key, value)) = line.split_once(':') {
            let key = key.trim().to_string();
            let value = value.trim().trim_matches('"').trim_matches('\'').to_string();
            frontmatter.insert(key, value);
        }
    }
    
    let body = parts[2].trim_start().to_string();
    Ok((frontmatter, body))
}

// Extract ID from path
fn extract_id_from_path(path: &Path) -> String {
    path.file_stem()
        .and_then(|s| s.to_str())
        .unwrap_or("unknown")
        .to_string()
}

// Extract title from filename
fn extract_title_from_filename(path: &Path) -> String {
    path.file_stem()
        .and_then(|s| s.to_str())
        .unwrap_or("Untitled")
        .replace("-", " ")
        .replace("_", " ")
    // TODO: Capitalize words properly
}

#[tauri::command]
fn read_nodes(domain: String) -> Result<Vec<Node>, String> {
    let nkb_path = get_nkb_path()?;
    let domain_path = nkb_path.join("nodes").join(&domain);
    
    let mut nodes = Vec::new();
    
    if !domain_path.exists() {
        return Ok(nodes);
    }
    
    // Read directory recursively
    read_nodes_recursive(&domain_path, &nkb_path, &mut nodes)?;
    
    Ok(nodes)
}

fn read_nodes_recursive(
    dir: &Path,
    base_path: &Path,
    nodes: &mut Vec<Node>,
) -> Result<(), String> {
    let entries = fs::read_dir(dir)
        .map_err(|e| format!("Failed to read directory {}: {}", dir.display(), e))?;
    
    for entry in entries {
        let entry = entry.map_err(|e| format!("Failed to read entry: {}", e))?;
        let path = entry.path();
        
        if path.is_dir() {
            read_nodes_recursive(&path, base_path, nodes)?;
        } else if path.is_file() {
            if let Some(ext) = path.extension() {
                if ext == "md" {
                    let content = fs::read_to_string(&path)
                        .map_err(|e| format!("Failed to read file {}: {}", path.display(), e))?;
                    
                    let (frontmatter, body) = parse_markdown(&content)?;
                    
                    let relative_path = path.strip_prefix(base_path)
                        .map_err(|e| format!("Failed to get relative path: {}", e))?;
                    
                    let id = frontmatter.get("id")
                        .cloned()
                        .unwrap_or_else(|| extract_id_from_path(&path));
                    
                    let title = frontmatter.get("title")
                        .cloned()
                        .unwrap_or_else(|| extract_title_from_filename(&path));
                    
                    nodes.push(Node {
                        id,
                        title,
                        content: body,
                        path: relative_path.to_string_lossy().to_string(),
                        classification: frontmatter.get("classification").cloned(),
                    });
                }
            }
        }
    }
    
    Ok(())
}

#[tauri::command]
fn read_relationships(domain: String) -> Result<RelationshipFile, String> {
    let nkb_path = get_nkb_path()?;
    let rel_path = nkb_path
        .join("data")
        .join("relationships")
        .join("domains")
        .join(format!("{}-relationships.json", domain));
    
    if !rel_path.exists() {
        return Err(format!("Relationship file not found: {}", rel_path.display()));
    }
    
    let content = fs::read_to_string(&rel_path)
        .map_err(|e| format!("Failed to read relationship file: {}", e))?;
    
    let relationships: RelationshipFile = serde_json::from_str(&content)
        .map_err(|e| format!("Failed to parse JSON: {}", e))?;
    
    Ok(relationships)
}

#[tauri::command]
fn read_node(node_id: String) -> Result<Node, String> {
    let nkb_path = get_nkb_path()?;
    
    // Search for node file (simplified - you'd want to index nodes)
    // For now, search in all domains
    let nodes_dir = nkb_path.join("nodes");
    
    if !nodes_dir.exists() {
        return Err("Nodes directory not found".to_string());
    }
    
    // Search recursively
    if let Some(node) = find_node_recursive(&nodes_dir, &node_id, &nkb_path)? {
        return Ok(node);
    }
    
    Err(format!("Node '{}' not found", node_id))
}

fn find_node_recursive(
    dir: &Path,
    node_id: &str,
    base_path: &Path,
) -> Result<Option<Node>, String> {
    let entries = fs::read_dir(dir)
        .map_err(|e| format!("Failed to read directory: {}", e))?;
    
    for entry in entries {
        let entry = entry.map_err(|e| format!("Failed to read entry: {}", e))?;
        let path = entry.path();
        
        if path.is_dir() {
            if let Some(node) = find_node_recursive(&path, node_id, base_path)? {
                return Ok(Some(node));
            }
        } else if path.is_file() {
            if let Some(ext) = path.extension() {
                if ext == "md" {
                    let content = fs::read_to_string(&path)
                        .map_err(|e| format!("Failed to read file: {}", e))?;
                    
                    let (frontmatter, body) = parse_markdown(&content)?;
                    
                    let id = frontmatter.get("id")
                        .cloned()
                        .unwrap_or_else(|| extract_id_from_path(&path));
                    
                    if id == node_id || path.file_stem().and_then(|s| s.to_str()) == Some(node_id) {
                        let relative_path = path.strip_prefix(base_path)
                            .map_err(|e| format!("Failed to get relative path: {}", e))?;
                        
                        let title = frontmatter.get("title")
                            .cloned()
                            .unwrap_or_else(|| extract_title_from_filename(&path));
                        
                        return Ok(Some(Node {
                            id,
                            title,
                            content: body,
                            path: relative_path.to_string_lossy().to_string(),
                            classification: frontmatter.get("classification").cloned(),
                        }));
                    }
                }
            }
        }
    }
    
    Ok(None)
}

#[tauri::command]
fn list_domains() -> Result<Vec<String>, String> {
    let nkb_path = get_nkb_path()?;
    let nodes_dir = nkb_path.join("nodes");
    
    if !nodes_dir.exists() {
        return Ok(Vec::new());
    }
    
    let entries = fs::read_dir(&nodes_dir)
        .map_err(|e| format!("Failed to read nodes directory: {}", e))?;
    
    let mut domains = Vec::new();
    for entry in entries {
        let entry = entry.map_err(|e| format!("Failed to read entry: {}", e))?;
        let path = entry.path();
        
        if path.is_dir() {
            if let Some(name) = path.file_name().and_then(|n| n.to_str()) {
                domains.push(name.to_string());
            }
        }
    }
    
    Ok(domains)
}

fn main() {
    tauri::Builder::default()
        .invoke_handler(tauri::generate_handler![
            read_nodes,
            read_relationships,
            read_node,
            list_domains
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}

