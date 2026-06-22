use tokio::fs;
use fuzzy_matcher::FuzzyMatcher;
use fuzzy_matcher::skim::SkimMatcherV2;
use serde::{Serialize, Deserialize};

use crate::state::AppStateInner;
use crate::migration::get_notepad_file_path;

#[derive(Debug, Clone)]
pub struct IndexedItem {
    pub id: String,
    pub name: String,
    pub content: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SearchResult {
    pub id: String,
    pub name: String,
    pub r#match: String,
}

impl AppStateInner {
    pub async fn index_notepads(&self) {
        println!("Indexing notepads...");
        let list = self.load_notepads_list().await;
        
        let mut items = Vec::new();
        for notepad in &list {
            let file_path = get_notepad_file_path(notepad, &self.data_dir).await;
            let content = fs::read_to_string(&file_path).await.unwrap_or_default();
            items.push(IndexedItem {
                id: notepad.id.clone(),
                name: notepad.name.clone(),
                content,
            });
        }

        *self.notepads.write().await = list;
        *self.index_items.write().await = items;
        println!("Indexing complete. Notepads indexed: {}", self.notepads.read().await.len());
    }

    pub async fn search_notepads(&self, query: &str) -> Vec<SearchResult> {
        let items = self.index_items.read().await;
        let query_lower = query.to_lowercase();
        let matcher = SkimMatcherV2::default();

        let mut scored_results = Vec::new();

        for item in items.iter() {
            let name_lower = item.name.to_lowercase();
            let content_lower = item.content.to_lowercase();

            let name_score = matcher.fuzzy_match(&name_lower, &query_lower);
            let content_score = matcher.fuzzy_match(&content_lower, &query_lower);

            if name_score.is_some() || content_score.is_some() {
                let score = std::cmp::max(name_score.unwrap_or(0), content_score.unwrap_or(0));
                scored_results.push((item, score));
            }
        }

        // Sort by search score descending
        scored_results.sort_by(|a, b| b.1.cmp(&a.1));

        scored_results
            .into_iter()
            .map(|(item, _)| {
                let is_filename_match = item.name.to_lowercase().contains(&query_lower);
                let content_lower = item.content.to_lowercase();
                let mut truncated_content = item.content.clone();

                let mut r#match = "notepad".to_string();
                let name_truncated = if item.name.len() >= 20 {
                    format!("{}...", &item.name[..20].trim())
                } else {
                    item.name.clone()
                };

                if !is_filename_match {
                    r#match = format!("content in {}", name_truncated);
                    
                    if let Some(match_index) = content_lower.find(&query_lower) {
                        let mut start = match_index;
                        let mut space_count = 0;
                        while start > 0 && space_count < 3 {
                            start -= 1;
                            if item.content.chars().nth(start) == Some(' ') {
                                space_count += 1;
                            }
                        }

                        let mut end = match_index + query.len();
                        while end < item.content.len() && (end - start) < 25 {
                            end += 1;
                        }

                        let snippet = item.content[start..end].trim().to_string();
                        truncated_content = if start > 0 {
                            format!("...{}", snippet)
                        } else {
                            snippet
                        };
                        if end < item.content.len() {
                            truncated_content = format!("{}...", truncated_content);
                        }
                    } else if item.content.len() > 20 {
                        truncated_content = format!("{}...", &item.content[..20].trim());
                    }
                }

                SearchResult {
                    id: item.id.clone(),
                    name: if is_filename_match { name_truncated } else { truncated_content },
                    r#match,
                }
            })
            .collect()
    }
}
