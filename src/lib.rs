use ada_url::{HostType as AHostType, SchemeType as ASchemeType, Url as AUrl};
use napi_derive_ohos::napi;
use napi_ohos::{Error, Result, Status};

/// ada-url wrapper support parse and canParse method.
#[napi]
pub struct Ada();

/// url instance wrapper, can directly get host,protocol,etc.
#[napi]
pub struct Url {
    inner: AUrl,
}

/// SchemeType Enum which if copied from ada-url
#[napi]
pub enum SchemeType {
    Http = 0,
    NotSpecial = 1,
    Https = 2,
    Ws = 3,
    Ftp = 4,
    Wss = 5,
    File = 6,
}

impl From<ASchemeType> for SchemeType {
    fn from(value: ASchemeType) -> Self {
        match value {
            ASchemeType::File => SchemeType::File,
            ASchemeType::Http => SchemeType::Http,
            ASchemeType::NotSpecial => SchemeType::NotSpecial,
            ASchemeType::Https => SchemeType::Https,
            ASchemeType::Ws => SchemeType::Ws,
            ASchemeType::Ftp => SchemeType::Ftp,
            ASchemeType::Wss => SchemeType::Wss,
        }
    }
}

/// HostType Enum which if copied from ada-url
#[napi]
pub enum HostType {
    Domain = 0,
    IPV4 = 1,
    IPV6 = 2,
}

impl From<AHostType> for HostType {
    fn from(value: AHostType) -> Self {
        match value {
            AHostType::Domain => HostType::Domain,
            AHostType::IPV4 => HostType::IPV4,
            AHostType::IPV6 => HostType::IPV6,
        }
    }
}

#[napi]
impl Ada {
    /// parse url
    #[napi]
    pub fn parse(url: String, base: Option<String>) -> Result<Url> {
        let ada = AUrl::parse(url, base.as_deref())
            .map_err(|e| Error::new(Status::GenericFailure, e.to_string()))?;
        Ok(Url { inner: ada })
    }

    /// check url can be parsed
    #[napi]
    pub fn can_parse(url: String, base: Option<String>) -> bool {
        AUrl::can_parse(url.as_ref(), base.as_deref())
    }
}

#[napi]
impl Url {
    #[napi(getter)]
    pub fn host(&self) -> &str {
        self.inner.host()
    }

    #[napi(setter)]
    pub fn set_host(&mut self, host: String) -> Result<()> {
        self.inner
            .set_host(Some(host.as_str()))
            .map_err(|_| Error::new(Status::GenericFailure, "set host failed"))?;
        Ok(())
    }

    #[napi(getter)]
    pub fn hash(&self) -> &str {
        self.inner.hash()
    }

    #[napi(setter)]
    pub fn set_hash(&mut self, hash: String) {
        self.inner.set_hash(Some(hash.as_str()));
    }

    #[napi(getter)]
    pub fn hostname(&self) -> &str {
        self.inner.hostname()
    }

    #[napi(setter)]
    pub fn set_hostname(&mut self, hosttname: String) -> Result<()> {
        self.inner
            .set_hostname(Some(hosttname.as_str()))
            .map_err(|_| Error::new(Status::GenericFailure, "set hostname failed"))?;
        Ok(())
    }

    #[napi(getter)]
    pub fn pathname(&self) -> &str {
        self.inner.pathname()
    }

    #[napi(setter)]
    pub fn set_pathname(&mut self, pathtname: String) -> Result<()> {
        self.inner
            .set_pathname(Some(pathtname.as_str()))
            .map_err(|_| Error::new(Status::GenericFailure, "set pathname failed"))?;
        Ok(())
    }

    #[napi(getter)]
    pub fn search(&self) -> &str {
        self.inner.search()
    }

    #[napi(setter)]
    pub fn set_search(&mut self, search: String) {
        self.inner.set_search(Some(search.as_str()));
    }

    #[napi(getter)]
    pub fn protocol(&self) -> &str {
        self.inner.protocol()
    }

    #[napi(setter)]
    pub fn set_protocol(&mut self, protocol: String) -> Result<()> {
        self.inner
            .set_protocol(protocol.as_str())
            .map_err(|_| Error::new(Status::GenericFailure, "set protocol failed"))?;
        Ok(())
    }

    #[napi(getter)]
    pub fn href(&self) -> &str {
        self.inner.href()
    }

    #[napi(setter)]
    pub fn set_href(&mut self, href: String) -> Result<()> {
        self.inner
            .set_href(href.as_str())
            .map_err(|_| Error::new(Status::GenericFailure, "set href failed"))?;
        Ok(())
    }

    #[napi(getter)]
    pub fn username(&self) -> &str {
        self.inner.username()
    }

    #[napi(setter)]
    pub fn set_username(&mut self, username: String) -> Result<()> {
        self.inner
            .set_username(Some(username.as_str()))
            .map_err(|_| Error::new(Status::GenericFailure, "set username failed"))?;
        Ok(())
    }

    #[napi(getter)]
    pub fn password(&self) -> &str {
        self.inner.password()
    }

    #[napi(setter)]
    pub fn set_password(&mut self, password: String) -> Result<()> {
        self.inner
            .set_password(Some(password.as_str()))
            .map_err(|_| Error::new(Status::GenericFailure, "set password failed"))?;
        Ok(())
    }

    #[napi(getter)]
    pub fn port(&self) -> &str {
        self.inner.port()
    }

    #[napi(setter)]
    pub fn set_port(&mut self, port: String) -> Result<()> {
        self.inner
            .set_port(Some(port.as_str()))
            .map_err(|_| Error::new(Status::GenericFailure, "set port failed"))?;
        Ok(())
    }

    #[napi(getter)]
    pub fn origin(&self) -> String {
        self.inner.origin()
    }

    #[napi(getter)]
    pub fn schema_type(&self) -> SchemeType {
        self.inner.scheme_type().into()
    }

    #[napi(getter)]
    pub fn host_type(&self) -> HostType {
        self.inner.host_type().into()
    }
}
