//! The httpOnly refresh-token cookie, emitted directly by the public auth
//! handlers.
//!
//! No cookie middleware lives in this tree (and none is needed): the
//! handlers build `Set-Cookie` and parse `Cookie` themselves. The
//! attributes are the hardening set: HttpOnly (page scripts can never read
//! the credential), Secure, SameSite=Strict, and a Path the HOST pins to
//! the mount base it chose for the public auth router (see the mounting
//! contract on [`crate::presentation::http::public_auth_routes`]) — the
//! cookie never rides any other request path. A session cookie (no
//! Max-Age): closing the browser ends the session.
//!
//! Same-site law for deployers: the browser origin must be SAME-SITE with
//! the API origin or a Strict cookie is silently never sent — sibling
//! subdomains qualify, `localhost` versus `127.0.0.1` does not.

use axum::http::{header, HeaderMap, HeaderValue};

/// The cookie carrying the refresh token.
pub const REFRESH_COOKIE_NAME: &str = "sapiens_refresh";

/// Build the `Set-Cookie` header value that arms the refresh cookie under
/// the given path (the host's mount base for the auth router). A remembered
/// session (keep me signed in) passes `Some(max_age_secs)`: the cookie then
/// survives browser restarts for as long as the refresh token lives.
pub fn refresh_cookie_header(value: &str, path: &str, max_age_secs: Option<i64>) -> HeaderValue {
    match max_age_secs {
        Some(max_age) => {
            format!("{REFRESH_COOKIE_NAME}={value}; Path={path}; HttpOnly; Secure; SameSite=Strict; Max-Age={max_age}")
        }
        None => format!("{REFRESH_COOKIE_NAME}={value}; Path={path}; HttpOnly; Secure; SameSite=Strict"),
    }
    .parse()
    .expect("cookie header with caller-supplied path always parses")
}

/// Build the `Set-Cookie` header value that clears the refresh cookie.
pub fn clear_refresh_cookie_header(path: &str) -> HeaderValue {
    format!("{REFRESH_COOKIE_NAME}=; Path={path}; HttpOnly; Secure; SameSite=Strict; Max-Age=0")
        .parse()
        .expect("cookie header with caller-supplied path always parses")
}

/// Read one cookie's value off a `Cookie` header. Refresh tokens are
/// base64url, so the simple `name=value` scan is exact for this cookie.
pub fn cookie_value(headers: &HeaderMap, name: &str) -> Option<String> {
    let raw = headers.get(header::COOKIE)?.to_str().ok()?;
    raw.split(';')
        .map(str::trim)
        .find_map(|part| part.strip_prefix(&format!("{name}=")).map(str::to_string))
}

#[cfg(test)]
mod tests {
    use super::*;
    use axum::http::HeaderMap;

    #[test]
    fn refresh_cookie_carries_the_hardening_attributes() {
        let raw = refresh_cookie_header("jwt-value", "/api/v1/auth", None)
            .to_str()
            .unwrap()
            .to_string();
        assert!(raw.starts_with("sapiens_refresh=jwt-value;"));
        assert!(raw.contains("Path=/api/v1/auth"));
        assert!(raw.contains("HttpOnly"));
        assert!(raw.contains("Secure"));
        assert!(raw.contains("SameSite=Strict"));
        assert!(!raw.contains("Max-Age"));
    }

    #[test]
    fn remembered_cookie_carries_the_refresh_lifetime() {
        let raw = refresh_cookie_header("jwt-value", "/api/v1/auth", Some(2_592_000))
            .to_str()
            .unwrap()
            .to_string();
        assert!(raw.contains("Max-Age=2592000"));
        for attr in ["HttpOnly", "Secure", "SameSite=Strict", "Path=/api/v1/auth"] {
            assert!(raw.contains(attr), "missing `{attr}`: {raw:?}");
        }
    }

    #[test]
    fn clear_cookie_expires_immediately() {
        let raw = clear_refresh_cookie_header("/api/v1/auth")
            .to_str()
            .unwrap()
            .to_string();
        assert!(raw.starts_with("sapiens_refresh=;"));
        assert!(raw.contains("Max-Age=0"));
    }

    #[test]
    fn cookie_value_reads_through_a_cookie_header() {
        let mut headers = HeaderMap::new();
        headers.insert(
            header::COOKIE,
            "other=1; sapiens_refresh=abc.def.ghi; again=2"
                .parse()
                .unwrap(),
        );
        assert_eq!(
            cookie_value(&headers, REFRESH_COOKIE_NAME).as_deref(),
            Some("abc.def.ghi")
        );
        assert_eq!(cookie_value(&headers, "absent"), None);
    }
}
