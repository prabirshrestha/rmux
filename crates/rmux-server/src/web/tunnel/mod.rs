mod preset;
mod runner;

use rmux_proto::RmuxError;

use super::settings::WebShareSettings;

pub(crate) use runner::TunnelHandle;

#[derive(Debug)]
pub(crate) struct TunnelInfo {
    pub(crate) handle: TunnelHandle,
    pub(crate) provider: String,
    pub(crate) public_url: String,
}

pub(crate) async fn start_provider(
    name: &str,
    settings: &WebShareSettings,
) -> Result<TunnelInfo, RmuxError> {
    let preset = preset::load(name)?;
    runner::start(preset, settings).await
}

#[cfg(test)]
mod tests {
    use super::preset::{available_from, parse, PresetSource};

    #[test]
    fn embedded_presets_are_valid() {
        for (name, content) in super::preset::embedded() {
            parse(name, PresetSource::Embedded, content).expect("embedded preset parses");
        }
    }

    #[test]
    fn available_presets_are_sorted_and_unique() {
        let names = available_from([("b".to_owned(), ""), ("a".to_owned(), "")], Vec::new());
        assert_eq!(names, vec!["a", "b"]);
    }

    #[test]
    fn devtunnel_url_pattern_matches_browser_url_not_inspect() {
        let content = super::preset::embedded()
            .iter()
            .find(|(name, _)| *name == "devtunnel")
            .map(|(_, content)| *content)
            .expect("devtunnel preset is embedded");
        let preset = parse("devtunnel", PresetSource::Embedded, content).expect("parses");
        let regex = regex::Regex::new(&preset.url_pattern).expect("valid url_pattern");

        // "Connect via browser:" line — the URL we want.
        assert_eq!(
            regex
                .find("Connect via browser: https://b42v28rg-9777.usw2.devtunnels.ms")
                .map(|m| m.as_str()),
            Some("https://b42v28rg-9777.usw2.devtunnels.ms")
        );
        // Custom/named tunnel ids contain hyphens and must still match.
        assert_eq!(
            regex
                .find("https://my-rmux-tunnel-9777.euw.devtunnels.ms")
                .map(|m| m.as_str()),
            Some("https://my-rmux-tunnel-9777.euw.devtunnels.ms")
        );
        // The "Inspect network activity:" URL must NOT be matched.
        assert!(regex
            .find("Inspect network activity: https://b42v28rg-9777-inspect.usw2.devtunnels.ms")
            .is_none());
    }
}
