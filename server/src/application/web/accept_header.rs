use regex::Regex;

use crate::application::config::version::ApiVersion;
use crate::application::web::filters::validate::api::error::ApiValidationError;

#[derive(Debug, Clone)]
pub(crate) struct AcceptHeader {
    api_version: ApiVersion,
}

impl AcceptHeader {
    fn new(api_version: ApiVersion) -> Self {
        AcceptHeader { api_version }
    }

    pub(crate) fn parse<S: AsRef<str>>(
        accept_string: S,
    ) -> Result<AcceptHeader, ApiValidationError> {
        let api_version = extract_api_version_from_accept_header(accept_string)?;
        Ok(AcceptHeader::new(api_version))
    }

    pub(crate) fn api_version(&self) -> &ApiVersion {
        &self.api_version
    }
}

fn extract_api_version_from_accept_header<T: AsRef<str>>(
    accept_header: T,
) -> Result<ApiVersion, ApiValidationError> {
    let re: Regex = Regex::new(r"(?i)application/vnd\.warpj\.(v\d+)\+?\w*").unwrap();
    let result = re.captures(accept_header.as_ref());

    match result {
        Some(captures) => match captures.get(1) {
            Some(m) => {
                let s = m.as_str();
                let api_version = s.parse::<ApiVersion>()?;
                Ok(api_version)
            }
            None => Err(ApiValidationError::MissingMatch),
        },
        None => Err(ApiValidationError::MissingMatch),
    }
}

#[cfg(test)]
mod tests {
    use spectral::prelude::*;

    use super::*;

    #[test]
    fn finds_api_version_in_valid_header_string() {
        let header_string = "application/vnd.warpj.v12+text";

        asserting("finds api version in valid header string")
            .that(&AcceptHeader::parse(header_string))
            .is_ok();
    }

    #[test]
    fn errors_if_no_version_present() {
        let header_string = "application/vnd.warpj.+text";

        asserting("errors if no api version present")
            .that(&AcceptHeader::parse(header_string))
            .is_err_containing(ApiValidationError::MissingMatch);
    }

    #[test]
    fn errors_if_malformed_header_string() {
        let header_string = "application/vnd.warpjv12+text";

        asserting("errors when parsing a malformed header string")
            .that(&AcceptHeader::parse(header_string))
            .is_err_containing(ApiValidationError::MissingMatch);
    }

    #[test]
    fn errors_if_contains_non_digit_version_string() {
        let header_string = "application/vnd.warpj.vtwelve+text";

        asserting("errors when parsing a non-digit version string")
            .that(&AcceptHeader::parse(header_string))
            .is_err_containing(ApiValidationError::MissingMatch);
    }
}