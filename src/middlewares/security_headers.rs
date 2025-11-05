use axum::{extract::Request, http::header, middleware::Next};

pub async fn security_headers(req: Request, next: Next) -> axum::response::Response {
    let mut res = next.run(req).await;
    let headers = res.headers_mut();

    // Prevent MIME type sniffing
    headers.insert(header::X_CONTENT_TYPE_OPTIONS, "nosniff".parse().unwrap());

    // Prevent clickjacking
    headers.insert(header::X_FRAME_OPTIONS, "DENY".parse().unwrap());

    // Enable XSS filter
    headers.insert("X-XSS-Protection", "1; mode=block".parse().unwrap());

    // Force HTTPS for 1 year
    headers.insert(
        header::STRICT_TRANSPORT_SECURITY,
        "max-age=31536000; includeSubDomains".parse().unwrap(),
    );

    // Control referrer information
    headers.insert(
        header::REFERRER_POLICY,
        "strict-origin-when-cross-origin".parse().unwrap(),
    );

    // Content Security Policy - controls what resources can be loaded
    let csp = [
        "default-src 'self'",
        "script-src 'self' 'unsafe-inline' https://unpkg.com https://cdn.jsdelivr.net",
        "style-src 'self' 'unsafe-inline' https://cdn.jsdelivr.net",
        "connect-src 'self' https://cdn.jsdelivr.net",
        "img-src 'self' data:",
        "font-src 'self' https://cdn.jsdelivr.net",
    ]
    .join("; ");

    headers.insert(header::CONTENT_SECURITY_POLICY, csp.parse().unwrap());

    // Restrict browser features
    headers.insert(
        "Permissions-Policy",
        "geolocation=(), microphone=(), camera=()".parse().unwrap(),
    );

    res
}
