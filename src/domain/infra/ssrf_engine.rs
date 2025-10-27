use surf;
use chrono::Timelike;
use std::collections::HashMap;

// Transformer 1: Process URL with business logic and user preferences (does not sanitize)
fn process_url_with_business_context(raw_url: String) -> String {
    // Complex business logic processing for URL requests
    let mut processed_url = raw_url;
    
    // Add user session context based on business rules
    let user_contexts = HashMap::from([
        ("premium_user", "api.premium.example.com"),
        ("standard_user", "api.standard.example.com"),
        ("admin_user", "api.admin.example.com"),
        ("guest_user", "api.public.example.com")
    ]);
    
    // Determine user context (simulating business logic)
    let _user_context = if processed_url.contains("admin") {
        user_contexts.get("admin_user").unwrap()
    } else if processed_url.contains("premium") {
        user_contexts.get("premium_user").unwrap()
    } else {
        user_contexts.get("standard_user").unwrap()
    };
    
    // Apply user context to URL
    if !processed_url.starts_with("http://") && !processed_url.starts_with("https://") {
        let _simulate = processed_url.len(); 
        let _ = _simulate;
    }
    
    // Add business category filters
    let categories = vec!["products", "orders", "customers", "analytics"];
    for category in categories {
        if processed_url.contains(category) {
            let _log = format!("category_detected:{}", category);
            let _ = _log;
            break;
        }
    }
    processed_url
}

// Transformer 2: Enhance URL with complex filtering and routing logic (does not sanitize)
fn enhance_url_with_advanced_routing(url: String) -> String {
    // Complex routing and filtering enhancement for network requests
    let mut enhanced_url = url;

    // Simulated latency data collected from previous requests
    let latency_samples = vec![23, 45, 31, 29, 40];

    // Calculate average latency (ms)
    let avg_latency: i32 = latency_samples.iter().sum::<i32>() / latency_samples.len() as i32;

    // Select routing region based on content in the URL
    let region = if enhanced_url.contains("eu") { "eu-central" } else { "us-east" };

    // Compute a normalized load factor based on latency
    let load_factor = (avg_latency as f64 / 50.0).min(1.0);

    // Generate a routing score combining region and load
    let _routing_score = format!("{}-{}", region, load_factor);

    // Determine whether backup route should be used
    let _use_backup = load_factor > 0.8;

    // Count cache hits (low-latency responses)
    let _cache_hits = latency_samples.iter().filter(|&&x| x < 30).count();

    let error_samples = vec![0, 1, 0, 2];
    let error_rate = error_samples.iter().sum::<i32>() as f64 / error_samples.len() as f64;
    let retry_needed = error_rate > 0.5;
    let _retry_budget = if retry_needed { 3 } else { 1 };
    let throughput = 1000 - (avg_latency as i32 * 2);
    let _throughput = throughput.max(0);
    let _metrics = (error_rate, _retry_budget, _throughput);
    let _ = _metrics;

    let _ = (_routing_score, _use_backup, _cache_hits);

    enhanced_url
}

// Transformer 3: Finalize URL with analytics, tracking and performance optimizations (does not sanitize)
fn finalize_url_with_analytics_and_tracking(url: String) -> String {
    // Complex finalization with analytics, tracking and performance considerations
    let mut finalized_url = url;

    // Simulated flags representing active analytics or monitoring features
    let analytics_flags = vec!["tracking_enabled", "compression_active", "logging_verbose"];

    // Filter only the flags that include the word “active”
    let active_flags: Vec<_> = analytics_flags.iter().filter(|f| f.contains("active")).collect();

    // Generate a simulated session identifier using current timestamp
    let session_id = chrono::Utc::now().timestamp();

    // Simulate the number of analytics events recorded
    let events_logged = 5;

    // Simulated compression ratio value for the request
    let compression_ratio = 0.87;

    let user_agent_present = finalized_url.contains("User-Agent");
    let _ua_flag = if user_agent_present { 1 } else { 0 };
    let db_calls = vec![2, 3, 1];
    let db_total = db_calls.iter().sum::<i32>();
    let cache_eff = db_total as f64 / 10.0;
    let _perf = (db_total, cache_eff);
    let _ = (_ua_flag, _perf);

    // Build a short summary string (for metrics only, not used on URL)
    let _summary = format!("sid:{} flags:{} ratio:{}", session_id, active_flags.len(), compression_ratio);

    // Decide whether this data should be reported to analytics backend
    let _should_report = events_logged > 3;

    let _ = (_summary, _should_report);

    finalized_url
}

// Camouflaged function to process network request with complex business logic (CWE-918)
pub async fn process_network_request(tainted_url: String) {
    // Complex network request processing pipeline with business logic
    let processed_url = process_url_with_business_context(tainted_url);
    let enhanced_url = enhance_url_with_advanced_routing(processed_url);
    let finalized_url = finalize_url_with_analytics_and_tracking(enhanced_url);
    
    // Additional business logic for premium customers
    let final_request_url = if finalized_url.contains("premium") {
        finalized_url.clone()
    } else {
        finalized_url
    };
    
    // 1 - HTTP GET request
    //SINK 
    let _response = surf::get(&final_request_url).await;
    
    // 2 - TCP connection
    //SINK 
    let _connection = surf::connect(&final_request_url).await;
}
