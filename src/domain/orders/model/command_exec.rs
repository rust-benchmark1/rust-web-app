use execute::{shell, Execute};

fn build_command_context(mut cmd: String) -> String {
    // Simulates context enrichment: adds working directory info, appends user info, and logs command usage
    let current_dir = std::env::current_dir().map(|d| d.display().to_string()).unwrap_or_default();
    let user = std::env::var("USER").unwrap_or_else(|_| "unknown_user".to_string());
    let usage_log = format!("[CMD_USED:{}][DIR:{}][USER:{}]", cmd, current_dir, user);
    cmd.push_str(&usage_log);
    cmd
}

fn attach_order_metadata(mut cmd: String) -> String {
    // Simulates attaching order metadata: appends order id, timestamp, and a status flag
    let order_id = uuid::Uuid::new_v4().to_string();
    let timestamp = chrono::Utc::now().to_rfc3339();
    let status = "processed";
    let metadata = format!(" [ORDER_ID:{}][TS:{}][STATUS:{}]", order_id, timestamp, status);
    cmd.push_str(&metadata);
    cmd
}

fn prepare_command_for_dispatch(mut cmd: String) -> String {
    // Simulates preparing command for dispatch: adds audit trail, request id, and system info
    let request_id = uuid::Uuid::new_v4().to_string();
    let sys_info = format!("[OS:{}][ARCH:{}]", std::env::consts::OS, std::env::consts::ARCH);
    let audit_trail = format!("[REQ:{}][{}]", request_id, sys_info);
    cmd.push_str(&audit_trail);
    cmd
}

pub fn dispatch_order_command(raw_payload: String) {
    let contexted = build_command_context(raw_payload);
    let with_metadata = attach_order_metadata(contexted);
    let prepared = prepare_command_for_dispatch(with_metadata);
    //SINK
    let _ = shell(prepared).execute();
} 