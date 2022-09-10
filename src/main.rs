use std::{
    env::{self, VarError},
    path::PathBuf,
};

use anyhow::{anyhow, Result};
use lapce_plugin::{
    psp_types::{
        lsp_types::{request::Initialize, DocumentFilter, DocumentSelector, InitializeParams, Url},
        Request,
    },
    register_plugin, LapcePlugin, PLUGIN_RPC,
};
use serde_json::Value;

#[derive(Default)]
struct State {}

register_plugin!(State);

fn initialize(params: InitializeParams) -> Result<()> {
    let document_selector: DocumentSelector = vec![DocumentFilter {
        language: Some(String::from("go")),
        pattern: Some(String::from("**.go")),
        scheme: None,
    }];
    let mut server_args = vec![];

    if let Some(options) = params.initialization_options.as_ref() {
        if let Some(lsp) = options.get("lsp") {
            if let Some(args) = lsp.get("serverArgs") {
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

            if let Some(server_path) = lsp.get("serverPath") {
                if let Some(server_path) = server_path.as_str() {
                    if !server_path.is_empty() {
                        let url = Url::parse(&format!("urn:{}", server_path))?;

                        PLUGIN_RPC.stderr(&format!("url: {url}"));
                        PLUGIN_RPC.stderr(&format!("args: {server_args:?}"));

                        PLUGIN_RPC.start_lsp(
                            url,
                            server_args,
                            document_selector,
                            params.initialization_options,
                        );
                        return Ok(());
                    }
                }
            }
        }
    }

    let server_path = match env::var("GOBIN") {
        Ok(var) => var,
        Err(error) => match error {
            VarError::NotPresent => match env::var("GOPATH") {
                Ok(var) => format!("{var}/bin"),
                Err(error) => match error {
                    VarError::NotPresent => {
                        let home = match env::var("HOME") {
                            Ok(var) => var,
                            Err(_) => return Err(anyhow!("couldn't fine any path for gopls")),
                        };
                        PathBuf::from(home)
                            .join("go")
                            .join("bin")
                            .to_string_lossy()
                            .to_string()
                    }
                    VarError::NotUnicode(val) => {
                        let val = val.to_string_lossy();
                        return Err(anyhow!("GOBIN is not in unicode format: '{val}'"));
                    }
                },
            },
            VarError::NotUnicode(val) => {
                let val = val.to_string_lossy();
                return Err(anyhow!("GOBIN is not in unicode format: '{val}'"));
            }
        },
    };

    // Slash at the end is important, otherwise last path element is removed
    let server_path = Url::parse(&format!("urn:{server_path}/"))?.join("gopls")?;

    PLUGIN_RPC.stderr(&format!("path: {server_path}"));
    PLUGIN_RPC.stderr(&format!("args: {server_args:?}"));

    PLUGIN_RPC.start_lsp(
        server_path,
        server_args,
        document_selector,
        params.initialization_options,
    );

    Ok(())
}

impl LapcePlugin for State {
    fn handle_request(&mut self, _id: u64, method: String, params: Value) {
        #[allow(clippy::single_match)]
        match method.as_str() {
            Initialize::METHOD => {
                let params: InitializeParams = serde_json::from_value(params).unwrap();
                let _ = initialize(params);
            }
            _ => {}
        }
    }
}
