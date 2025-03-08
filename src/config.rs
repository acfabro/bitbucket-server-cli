pub struct Config {
    pub server: String,
    pub api_token: String,
}

impl Config {
    pub fn new(server: String, api_token: String) -> Self {
        Config { server, api_token }
    }
}
