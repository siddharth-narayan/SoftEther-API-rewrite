use std::sync::LazyLock;
use std::fmt::Arguments;
use std::io::IoSlice;
use std::io::{Read, Write};

use openssl::ssl::{Error, Ssl};
use openssl::ssl::SslConnector;
use openssl::ssl::SslContext;
use openssl::ssl::SslFiletype;
use openssl::ssl::SslMethod;
use openssl::ssl::SslStream;
use openssl::ssl::SslVerifyMode;
// use tokio::io::ReadHalf;
// use tokio::io::WriteHalf;
// use tokio::net::TcpStream;
// use tokio_openssl::SslStream;

// pub type SslRead = ReadHalf<SslStream<TcpStream>>;
// pub type SslWrite = WriteHalf<SslStream<TcpStream>>;

pub static SSL_CTX_CLIENT: LazyLock<SslContext> = LazyLock::new(create_client_ctx);
pub static SSL_CTX_SERVER: LazyLock<SslContext> = LazyLock::new(create_server_ctx);

pub enum SslUpgradable<S: Read + Write> {
    RawStream(S),
    SslStream(SslStream<S>),
}

impl<S: Read + Write> SslUpgradable<S> {
    pub fn is_encrypted(&self) -> bool {
        match self {
            Self::RawStream(_) => false,
            Self::SslStream(_) => true,
        }
    }

    pub fn is_unencrypted(&self) -> bool {
        !self.is_encrypted()
    }
}

impl<S: Read + Write> SslUpgradable<S> {
    pub fn upgrade(self, ssl: Ssl) -> Self {
        match self {
            SslUpgradable::RawStream(raw) => {
                Self::SslStream(SslStream::new(ssl, raw).unwrap())
            },
            
            Self::SslStream(s) => Self::SslStream(s),
        }
    }
}

impl<S: Read + Write> Read for SslUpgradable<S> {
    fn read(&mut self, buf: &mut[u8]) -> Result<usize, std::io::Error> {
        match self {
            SslUpgradable::RawStream(s) => s.read(buf),
            SslUpgradable::SslStream(s) => s.read(buf),
        }
    }
}

impl<S: Read + Write> Write for SslUpgradable<S> {
    fn write(&mut self, buf: &[u8]) -> Result<usize, std::io::Error> {
        match self {
            SslUpgradable::RawStream(s) => s.write(buf),
            SslUpgradable::SslStream(s) => s.write(buf),
        }
    }
    
    fn flush(&mut self) -> std::io::Result<()> {
        match self {
            SslUpgradable::RawStream(s) => s.flush(),
            SslUpgradable::SslStream(s) => s.flush(),
        }
    }

    // Provided methods
    fn write_vectored(&mut self, bufs: &[IoSlice<'_>]) -> std::io::Result<usize> {
        match self {
            SslUpgradable::RawStream(s) => s.write_vectored(bufs),
            SslUpgradable::SslStream(s) => s.write_vectored(bufs),
        }
    }
    
    // fn is_write_vectored(&self) -> bool {  false }
    
    fn write_all(&mut self, buf: &[u8]) -> std::io::Result<()> { 
        match self {
            SslUpgradable::RawStream(s) => s.write_all(buf),
            SslUpgradable::SslStream(s) => s.write_all(buf),
        }
    }
    
    // fn write_all_vectored(&mut self, bufs: &mut [IoSlice<'_>]) -> std::io::Result<()> {  Ok(())}
    
    fn write_fmt(&mut self, args: Arguments<'_>) -> std::io::Result<()> {
        match self {
            SslUpgradable::RawStream(s) => s.write_fmt(args),
            SslUpgradable::SslStream(s) => s.write_fmt(args),
        }
    }
    
    fn by_ref(&mut self) -> &mut Self
       where Self: Sized { self }
}

pub struct SslVerifyOption {

}

pub fn create_server_ctx() -> SslContext {
    let mut builder = SslConnector::builder(SslMethod::tls()).expect("Failed to create server SSLContext");;
    builder.set_verify(SslVerifyMode::NONE);
    builder.set_certificate_file("cert.pem", SslFiletype::PEM).expect("Failed to set SSLContext certificate");;
    builder.set_private_key_file("key.pem", SslFiletype::PEM).expect("Failed to set SSLContext private key");;

    builder.build().into_context()
}

pub fn create_client_ctx() -> SslContext {
    let mut builder = SslConnector::builder(SslMethod::tls()).expect("Failed to create client SSLContext");
    builder.set_verify(SslVerifyMode::NONE);

    builder.build().into_context()
}
