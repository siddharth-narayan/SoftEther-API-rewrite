use std::fmt::Arguments;
use std::io::IoSlice;
use std::io::{Read, Write};
use std::sync::LazyLock;

use openssl::ssl::SslConnector;
use openssl::ssl::SslContext;
use openssl::ssl::SslFiletype;
use openssl::ssl::SslMethod;
use openssl::ssl::SslStream;
use openssl::ssl::SslVerifyMode;
use openssl::ssl::{Error, Ssl};
// use tokio::io::ReadHalf;
// use tokio::io::WriteHalf;
// use tokio::net::TcpStream;
// use tokio_openssl::SslStream;

// pub type SslRead = ReadHalf<SslStream<TcpStream>>;
// pub type SslWrite = WriteHalf<SslStream<TcpStream>>;

pub static SSL_CTX_CLIENT: LazyLock<SslContext> = LazyLock::new(create_client_ctx);
pub static SSL_CTX_SERVER: LazyLock<SslContext> = LazyLock::new(create_server_ctx);

static SSL_UPGRADEABLE_INTERNAL_PANIC_MSG: &str =
    "SslUpgradable had an internal None, which is an unexpected state";

pub struct SslUpgradable<S: Read + Write> {
    _internal: Option<_SslUpgradable<S>>,
}

impl<S: Read + Write> SslUpgradable<S> {
    pub fn new(socket: S) -> Self {
        Self {
            _internal: Some(_SslUpgradable::RawStream(socket)),
        }
    }

    pub fn upgrade(&mut self, ssl: Ssl) {
        let upgradeable = self._internal.take();

        // Is this a reasonable place to panic?
        let upgradeable = upgradeable.expect(SSL_UPGRADEABLE_INTERNAL_PANIC_MSG);

        self._internal = Some(upgradeable.upgrade(ssl));
    }

    // pub fn peek(&mut self) {
    //     self._internal.unwrap().
    // }
}

impl<S: Read + Write> Read for SslUpgradable<S> {
    fn read(&mut self, buf: &mut [u8]) -> Result<usize, std::io::Error> {
        self._internal
            .as_mut()
            .expect(SSL_UPGRADEABLE_INTERNAL_PANIC_MSG)
            .read(buf)
    }
}

impl<S: Read + Write> Write for SslUpgradable<S> {
    fn write(&mut self, buf: &[u8]) -> Result<usize, std::io::Error> {
        self._internal
            .as_mut()
            .expect(SSL_UPGRADEABLE_INTERNAL_PANIC_MSG)
            .write(buf)
    }

    fn flush(&mut self) -> std::io::Result<()> {
        self._internal
            .as_mut()
            .expect(SSL_UPGRADEABLE_INTERNAL_PANIC_MSG)
            .flush()
    }

    // Provided methods
    fn write_vectored(&mut self, bufs: &[IoSlice<'_>]) -> std::io::Result<usize> {
        self._internal
            .as_mut()
            .expect(SSL_UPGRADEABLE_INTERNAL_PANIC_MSG)
            .write_vectored(bufs)
    }

    // fn is_write_vectored(&self) -> bool {  false }

    fn write_all(&mut self, buf: &[u8]) -> std::io::Result<()> {
        self._internal
            .as_mut()
            .expect(SSL_UPGRADEABLE_INTERNAL_PANIC_MSG)
            .write_all(buf)
    }

    // fn write_all_vectored(&mut self, bufs: &mut [IoSlice<'_>]) -> std::io::Result<()> {  Ok(())}

    fn write_fmt(&mut self, args: Arguments<'_>) -> std::io::Result<()> {
        self._internal
            .as_mut()
            .expect(SSL_UPGRADEABLE_INTERNAL_PANIC_MSG)
            .write_fmt(args)
    }

    fn by_ref(&mut self) -> &mut Self
    where
        Self: Sized,
    {
        self
    }
}

// ==================================================
// ======== _SslUpgradeable  Internal API ===========
// ==================================================

enum _SslUpgradable<S: Read + Write> {
    RawStream(S),
    SslStream(SslStream<S>),
}

impl<S: Read + Write> _SslUpgradable<S> {
    fn is_encrypted(&self) -> bool {
        match self {
            Self::RawStream(_) => false,
            Self::SslStream(_) => true,
        }
    }

    pub fn is_unencrypted(&self) -> bool {
        !self.is_encrypted()
    }

    pub fn upgrade(self, ssl: Ssl) -> Self {
        match self {
            Self::RawStream(raw) => Self::SslStream(SslStream::new(ssl, raw).unwrap()),

            Self::SslStream(s) => Self::SslStream(s),
        }
    }
}

impl<S: Read + Write> Read for _SslUpgradable<S> {
    fn read(&mut self, buf: &mut [u8]) -> Result<usize, std::io::Error> {
        match self {
            Self::RawStream(s) => s.read(buf),
            Self::SslStream(s) => s.read(buf),
        }
    }
}

impl<S: Read + Write> Write for _SslUpgradable<S> {
    fn write(&mut self, buf: &[u8]) -> Result<usize, std::io::Error> {
        match self {
            Self::RawStream(s) => s.write(buf),
            Self::SslStream(s) => s.write(buf),
        }
    }

    fn flush(&mut self) -> std::io::Result<()> {
        match self {
            Self::RawStream(s) => s.flush(),
            Self::SslStream(s) => s.flush(),
        }
    }

    // Provided methods
    fn write_vectored(&mut self, bufs: &[IoSlice<'_>]) -> std::io::Result<usize> {
        match self {
            Self::RawStream(s) => s.write_vectored(bufs),
            Self::SslStream(s) => s.write_vectored(bufs),
        }
    }

    // fn is_write_vectored(&self) -> bool {  false }

    fn write_all(&mut self, buf: &[u8]) -> std::io::Result<()> {
        match self {
            Self::RawStream(s) => s.write_all(buf),
            Self::SslStream(s) => s.write_all(buf),
        }
    }

    // fn write_all_vectored(&mut self, bufs: &mut [IoSlice<'_>]) -> std::io::Result<()> {  Ok(())}

    fn write_fmt(&mut self, args: Arguments<'_>) -> std::io::Result<()> {
        match self {
            Self::RawStream(s) => s.write_fmt(args),
            Self::SslStream(s) => s.write_fmt(args),
        }
    }

    fn by_ref(&mut self) -> &mut Self
    where
        Self: Sized,
    {
        self
    }
}

pub struct SslVerifyOption {}

pub fn create_server_ctx() -> SslContext {
    let mut builder =
        SslConnector::builder(SslMethod::tls()).expect("Failed to create server SSLContext");
    builder.set_verify(SslVerifyMode::NONE);
    builder
        .set_certificate_file("cert.pem", SslFiletype::PEM)
        .expect("Failed to set SSLContext certificate");
    builder
        .set_private_key_file("key.pem", SslFiletype::PEM)
        .expect("Failed to set SSLContext private key");

    builder.build().into_context()
}

pub fn create_client_ctx() -> SslContext {
    let mut builder =
        SslConnector::builder(SslMethod::tls()).expect("Failed to create client SSLContext");
    builder.set_verify(SslVerifyMode::NONE);

    builder.build().into_context()
}
