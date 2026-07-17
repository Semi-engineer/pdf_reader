/*!
Render Scheduler
Priority-based render queue with cancellation support
*/

use crate::page_cache::CacheKey;
use crate::performance_metrics::PerformanceMetrics;
use std::collections::{HashMap, VecDeque};
use std::sync::{Arc, Mutex};
use std::time::Instant;

/// Priority levels for render requests
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum RenderPriority {
    /// User is currently viewing this page (highest priority)
    Visible = 0,
    /// Page is near the viewport
    Prefetch = 1,
    /// Background thumbnail generation
    Thumbnail = 2,
    /// Low priority background task
    Background = 3,
}

/// A render request with priority and metadata
#[derive(Debug, Clone)]
pub struct RenderRequest {
    pub key: CacheKey,
    pub priority: RenderPriority,
    pub requested_at: Instant,
    pub request_id: u64,
}

/// Tracks active render jobs
#[derive(Debug, Clone)]
struct ActiveJob {
    pub key: CacheKey,
    pub priority: RenderPriority,
    pub started_at: Instant,
    pub request_id: u64,
}

/// Render scheduler manages a priority queue of render requests.
pub struct RenderScheduler {
    /// Pending requests by priority
    queue: Arc<Mutex<HashMap<RenderPriority, VecDeque<RenderRequest>>>>,
    /// Currently active jobs
    active: Arc<Mutex<HashMap<u64, ActiveJob>>>,
    /// Cancelled request IDs
    cancelled: Arc<Mutex<Vec<u64>>>,
    /// Next request ID
    next_id: Arc<Mutex<u64>>,
    /// Maximum concurrent renders
    max_concurrent: usize,
    /// Metrics
    metrics: Arc<Mutex<PerformanceMetrics>>,
}

impl RenderScheduler {
    pub fn new(max_concurrent: usize) -> Self {
        let mut queue = HashMap::new();
        queue.insert(RenderPriority::Visible, VecDeque::new());
        queue.insert(RenderPriority::Prefetch, VecDeque::new());
        queue.insert(RenderPriority::Thumbnail, VecDeque::new());
        queue.insert(RenderPriority::Background, VecDeque::new());
        
        Self {
            queue: Arc::new(Mutex::new(queue)),
            active: Arc::new(Mutex::new(HashMap::new())),
            cancelled: Arc::new(Mutex::new(Vec::new())),
            next_id: Arc::new(Mutex::new(0)),
            max_concurrent,
            metrics: Arc::new(Mutex::new(PerformanceMetrics::new())),
        }
    }
    
    /// Request a page render with given priority
    pub fn request(&self, key: CacheKey, priority: RenderPriority) -> u64 {
        let mut next_id = self.next_id.lock().unwrap();
        let request_id = *next_id;
        *next_id += 1;
        drop(next_id);
        
        let request = RenderRequest {
            key,
            priority,
            requested_at: Instant::now(),
            request_id,
        };
        
        let mut queue = self.queue.lock().unwrap();
        if let Some(pq) = queue.get_mut(&priority) {
            // Don't add duplicate requests for same cache key
            if !pq.iter().any(|r| r.key == request.key) {
                pq.push_back(request);
            }
        }
        
        request_id
    }
    
    /// Cancel a specific request
    pub fn cancel_request(&self, request_id: u64) {
        let mut cancelled = self.cancelled.lock().unwrap();
        cancelled.push(request_id);
    }
    
    /// Cancel all requests for a specific cache key
    pub fn cancel_key(&self, key: &CacheKey) {
        let mut queue = self.queue.lock().unwrap();
        for pq in queue.values_mut() {
            pq.retain(|r| r.key != *key);
        }
    }
    
    /// Cancel all requests below a certain priority
    pub fn cancel_below_priority(&self, min_priority: RenderPriority) {
        let mut queue = self.queue.lock().unwrap();
        for (&priority, pq) in queue.iter_mut() {
            if priority > min_priority {
                pq.clear();
            }
        }
    }
    
    /// Cancel all pending requests (does not cancel active jobs)
    pub fn cancel_all_pending(&self) {
        let mut queue = self.queue.lock().unwrap();
        for pq in queue.values_mut() {
            pq.clear();
        }
    }
    
    /// Get next render request to process (highest priority first)
    pub fn next_request(&self) -> Option<RenderRequest> {
        let active_count = self.active.lock().unwrap().len();
        if active_count >= self.max_concurrent {
            return None;
        }
        
        let mut queue = self.queue.lock().unwrap();
        let cancelled = self.cancelled.lock().unwrap();
        
        // Try each priority level from highest to lowest
        for priority in &[
            RenderPriority::Visible,
            RenderPriority::Prefetch,
            RenderPriority::Thumbnail,
            RenderPriority::Background,
        ] {
            if let Some(pq) = queue.get_mut(priority) {
                // Skip cancelled requests
                while let Some(req) = pq.pop_front() {
                    if !cancelled.contains(&req.request_id) {
                        // Mark as active
                        let mut active = self.active.lock().unwrap();
                        active.insert(
                            req.request_id,
                            ActiveJob {
                                key: req.key.clone(),
                                priority: req.priority,
                                started_at: Instant::now(),
                                request_id: req.request_id,
                            },
                        );
                        return Some(req);
                    }
                }
            }
        }
        
        None
    }
    
    /// Mark a request as completed
    pub fn complete_request(&self, request_id: u64) {
        let mut active = self.active.lock().unwrap();
        if let Some(job) = active.remove(&request_id) {
            let duration = Instant::now().duration_since(job.started_at);
            let mut metrics = self.metrics.lock().unwrap();
            metrics.record_render_duration(duration);
            metrics.total_renders += 1;
        }
    }
    
    /// Mark a request as failed
    pub fn fail_request(&self, request_id: u64) {
        let mut active = self.active.lock().unwrap();
        active.remove(&request_id);
        let mut metrics = self.metrics.lock().unwrap();
        metrics.failed_renders += 1;
    }
    
    /// Check if a request was cancelled
    pub fn is_cancelled(&self, request_id: u64) -> bool {
        let cancelled = self.cancelled.lock().unwrap();
        cancelled.contains(&request_id)
    }
    
    /// Get current queue length
    pub fn queue_length(&self) -> usize {
        let queue = self.queue.lock().unwrap();
        queue.values().map(|pq| pq.len()).sum()
    }
    
    /// Get active job count
    pub fn active_count(&self) -> usize {
        self.active.lock().unwrap().len()
    }
    
    /// Get metrics
    pub fn metrics(&self) -> PerformanceMetrics {
        self.metrics.lock().unwrap().clone()
    }
    
    /// Update queue length metric
    pub fn update_metrics(&self) {
        let queue_len = self.queue_length();
        let mut metrics = self.metrics.lock().unwrap();
        metrics.record_queue_length(queue_len);
    }
    
    /// Clear old cancelled IDs (periodically call this to prevent memory growth)
    pub fn cleanup_cancelled(&self) {
        let mut cancelled = self.cancelled.lock().unwrap();
        if cancelled.len() > 1000 {
            cancelled.clear();
        }
    }
}

impl Clone for RenderScheduler {
    fn clone(&self) -> Self {
        Self {
            queue: Arc::clone(&self.queue),
            active: Arc::clone(&self.active),
            cancelled: Arc::clone(&self.cancelled),
            next_id: Arc::clone(&self.next_id),
            max_concurrent: self.max_concurrent,
            metrics: Arc::clone(&self.metrics),
        }
    }
}
