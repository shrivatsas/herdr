//! Build identity helpers.

pub const BASE_VERSION: &str = env!("CARGO_PKG_VERSION");

pub fn channel() -> &'static str {
    non_empty(option_env!("HERDR_BUILD_CHANNEL")).unwrap_or("stable")
}

pub fn build_id() -> Option<&'static str> {
    non_empty(option_env!("HERDR_BUILD_ID"))
}

pub fn version() -> String {
    format_version(BASE_VERSION, channel(), build_id())
}

fn format_version(base_version: &str, channel: &str, build_id: Option<&str>) -> String {
    match channel {
        "stable" => base_version.to_string(),
        channel => match build_id {
            Some(build_id) => format!("{base_version}-{channel}.{build_id}"),
            None => format!("{base_version}-{channel}"),
        },
    }
}

pub fn is_preview() -> bool {
    channel() == "preview"
}

fn non_empty(value: Option<&'static str>) -> Option<&'static str> {
    value.and_then(|value| {
        let trimmed = value.trim();
        if trimmed.is_empty() {
            None
        } else {
            Some(trimmed)
        }
    })
}

#[cfg(test)]
mod tests {
    use super::format_version;

    #[test]
    fn stable_version_defaults_to_cargo_version() {
        assert!(!super::version().is_empty());
    }

    #[test]
    fn stable_channel_ignores_build_id() {
        assert_eq!(format_version("0.8.2", "stable", Some("9d8bc4e4")), "0.8.2");
    }

    #[test]
    fn dev_channel_formats_with_git_sha_build_id() {
        assert_eq!(
            format_version("0.8.2", "dev", Some("9d8bc4e4")),
            "0.8.2-dev.9d8bc4e4"
        );
    }

    #[test]
    fn dev_channel_without_build_id_falls_back_to_bare_channel_suffix() {
        assert_eq!(format_version("0.8.2", "dev", None), "0.8.2-dev");
    }

    #[test]
    fn preview_channel_formatting_is_unchanged() {
        assert_eq!(
            format_version("0.8.2", "preview", Some("42")),
            "0.8.2-preview.42"
        );
    }
}
