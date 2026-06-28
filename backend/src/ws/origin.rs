pub fn is_origin_allowed(
    origin_header: Option<&str>,
    config_base_url: &str,
    node_env: &str,
) -> bool {
    if node_env == "development" {
        return true;
    }
    let Some(origin) = origin_header else {
        return false;
    };
    origin.trim_end_matches('/') == config_base_url.trim_end_matches('/')
}
