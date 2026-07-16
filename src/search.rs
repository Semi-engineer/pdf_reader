/*!
Search Manager
Handles text search in PDF documents
*/

use egui::Rect;

#[derive(Debug, Clone)]
pub struct SearchResult {
    pub page: usize,
    pub rect: Rect,
    pub text: String,
}

pub struct SearchManager {
    query: String,
    results: Vec<SearchResult>,
    current_result_index: usize,
}

impl SearchManager {
    pub fn new() -> Self {
        Self {
            query: String::new(),
            results: Vec::new(),
            current_result_index: 0,
        }
    }
    
    /// Set search query
    pub fn set_query(&mut self, query: String) {
        self.query = query;
    }
    
    /// Get current search query
    pub fn query(&self) -> &str {
        &self.query
    }
    
    /// Set search results
    pub fn set_results(&mut self, results: Vec<SearchResult>) {
        self.results = results;
        self.current_result_index = 0;
    }
    
    /// Get all search results
    pub fn results(&self) -> &[SearchResult] {
        &self.results
    }
    
    /// Get results for a specific page
    pub fn page_results(&self, page: usize) -> Vec<&SearchResult> {
        self.results.iter().filter(|r| r.page == page).collect()
    }
    
    /// Move to next search result
    pub fn next_result(&mut self) -> Option<&SearchResult> {
        if self.results.is_empty() {
            return None;
        }
        
        self.current_result_index = (self.current_result_index + 1) % self.results.len();
        self.results.get(self.current_result_index)
    }
    
    /// Move to previous search result
    pub fn prev_result(&mut self) -> Option<&SearchResult> {
        if self.results.is_empty() {
            return None;
        }
        
        if self.current_result_index == 0 {
            self.current_result_index = self.results.len() - 1;
        } else {
            self.current_result_index -= 1;
        }
        
        self.results.get(self.current_result_index)
    }
    
    /// Get current result
    pub fn current_result(&self) -> Option<&SearchResult> {
        self.results.get(self.current_result_index)
    }
    
    /// Get current result index
    pub fn current_index(&self) -> usize {
        self.current_result_index
    }
    
    /// Get total result count
    pub fn result_count(&self) -> usize {
        self.results.len()
    }
    
    /// Clear search
    pub fn clear(&mut self) {
        self.query.clear();
        self.results.clear();
        self.current_result_index = 0;
    }
    
    /// Check if search is active
    pub fn is_active(&self) -> bool {
        !self.query.is_empty()
    }
}

impl Default for SearchManager {
    fn default() -> Self {
        Self::new()
    }
}
