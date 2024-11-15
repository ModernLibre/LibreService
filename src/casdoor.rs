use casdoor_rust_sdk::CasdoorConfig;

pub fn create_casdoor_client() -> CasdoorConfig {
    let config_path = "casdoorConf.toml"; // 请根据实际路径修改

    CasdoorConfig::from_toml(config_path)
        .expect("Failed to load Casdoor configuration from conf.toml")
}
