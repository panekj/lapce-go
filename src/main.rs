#![allow(dead_code)]

use std::{
    env::{self, VarError},
    fmt::Display,
    path::PathBuf,
};

use anyhow::{anyhow, Result};
use serde_json::Value;
use volt::{
    psp_types::{
        lsp_types::{
            request::Initialize, DocumentFilter, DocumentSelector, InitializeParams, MessageType,
            Url,
        },
        Request,
    },
    register_plugin, LapcePlugin, VoltEnvironment, PLUGIN_RPC,
};

mod tools;

#[derive(Default)]
struct State {}

register_plugin!(State);

#[derive(thiserror::Error, Debug)]
enum PluginError {
    InstallFailed(&'static str),
    NoPathFound,
}

impl Display for PluginError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::InstallFailed(v) => write!(f, "Install failed: {v}"),
            Self::NoPathFound => write!(f, "No path found"),
        }
    }
}

#[macro_export]
macro_rules! string {
    ($s:expr) => {
        String::from($s)
    };
}
fn initialize(params: InitializeParams) -> Result<()> {
    let document_selector: DocumentSelector = vec![DocumentFilter {
        language: Some(string!("go")),
        pattern:  Some(string!("**.go")),
        scheme:   None,
    }];
    let mut server_args = vec![];
    let mut options = None;

    if let Some(opts) = params.initialization_options.as_ref() {
        options = opts.get("gopls").map(|k| k.to_owned());

        if let Some(volt) = opts.get("volt") {
            if let Some(args) = volt.get("serverArgs") {
                if let Some(args) = args.as_array() {
                    if !args.is_empty() {
                        server_args = vec![];
                    }
                    for arg in args {
                        if let Some(arg) = arg.as_str() {
                            server_args.push(arg.to_string());
                        }
                    }
                }
            }

            if let Some(server_path) = volt.get("serverPath") {
                if let Some(server_path) = server_path.as_str() {
                    if !server_path.is_empty() {
                        let url = Url::parse(&format!("urn:{server_path}"))?;
                        PLUGIN_RPC.start_lsp(url, server_args, document_selector, options);
                        return Ok(());
                    }
                }
            }
        }
    }

    for tool in tools::ALL_TOOLS_INFORMATION {
        if !PLUGIN_RPC
            .execute_process(string!("go"), vec![string!("install"), tool.install_path()])?
            .success
        {
            return Err(anyhow!("Failed to install tool: {}", tool.name));
        }
    }

    let server_path = match env::var("GOBIN") {
        Ok(v) => PathBuf::from(v),
        Err(e) => match e {
            VarError::NotPresent => match env::var("GOPATH") {
                Ok(v) => PathBuf::from(v).join("bin"),
                Err(e) => match e {
                    VarError::NotPresent => {
                        let home = match env::var("HOME") {
                            Ok(v) => PathBuf::from(v),
                            Err(_) => return Err(PluginError::NoPathFound.into()),
                        };
                        home.join("go").join("bin")
                    }
                    VarError::NotUnicode(v) => {
                        let v = v.to_string_lossy();
                        return Err(anyhow!("GOBIN is not in unicode format: '{v}'"));
                    }
                },
            },
            VarError::NotUnicode(v) => {
                let v = v.to_string_lossy();
                return Err(anyhow!("GOBIN is not in unicode format: '{v}'"));
            }
        },
    };

    let server_path = match VoltEnvironment::operating_system().as_deref() {
        Ok("windows") => server_path.join("gopls.exe"),
        _ => server_path.join("gopls"),
    };

    let server_uri = Url::parse(&format!("urn:{}", server_path.display()))?;

    PLUGIN_RPC.start_lsp(dbg!(server_uri), server_args, document_selector, options);

    Ok(())
}

impl LapcePlugin for State {
    fn handle_request(&mut self, _id: u64, method: String, params: Value) {
        #[allow(clippy::single_match)]
        match method.as_str() {
            Initialize::METHOD => {
                let params: InitializeParams = serde_json::from_value(params).unwrap();
                if let Err(e) = initialize(params) {
                    PLUGIN_RPC.window_show_message(
                        MessageType::ERROR,
                        format!("plugin returned with error: {e}"),
                    );
                } else {
                    PLUGIN_RPC.window_log_message(
                        MessageType::INFO,
                        string!("plugin finished execution"),
                    );
                }
            }
            _ => {}
        }
    }
}
