fn main () {
    let config = ScannerConfig::builder("./log")
                    .case_sensitive(true)
                    .max_file_size(4096)
                    .recursive(true)
                    .add_keyword("Secret")
                    .add_keyword("Password")
                    .add_keyword("Token")
                    .build();
    println!("{:#?}", config);
}

#[derive(Debug)]
struct ScannerConfig {
    path: String, 
    case_sensitive: bool,
    max_file_size: u64,
    recursive: bool,
    keywords: Vec<String>,
}

struct ScannerConfigBuilder {
    path: String, 
    case_sensitive: bool,
    max_file_size: u64,
    recursive: bool,
    keywords: Vec<String>,
}

impl ScannerConfig {
    fn builder(path: &str) -> ScannerConfigBuilder {
        ScannerConfigBuilder {
            path: path.to_string(),
            case_sensitive: false,
            max_file_size: 1024,
            recursive: false,
            keywords: Vec::new(),
        }
    }
}

impl ScannerConfigBuilder {
    fn case_sensitive(mut self, value: bool) -> Self {
        self.case_sensitive = value;
        self
    }
    fn max_file_size(mut self, value: u64) -> Self {
        self.max_file_size = value;
        self
    }
    fn recursive(mut self, value: bool) -> Self {
        self.recursive = value;
        self
    }
    fn add_keyword(mut self, value: &str) -> Self {
        self.keywords.push(value.to_string());
        self
    }

    fn build(self) -> ScannerConfig {
        ScannerConfig {
            path: self.path, 
            case_sensitive: self.case_sensitive,
            max_file_size: self.max_file_size,
            recursive: self.recursive,
            keywords: self.keywords,
        }
    }
}