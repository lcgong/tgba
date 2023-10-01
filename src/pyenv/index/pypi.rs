#[derive(Clone)]
pub struct PyPI {
    name: String,
    url: String,
}

impl PyPI {
    pub fn new(name: &str, url: &str) -> Self {
        let url = if url.ends_with('/') {
            url.to_string()
        } else {
            format!("{}/", url)
        };

        PyPI {
            name: name.to_string(),
            url,
        }
    }

    pub fn name(&self) -> &str {
        &self.name
    }

    pub fn url(&self) -> &str {
        &self.url
    }

    pub fn package_url(&self, canonical_name: &str) -> String {
        format!("{}{}/", self.url, canonical_name)
    }
}