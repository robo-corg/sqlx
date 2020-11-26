use std::str::FromStr;

/// Options for controlling the desired security state of the
/// connection to the database server.
#[derive(Debug, PartialEq, Copy, Clone)]
pub enum SslMode {
    /// Establish an unencrypted connection.
    Disable,

    /// Establish an encrypted connection if the server supports encrypted connections,
    /// falling back to an unencrypted connection if an encrypted connection cannot be established.
    Prefer,

    /// Establish an encrypted connection if the server supports encrypted connections.
    /// The connection attempt fails if an encrypted connection cannot be established.
    Require,

    /// Like `Required`, but additionally verify the server Certificate Authority (CA) certificate
    /// against the configured CA certificates. The connection attempt fails if no valid
    /// matching CA certificates are found.
    VerifyCa,

    /// Like `VerifyCa`, but additionally perform host name identity verification by
    /// checking the host name the client uses for connecting to the server against the
    /// identity in the certificate that the server sends to the client.
    VerifyIdentity,
}

impl Default for SslMode {
    fn default() -> Self {
        return SslMode::Prefer;
    }
}

impl FromStr for SslMode {
    type Err = crate::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(match &*s.to_ascii_lowercase() {
            // NOTE: allow is explicitly set to disable as SQLx does not implement
            //       the allow mode; allow is only found in libpq.
            "disable" | "allow" => SslMode::Disable,
            "prefer" => SslMode::Prefer,
            "require" => SslMode::Require,
            "verify-ca" | "verify_ca" => SslMode::VerifyCa,
            "verify-full" | "verify-identity" | "verify_full" | "verify_identity" => {
                SslMode::VerifyIdentity
            }

            _ => {
                return Err(crate::Error::Configuration(
                    format!("unknown value {:?} for `ssl_mode`", s).into(),
                ));
            }
        })
    }
}

#[test]
fn test_parse_sql_mode() {
    for (text, expected) in &[
        ("disable", SslMode::Disable),
        ("prefer", SslMode::Prefer),
        ("require", SslMode::Require),
        ("verify-ca", SslMode::VerifyCa),
        ("verify-full", SslMode::VerifyIdentity),
        ("verify-identity", SslMode::VerifyIdentity),
    ] {
        assert_eq!(text.parse::<SslMode>().unwrap(), *expected);
    }
}
