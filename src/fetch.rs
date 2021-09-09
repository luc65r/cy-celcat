use lazy_static::lazy_static;
use log::{debug, info};
use regex::Regex;
use serde::Serialize;
use thiserror::Error;

use crate::fetchable::Fetchable;

#[derive(Error, Debug)]
pub enum FetchError {
    #[error("request error")]
    Reqwest(#[from] reqwest::Error),
    #[error("cannot find the token")]
    Token,
}

#[derive(Debug, Clone)]
pub struct Celcat {
    client: reqwest::Client,
    address: String,
    token: String,
}

impl Celcat {
    pub async fn new<S>(address: S) -> Result<Self, FetchError>
    where
        S: AsRef<str>,
    {
        let client = reqwest::Client::builder().cookie_store(true).build()?;

        let token = Self::fetch_token(&client, address.as_ref()).await?;

        Ok(Self {
            client,
            address: address.as_ref().to_owned(),
            token,
        })
    }

    async fn fetch_token(client: &reqwest::Client, address: &str) -> Result<String, FetchError> {
        lazy_static! {
            static ref RE: Regex =
                Regex::new(r#"<input name="__RequestVerificationToken".*?value="([^"]+)""#)
                    .unwrap();
        }
        info!("fetching celcat token");
        let body = client
            .get(&format!("{}/LdapLogin", address))
            .send()
            .await?
            .text()
            .await?;

        if let Some(token) = RE.captures(&body).and_then(|caps| caps.get(1)) {
            Ok(token.as_str().to_owned())
        } else {
            Err(FetchError::Token)
        }
    }

    pub async fn login(&mut self, username: &str, password: &str) -> Result<(), FetchError> {
        #[derive(Debug, Serialize)]
        struct Form<'a> {
            #[serde(rename = "Name")]
            username: &'a str,
            #[serde(rename = "Password")]
            password: &'a str,
            #[serde(rename = "__RequestVerificationToken")]
            token: &'a str,
        }

        info!("fetching celcat federation ids");
        let form = Form {
            username,
            password,
            token: &self.token,
        };
        debug!("{:?}", form);
        self.client
            .post(&format!("{}/LdapLogin/Logon", self.address))
            .form(&form)
            .send()
            .await?;

        Ok(())
    }

    pub async fn fetch<F>(&self, req: F::Request) -> Result<F, FetchError>
    where
        F: Fetchable,
    {
        let res = self
            .client
            .post(&format!("{}/Home/{}", self.address, F::METHOD_NAME))
            .form(&req)
            .send()
            .await?
            .json()
            .await?;

        Ok(res)
    }
}
