fn main() {
    let tar_scanner = TarScanner;
    let zip_scanner = ZipScanner;
    let zstd_scanner = ZstdScanner;

    run_scanner(&tar_scanner, "Archive.tar");
    run_scanner(&zip_scanner, "Archive.zip");
    run_scanner(&zstd_scanner, "Archive.zstd");
}

fn run_scanner(scanner: &dyn FileScanner, file: &str) {
    scanner.scan(file);
}

struct TarScanner;
struct ZipScanner;
struct ZstdScanner;

trait FileScanner {
    fn scan(&self, file: &str);
}

impl FileScanner for TarScanner {
    fn scan(&self, file: &str) {
        println!("Scanning tar file: {}", file);
    }
}

impl FileScanner for ZipScanner {
    fn scan(&self, file: &str) {
        println!("Scanning zip file: {}", file);
    }
}

impl FileScanner for ZstdScanner {
    fn scan(&self, file: &str) {
        println!("Scanning zstd file: {}", file);
    }
}
