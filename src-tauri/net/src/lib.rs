//! The internet lists are downloaded from, and where the latest release of the program is.
//!
//! This is a crate of its own because encryption needs a C compiler, while the rest of the code
//! has to be checked for all systems on one machine.

use std::time::Duration;

use adx0_core::{Failure, Web};

mod latest;
mod version;

pub use latest::Latest;
pub use version::Version;

/// No list is larger: this is a guard against an endless answer.
const LIMIT: u64 = 32 * 1024 * 1024;

/// The real internet. Certificates are checked against roots of the program and not of the
/// system: those of High Sierra are out of date.
pub struct Http;

impl Web for Http {
    fn text(&self, url: &str) -> Result<String, Failure> {
        let agent: ureq::Agent = ureq::Agent::config_builder()
            .timeout_global(Some(Duration::from_secs(60)))
            .build()
            .into();
        agent
            .get(url)
            .call()
            .and_then(|mut answer| {
                answer
                    .body_mut()
                    .with_config()
                    .limit(LIMIT)
                    .read_to_string()
            })
            .map_err(|failure| Failure::Silent {
                url: url.to_owned(),
                reason: failure.to_string(),
            })
    }
}

#[cfg(test)]
mod tests {
    use std::io::{Read, Write};
    use std::net::TcpListener;
    use std::thread;

    use adx0_core::Web;

    use super::Http;

    /// The address of a server on this computer that gives the answer once.
    fn served(answer: &'static str) -> String {
        let listener = TcpListener::bind("127.0.0.1:0").unwrap();
        let url = format!("http://{}/list", listener.local_addr().unwrap());
        thread::spawn(move || {
            let (mut stream, _) = listener.accept().unwrap();
            let _ = stream.read(&mut [0; 2048]).unwrap();
            stream.write_all(answer.as_bytes()).unwrap();
        });
        url
    }

    #[test]
    fn reads_text_from_address() {
        assert_eq!(
            Http.text(&served(
                "HTTP/1.1 200 OK\r\nContent-Length: 5\r\nConnection: close\r\n\r\na.com"
            ))
            .unwrap(),
            "a.com"
        );
    }

    #[test]
    fn reports_missing_page() {
        assert!(Http
            .text(&served(
                "HTTP/1.1 404 Not Found\r\nContent-Length: 0\r\nConnection: close\r\n\r\n"
            ))
            .is_err());
    }
}
