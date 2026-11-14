fn main () {
    let config = HttpClientConfig::builder("https://api.example.com")
                    .timeout(30)
                    .retries(3)
                    .use_tls(true)
                    .build();
        println!("{:#?}", config);
    
}

#[derive(Debug)]
struct HttpClientConfig {
    base_url: String,
    timeout: u32,
    retries: u32,
    use_tls: bool,
}

struct HttpClientConfigBuilder {
    base_url: String,
    timeout: u32,
    retries: u32,
    use_tls: bool,
}

impl HttpClientConfig {
    fn builder(base_url: &str) -> HttpClientConfigBuilder {
        HttpClientConfigBuilder {
            base_url: base_url.to_string(),
            timeout: 0,
            retries: 0,
            use_tls: false
        }
    }
}


impl HttpClientConfigBuilder {
    fn timeout(mut self, value: u32) -> Self {
        self.timeout = value;
        self
    }
    fn retries(mut self, value: u32) -> Self {
        self.retries = value;
        self
    }
    fn use_tls(mut self, value: bool) -> Self {
        self.use_tls = value;
        self
    }
    fn build(self) -> HttpClientConfig {
        HttpClientConfig {
            base_url: self.base_url,
            timeout: self.timeout,
            retries: self.retries,
            use_tls: self.use_tls,
        }
    }
}