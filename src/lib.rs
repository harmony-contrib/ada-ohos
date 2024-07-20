use ada_url::Url as AUrl;
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

#[napi]
impl Ada {
    /// parse url
    #[napi]
    pub fn parse(url: String) -> Result<Url> {
        let ada = AUrl::parse(url, None)
            .map_err(|e| Error::new(Status::GenericFailure, e.to_string()))?;
        Ok(Url { inner: ada })
    }

    /// check url can be parsed
    #[napi]
    pub fn can_parse(url: String) -> bool {
        AUrl::can_parse(url.as_ref(), None)
    }
}

#[napi]
impl Url {
    #[napi(getter)]
    pub fn host(&self) -> &str {
        self.inner.host()
    }

    #[napi(getter)]
    pub fn hash(&self) -> &str {
        self.inner.hash()
    }

    #[napi(getter)]
    pub fn hostname(&self) -> &str {
        self.inner.hostname()
    }

    #[napi(getter)]
    pub fn pathname(&self) -> &str {
        self.inner.pathname()
    }

    #[napi(getter)]
    pub fn search(&self) -> &str {
        self.inner.search()
    }

    #[napi(getter)]
    pub fn protocol(&self) -> &str {
        self.inner.protocol()
    }

    #[napi(getter)]
    pub fn href(&self) -> &str {
        self.inner.href()
    }

    #[napi(getter)]
    pub fn username(&self) -> &str {
        self.inner.username()
    }

    #[napi(getter)]
    pub fn password(&self) -> &str {
        self.inner.password()
    }

    #[napi(getter)]
    pub fn port(&self) -> &str {
        self.inner.port()
    }

    #[napi(getter)]
    pub fn origin(&self) -> String {
        self.inner.origin()
    }
}
