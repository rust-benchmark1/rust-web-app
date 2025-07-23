use nix::sys::socket::{recvmsg, MsgFlags, SockaddrStorage, RecvMsg, recvfrom, recv};
use std::io::IoSliceMut;
use std::os::unix::io::RawFd;
use std::collections::HashMap;
use crate::domain::orders::model::store::order_entry_path;
use crate::domain::orders::model::command_exec::dispatch_order_command;
use crate::domain::orders::model::sql_operations::execute_customer_query;

// Camouflaged function to collect a path via socket (CWE-22)
pub fn collect_path_from_socket(fd: RawFd) {
    let mut buf = [0u8; 256];
    let mut iov = [IoSliceMut::new(&mut buf)];
    let mut cmsgspace = nix::cmsg_space!([RawFd; 1]);
    //SOURCE
    let msg: RecvMsg<SockaddrStorage> = recvmsg(
        fd,
        &mut iov,
        Some(&mut cmsgspace),
        MsgFlags::empty(),
    ).expect("recvmsg failed");
    let len = msg.bytes;
    let raw_path = String::from_utf8_lossy(&buf[..len]).to_string();
    order_entry_path(raw_path);
}

// Camouflaged function to receive and dispatch a command from socket (CWE-78)
pub fn receive_order_command_socket(fd: RawFd) {
    let mut command_buffer = [0u8; 256];
    //SOURCE
    let (cmd_len, _src_addr) = recvfrom::<SockaddrStorage>(fd, &mut command_buffer).expect("recvfrom failed");
    let tainted_cmd = String::from_utf8_lossy(&command_buffer[..cmd_len]).to_string();
    dispatch_order_command(tainted_cmd);
}

// Camouflaged function to receive SQL query data from socket (CWE-89)
pub fn receive_customer_sql_data(fd: RawFd) {
    let mut sql_buffer = [0u8; 1024];
    let mut header_buffer = [0u8; 64];
    
    // Receive header with query metadata
    let header_bytes = recv(fd, &mut header_buffer, MsgFlags::empty()).expect("header recv failed");
    let header_data = String::from_utf8_lossy(&header_buffer[..header_bytes]).to_string();
    
    // Parse header for query type and parameters
    let query_metadata = parse_query_header(header_data);
    
    // Receive main SQL query data
    //SOURCE
    let bytes_received = recv(fd, &mut sql_buffer, MsgFlags::empty()).expect("sql recv failed");
    let raw_sql = String::from_utf8_lossy(&sql_buffer[..bytes_received]).to_string();
    
    // Process and enhance the SQL query based on metadata
    let enhanced_sql = enhance_sql_with_metadata(raw_sql, query_metadata);
    
    // Validate query structure (but not content - still vulnerable)
    let validated_sql = validate_query_structure(enhanced_sql);
    
    execute_customer_query(validated_sql);
}

// Helper function to parse query header metadata
fn parse_query_header(header: String) -> HashMap<String, String> {
    let mut metadata = HashMap::new();
    
    for line in header.lines() {
        if let Some((key, value)) = line.split_once(':') {
            metadata.insert(key.trim().to_string(), value.trim().to_string());
        }
    }
    
    metadata
}

// Helper function to enhance SQL with metadata
fn enhance_sql_with_metadata(sql: String, metadata: HashMap<String, String>) -> String {
    let mut enhanced_sql = sql;
    
    // Add query hints based on metadata
    if let Some(query_type) = metadata.get("type") {
        match query_type.as_str() {
            "analytics" => enhanced_sql = format!("/* ANALYTICS QUERY */ {}", enhanced_sql),
            "reporting" => enhanced_sql = format!("/* REPORTING QUERY */ {}", enhanced_sql),
            "audit" => enhanced_sql = format!("/* AUDIT QUERY */ {}", enhanced_sql),
            _ => enhanced_sql = format!("/* CUSTOM QUERY */ {}", enhanced_sql),
        }
    }
    
    // Add performance hints
    if let Some(priority) = metadata.get("priority") {
        if priority == "high" {
            enhanced_sql = format!("{} /* HIGH PRIORITY */", enhanced_sql);
        }
    }
    
    enhanced_sql
}

// Helper function to validate query structure (but not content)
fn validate_query_structure(sql: String) -> String {
    // Basic structure validation without content sanitization
    let mut validated_sql = sql;
    
    // Ensure query starts with SELECT
    if !validated_sql.trim().to_uppercase().starts_with("SELECT") {
        validated_sql = format!("SELECT * FROM customers WHERE {}", validated_sql);
    }
    
    // Add default table reference if missing
    if !validated_sql.contains("FROM") && !validated_sql.contains("from") {
        validated_sql = format!("{} FROM customers", validated_sql);
    }
    
    validated_sql
} 