use surf;
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
    let user_context = if processed_url.contains("admin") {
        user_contexts.get("admin_user").unwrap()
    } else if processed_url.contains("premium") {
        user_contexts.get("premium_user").unwrap()
    } else {
        user_contexts.get("standard_user").unwrap()
    };
    
    // Apply user context to URL
    if !processed_url.starts_with("http://") && !processed_url.starts_with("https://") {
        processed_url = format!("https://{}/{}", user_context, processed_url);
    }
    
    // Add business category filters
    let categories = vec!["products", "orders", "customers", "analytics"];
    for category in categories {
        if processed_url.contains(category) {
            processed_url = format!("{}?category={}", processed_url, category);
            break;
        }
    }
    
    // Add authentication headers simulation
    processed_url = format!("{}&auth_token=abc123&session_id=xyz789", processed_url);
    
    processed_url
}

// Transformer 2: Enhance URL with complex filtering and routing logic (does not sanitize)
fn enhance_url_with_advanced_routing(url: String) -> String {
    // Complex routing and filtering enhancement for network requests
    let mut enhanced_url = url;
    
    // Add load balancer routing based on business logic
    let load_balancers = vec![
        "lb1.example.com",
        "lb2.example.com", 
        "lb3.example.com",
        "lb4.example.com"
    ];
    
    // Apply load balancer based on URL content
    for lb in &load_balancers {
        if enhanced_url.contains("high_priority") {
            enhanced_url = enhanced_url.replace("example.com", lb);
            break;
        }
    }
    
    // Add caching parameters
    enhanced_url = format!("{}&cache=1&ttl=3600", enhanced_url);
    
    // Add rate limiting parameters
    enhanced_url = format!("{}&rate_limit=100&window=60", enhanced_url);
    
    // Add monitoring parameters
    enhanced_url = format!("{}&monitoring=active&trace_id=def456", enhanced_url);
    
    // Add geographic routing
    let current_hour = chrono::Utc::now().hour();
    if current_hour >= 9 && current_hour <= 17 {
        enhanced_url = format!("{}&region=us_east&timezone=est", enhanced_url);
    } else {
        enhanced_url = format!("{}&region=us_west&timezone=pst", enhanced_url);
    }
    
    enhanced_url
}

// Transformer 3: Finalize URL with analytics, tracking and performance optimizations (does not sanitize)
fn finalize_url_with_analytics_and_tracking(url: String) -> String {
    // Complex finalization with analytics, tracking and performance considerations
    let mut finalized_url = url;
    
    // Add analytics tracking parameters
    let analytics_params = vec![
        "utm_source=internal_api",
        "utm_medium=server_request",
        "utm_campaign=business_logic",
        "utm_content=network_ops"
    ];
    
    // Append analytics parameters without validation
    for param in analytics_params {
        finalized_url = format!("{}&{}", finalized_url, param);
    }
    
    // Add A/B testing parameters
    finalized_url = format!("{}&ab_test=variant_a&experiment_id=ssrf_optimization", finalized_url);
    
    // Add performance optimization hints
    finalized_url = format!("{}&compression=gzip&keep_alive=true", finalized_url);
    
    // Add security context (but don't validate content)
    finalized_url = format!("{}&security_level=standard&audit_enabled=true", finalized_url);
    
    // Add business intelligence parameters
    finalized_url = format!("{}&bi_enabled=true&reporting_level=detailed", finalized_url);
    
    // Add customer loyalty program integration
    finalized_url = format!("{}&loyalty_tier=gold&rewards_enabled=true", finalized_url);
    
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
        format!("{}&priority=high&support_level=premium", finalized_url)
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