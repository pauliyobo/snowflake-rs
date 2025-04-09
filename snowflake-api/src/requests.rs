use crate::converter::{SnowflakeType, ToSnowflakeType};
use serde::Serialize;
use std::collections::HashMap;

#[derive(Serialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct ExecRequest {
    pub sql_text: String,
    pub async_exec: bool,
    pub sequence_id: u64,
    pub is_internal: bool,
    pub bindings: HashMap<String, SnowflakeType>,
}

#[derive(Default)]
pub struct ExecRequestBuilder {
    sql_text: String,
    bindings: Vec<SnowflakeType>,
    is_internal: bool,
    sequence_id: u64,
    async_exec: bool,
}

impl ExecRequestBuilder {
    pub fn new(sql_text: &str) -> Self {
        Self {
            sql_text: sql_text.into(),
            ..Default::default()
        }
    }

    pub fn bind<T: ToSnowflakeType>(mut self, value: T) -> Self {
        self.bindings.push(value.to_snowflake());
        self
    }

    pub fn sequence_id(mut self, sequence_id: u64) -> Self {
        self.sequence_id = sequence_id;
        self
    }

    pub fn build(self) -> ExecRequest {
        // let's process the snowflake binding types
        let bindings = self
            .bindings
            .into_iter()
            .enumerate()
            .map(|(i, x)| ((i+1usize).to_string(), x))
            .collect::<HashMap<_, _>>();
        ExecRequest {
            sql_text: self.sql_text,
            async_exec: self.async_exec,
            is_internal: self.is_internal,
            sequence_id: self.sequence_id,
            bindings,
        }
    }
}

#[derive(Serialize, Debug)]
pub struct LoginRequest<T> {
    pub data: T,
}

pub type PasswordLoginRequest = LoginRequest<PasswordRequestData>;
#[cfg(feature = "cert-auth")]
pub type CertLoginRequest = LoginRequest<CertRequestData>;

#[derive(Serialize, Debug)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub struct LoginRequestCommon {
    pub client_app_id: String,
    pub client_app_version: String,
    pub svn_revision: String,
    pub account_name: String,
    pub login_name: String,
    pub session_parameters: SessionParameters,
    pub client_environment: ClientEnvironment,
}

#[derive(Serialize, Debug)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub struct SessionParameters {
    pub client_validate_default_parameters: bool,
}

#[derive(Serialize, Debug)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub struct ClientEnvironment {
    pub application: String,
    pub os: String,
    pub os_version: String,
    pub ocsp_mode: String,
}

#[derive(Serialize, Debug)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub struct PasswordRequestData {
    #[serde(flatten)]
    pub login_request_common: LoginRequestCommon,
    pub password: String,
}

#[derive(Serialize, Debug)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub struct CertRequestData {
    #[serde(flatten)]
    pub login_request_common: LoginRequestCommon,
    pub authenticator: String,
    pub token: String,
}

#[derive(Serialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct RenewSessionRequest {
    pub old_session_token: String,
    pub request_type: String,
}
