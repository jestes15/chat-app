use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct NetworkData {
    pub ip_address: String,
    pub listening_port: String
}

#[derive(Debug, Deserialize)]
pub struct Production {
    pub outside_network: NetworkData,
    pub inside_network: NetworkData
}

#[derive(Debug, Deserialize)]
pub struct Configuration {
    pub test: NetworkData,
    pub production: Production
}

#[derive(Debug, Deserialize)]
pub struct ConfigurationFile {
    pub config_1: Configuration,
    pub config_2: Configuration
}