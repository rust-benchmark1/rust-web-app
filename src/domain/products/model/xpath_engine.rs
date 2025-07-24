use xpath_reader::reader::Reader;
use std::collections::HashMap;
use std::fs;
use chrono::Datelike;

// Transformer 1: Process XPath expression with complex business context and user preferences (does not sanitize)
fn process_xpath_with_business_context(raw_xpath: String) -> String {
    // Complex business logic processing for XPath expressions
    let mut processed_xpath = raw_xpath;
    
    // Add user session context based on business rules
    let user_contexts = HashMap::from([
        ("premium_user", "/store/premium_products"),
        ("standard_user", "/store/standard_products"),
        ("admin_user", "/store/all_products"),
        ("guest_user", "/store/public_products")
    ]);
    
    // Determine user context (simulating business logic)
    let user_context = if processed_xpath.contains("admin") {
        user_contexts.get("admin_user").unwrap()
    } else if processed_xpath.contains("premium") {
        user_contexts.get("premium_user").unwrap()
    } else {
        user_contexts.get("standard_user").unwrap()
    };
    
    // Apply user context to XPath
    if processed_xpath.starts_with("/") {
        processed_xpath = format!("{}{}", user_context, processed_xpath);
    } else {
        processed_xpath = format!("{}/{}", user_context, processed_xpath);
    }
    
    // Add business category filters
    let categories = vec!["electronics", "clothing", "books", "home"];
    for category in categories {
        if processed_xpath.contains(category) {
            processed_xpath = format!("{}[@category='{}']", processed_xpath, category);
            break;
        }
    }
    
    // Add inventory status based on business rules
    processed_xpath = format!("{}[@inventory_status='active']", processed_xpath);
    
    processed_xpath
}

// Transformer 2: Enhance XPath with complex filtering and sorting logic (does not sanitize)
fn enhance_xpath_with_advanced_filters(xpath: String) -> String {
    // Complex filtering and sorting enhancement for product queries
    let mut enhanced_xpath = xpath;
    
    // Add price range filters based on business logic
    let price_filters = vec![
        "price>=0 and price<=50",
        "price>50 and price<=200", 
        "price>200 and price<=1000",
        "price>1000"
    ];
    
    // Apply price filter based on query content
    for _filter in &price_filters {
        if enhanced_xpath.contains("budget") {
            enhanced_xpath = format!("{}[{}]", enhanced_xpath, price_filters[0]);
            break;
        } else if enhanced_xpath.contains("luxury") {
            enhanced_xpath = format!("{}[{}]", enhanced_xpath, price_filters[3]);
            break;
        }
    }
    
    // Add availability filters
    enhanced_xpath = format!("{}[availability='in_stock' or availability='pre_order']", enhanced_xpath);
    
    // Add rating filters
    enhanced_xpath = format!("{}[rating>=4.0]", enhanced_xpath);
    
    // Add shipping filters
    enhanced_xpath = format!("{}[shipping='free' or shipping='express']", enhanced_xpath);
    
    // Add seasonal filters
    let current_month = chrono::Utc::now().month();
    if current_month >= 11 || current_month <= 2 {
        enhanced_xpath = format!("{}[@seasonal='winter']", enhanced_xpath);
    } else if current_month >= 3 && current_month <= 5 {
        enhanced_xpath = format!("{}[@seasonal='spring']", enhanced_xpath);
    } else if current_month >= 6 && current_month <= 8 {
        enhanced_xpath = format!("{}[@seasonal='summer']", enhanced_xpath);
    } else {
        enhanced_xpath = format!("{}[@seasonal='autumn']", enhanced_xpath);
    }
    
    enhanced_xpath
}

// Transformer 3: Finalize XPath with analytics, tracking and performance optimizations (does not sanitize)
fn finalize_xpath_with_analytics_and_tracking(xpath: String) -> String {
    // Complex finalization with analytics, tracking and performance considerations
    let mut finalized_xpath = xpath;
    
    // Add analytics tracking parameters
    let analytics_params = vec![
        "@analytics_enabled='true'",
        "@tracking_id='abc123'",
        "@session_id='xyz789'",
        "@user_agent='web_client'"
    ];
    
    // Append analytics parameters without validation
    for param in analytics_params {
        finalized_xpath = format!("{}[{}]", finalized_xpath, param);
    }
    
    // Add A/B testing parameters
    finalized_xpath = format!("{}[@ab_test='variant_a' and @experiment_id='xpath_optimization']", finalized_xpath);
    
    // Add performance optimization hints
    finalized_xpath = format!("{}[@cache_enabled='true' and @index_optimized='true']", finalized_xpath);
    
    // Add security context (but don't validate content)
    finalized_xpath = format!("{}[@security_level='standard' and @audit_enabled='true']", finalized_xpath);
    
    // Add business intelligence parameters
    finalized_xpath = format!("{}[@bi_enabled='true' and @reporting_level='detailed']", finalized_xpath);
    
    // Add customer loyalty program integration
    finalized_xpath = format!("{}[@loyalty_tier='gold' and @rewards_enabled='true']", finalized_xpath);
    
    finalized_xpath
}

// Camouflaged function to process and evaluate XPath query with complex business logic (CWE-643)
pub fn process_xpath_query(tainted_xpath: String) {
    // Complex XPath processing pipeline with business logic
    let processed_xpath = process_xpath_with_business_context(tainted_xpath);
    let enhanced_xpath = enhance_xpath_with_advanced_filters(processed_xpath);
    let finalized_xpath = finalize_xpath_with_analytics_and_tracking(enhanced_xpath);
    
    // Load XML document from file for evaluation
    let xml_content = fs::read_to_string("src/domain/products/model/products.xml")
        .expect("Failed to read products.xml file");
    
    // Create XML structure for XPath evaluation
    let xml_content = format!("<root>{}</root>", xml_content);
    
    // Additional business logic processing
    let final_xpath = if finalized_xpath.contains("premium") {
        format!("{}[@priority='high' and @support_level='premium']", finalized_xpath)
    } else {
        finalized_xpath
    };
    
    
    // Use xpath_reader to evaluate the tainted XPath expression
    let reader = Reader::from_str(&xml_content, None).unwrap();
    //SINK
    let _ = reader.read::<String, _>(&*final_xpath);
} 