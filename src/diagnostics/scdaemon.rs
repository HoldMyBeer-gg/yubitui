use anyhow::Result;

#[derive(Debug, Clone)]
pub struct ScdaemonStatus {
    pub configured: bool,
    pub issues: Option<String>,
}

pub fn check_scdaemon() -> Result<ScdaemonStatus> {
    // Check if scdaemon is configured
    let config_path = crate::utils::config::scdaemon_conf().ok().filter(|p| p.exists());

    let configured = config_path.is_some();

    // Check for common issues
    let issues = if configured {
        // TODO: Parse scdaemon.conf and check for common misconfigurations
        None
    } else {
        Some("scdaemon.conf not found".to_string())
    };

    Ok(ScdaemonStatus { configured, issues })
}
