use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Copy, PartialEq, Serialize, Deserialize)]
pub enum GrantType {
    #[serde(rename = "authorization_code")]
    AuthorizationCode,
    #[serde(rename = "client_credentials")]
    ClientCredentials,
}

/// serializes to space seperated list of scopes
///
/// https://docs.joinmastodon.org/api/oauth-scopes/
#[derive(Default, Debug, Clone, Copy, PartialEq)]
pub struct Scopes {
    /// Grants access to read data, including other users.
    /// Requesting read will also grant a set of [granular scopes](https://docs.joinmastodon.org/api/oauth-scopes/#granular)
    pub read: bool,
    /// Grants access to write data. Requesting write will also grant a set of [granular scopes](https://docs.joinmastodon.org/api/oauth-scopes/#granular)
    pub write: bool,
    /// Grants access to [Web Push API subscriptions](https://docs.joinmastodon.org/methods/push/).
    /// Added in Mastodon 2.4.0.
    pub push: bool,
    /// Grants access only to the
    /// [GET /api/v1/accounts/verify_credentials](https://docs.joinmastodon.org/methods/accounts/#verify_credentials)
    /// endpoint. Allowing you to retrieve information about only the currently authenticated user.
    pub profile: bool,
    /// Used for administrative and moderation APIs. Added in Mastodon 2.9.1.
    /// Requesting admin:read or admin:write will also grant granular scopes shown in [granular scopes](https://docs.joinmastodon.org/api/oauth-scopes/#granular)
    ///
    /// Note that there is no singular admin scope available.
    pub admin_read: bool,
    /// see [`Scopes::admin_read`]
    pub admin_write: bool,
}
impl Scopes {
    pub fn set_read(mut self, to: bool) -> Self {
        self.read = to;
        self
    }
    pub fn set_write(mut self, to: bool) -> Self {
        self.write = to;
        self
    }
    pub fn set_push(mut self, to: bool) -> Self {
        self.push = to;
        self
    }
    pub fn set_profile(mut self, to: bool) -> Self {
        self.profile = to;
        self
    }
    pub fn set_admin_read(mut self, to: bool) -> Self {
        self.admin_read = to;
        self
    }
    pub fn set_admin_write(mut self, to: bool) -> Self {
        self.admin_write = to;
        self
    }
}

impl Serialize for Scopes {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        let mut out: Vec<String> = Vec::with_capacity(6);
        if self.read {
            out.push("read".to_string());
        }
        if self.write {
            out.push("write".to_string());
        }
        if self.push {
            out.push("push".to_string());
        }
        if self.profile {
            out.push("profile".to_string());
        }
        if self.admin_read {
            out.push("admin:read".to_string());
        }
        if self.admin_write {
            out.push("admin:write".to_string());
        }
        serializer.serialize_str(&out.join(" "))
    }
}

impl<'de> Deserialize<'de> for Scopes {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        let input = <&str>::deserialize(deserializer)?;
        let mut out = Scopes::default();
        for scope in input.split_whitespace() {
            match scope {
                "read" => out.read = true,
                "write" => out.write = true,
                "push" => out.push = true,
                "profile" => out.profile = true,
                "admin:read" => out.admin_read = true,
                "admin:write" => out.admin_write = true,
                _ => {}
            }
        }

        Ok(out)
    }
}
