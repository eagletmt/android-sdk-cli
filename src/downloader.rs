use crypto::digest::Digest;
use crypto::sha1::Sha1;
use hyper;
use std::io::{Read, Write};
use std::path::Path;
use std;
use tempfile;

#[derive(Debug)]
pub struct Downloader {
    client: hyper::Client,
}

impl Downloader {
    pub fn new(client: hyper::Client) -> Downloader {
        Downloader {
            client: client,
        }
    }

    pub fn download(&self, url: &str, expected_checksum: &str, dest_dir: &Path) -> Result<(), hyper::error::Error> {
        let mut response = try!(self.client.get(url).send());
        let mut zip_file = tempfile::NamedTempFile::new().unwrap();
        let mut sha1 = Sha1::new();

        let mut buf = [0; 2048];
        loop {
            match response.read(&mut buf) {
                Ok(0) => { break; }
                Ok(len) => {
                    let b = &buf[..len];
                    let _ = zip_file.write_all(b);
                    sha1.input(b);
                }
                Err(e) => { return Err(hyper::error::Error::Io(e)); }
            }
        }
        let actual_checksum = sha1.result_str();
        if actual_checksum != expected_checksum {
            return Err(hyper::error::Error::Io(std::io::Error::new(std::io::ErrorKind::Other, format!("Checksum failure: {}\nExpected: {}\nActual : {}", url, expected_checksum, actual_checksum))));
        }

        // TODO: Extract zip in Rust
        match std::process::Command::new("unzip").arg("-q").arg("-f").arg("-d").arg(dest_dir).arg(zip_file.path()).spawn().and_then(|mut child| child.wait()) {
            Ok(_) => {}
            Err(e) => { return Err(hyper::error::Error::Io(e)); }
        }
        return Ok(());
    }
}
