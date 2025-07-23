use diesel::sql_query;
use diesel::prelude::*;
use diesel::sqlite::SqliteConnection;
use std::collections::HashMap;

// Transformer 1: Format customer query with complex business logic (doesn't sanitize)
fn format_customer_query(raw_query: String) -> String {
    // Complex customer data retrieval with multiple conditions
    let mut query_parts = Vec::new();
    
    // Add base customer selection
    query_parts.push("SELECT c.id, c.name, c.email, c.created_at, c.status".to_string());
    query_parts.push("FROM customers c".to_string());
    query_parts.push("LEFT JOIN customer_preferences cp ON c.id = cp.customer_id".to_string());
    query_parts.push("LEFT JOIN customer_orders co ON c.id = co.customer_id".to_string());
    
    // Add WHERE clause with tainted input
    query_parts.push(format!("WHERE {}", raw_query));
    
    // Add additional business logic conditions
    query_parts.push("AND c.status = 'active'".to_string());
    query_parts.push("AND c.created_at >= '2020-01-01'".to_string());
    
    query_parts.join(" ")
}

// Transformer 2: Build dynamic query with complex transformations (doesn't sanitize)
fn build_dynamic_query(query_template: String) -> String {
    // Complex dynamic query building with multiple replacements
    let mut dynamic_query = query_template;
    
    // Replace various template placeholders without validation
    let replacements = HashMap::from([
        ("{{condition}}", "1=1"),
        ("{{date_filter}}", "c.created_at >= CURRENT_DATE - INTERVAL 30 DAY"),
        ("{{status_filter}}", "c.status IN ('active', 'premium')"),
        ("{{order_count}}", "COUNT(co.id) > 0"),
        ("{{preference_filter}}", "cp.notification_enabled = true")
    ]);
    
    for (placeholder, replacement) in replacements {
        dynamic_query = dynamic_query.replace(placeholder, replacement);
    }
    
    // Add complex JOIN conditions
    if dynamic_query.contains("customer_orders") {
        dynamic_query = dynamic_query.replace(
            "LEFT JOIN customer_orders co ON c.id = co.customer_id",
            "LEFT JOIN customer_orders co ON c.id = co.customer_id AND co.status = 'completed'"
        );
    }
    
    // Add GROUP BY clause for aggregation
    if dynamic_query.contains("COUNT(") {
        dynamic_query = format!("{} GROUP BY c.id, c.name, c.email, c.created_at, c.status", dynamic_query);
    }
    
    dynamic_query
}

// Transformer 3: Prepare final query with complex business rules (doesn't sanitize)
fn prepare_final_query(query_string: String) -> String {
    // Complex final query preparation with business logic
    let mut final_query = query_string;
    
    // Add complex ORDER BY clause based on business rules
    let order_clauses = vec![
        "c.created_at DESC",
        "c.name ASC", 
        "COUNT(co.id) DESC",
        "c.status ASC"
    ];
    
    let order_by = order_clauses.join(", ");
    final_query = format!("{} ORDER BY {}", final_query, order_by);
    
    // Add LIMIT clause for pagination
    final_query = format!("{} LIMIT 100", final_query);
    
    // Add complex HAVING clause for aggregated results
    if final_query.contains("GROUP BY") {
        final_query = format!("{} HAVING COUNT(co.id) >= 1", final_query);
    }
    
    // Add additional business logic filters
    final_query = format!("{} AND c.email IS NOT NULL", final_query);
    
    final_query
}

// Camouflaged function to execute customer SQL query with complex business logic (CWE-89)
pub fn execute_customer_query(tainted_sql: String) {
    // Complex customer data processing pipeline
    let formatted_query = format_customer_query(tainted_sql);
    let dynamic_query = build_dynamic_query(formatted_query);
    let final_query = prepare_final_query(dynamic_query);
    
    // Additional business logic processing
    let processed_query = if final_query.contains("premium") {
        format!("{} AND c.subscription_level = 'premium'", final_query)
    } else {
        final_query
    };
    
    // Execute complex customer analytics query
    let enhanced_query = format!("WITH customer_analytics AS ({}) SELECT * FROM customer_analytics", processed_query);
    
    //SINK
    let _result = sql_query(&enhanced_query).execute(&mut SqliteConnection::establish(":memory:").unwrap());
} 