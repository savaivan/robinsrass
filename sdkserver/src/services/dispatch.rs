use crate::config::versions;
use axum::extract::Query;
use prost::Message;
use proto::{Gateserver, GlobalDispatchData, ServerData};
use serde::Deserialize;

pub const QUERY_DISPATCH_ENDPOINT: &str = "/query_dispatch";
pub const QUERY_GATEWAY_ENDPOINT: &str = "/query_gateway";

#[tracing::instrument]
pub async fn query_dispatch() -> String {
    let rsp = GlobalDispatchData {
        retcode: 0,
        server_list: vec![ServerData {
            name: String::from("os_nazi"),
            title: String::from("NaziSR"),
            env_type: String::from("2"),
            dispatch_url: String::from("http://192.168.0.90:21000/query_gateway"),
            ..Default::default()
        }],
        ..Default::default()
    };

    let mut buff = Vec::new();
    rsp.encode(&mut buff).unwrap();

    rbase64::encode(&buff)
}

#[derive(Deserialize, Debug)]
pub struct QueryGatewayParameters {
    pub version: String,
}

#[tracing::instrument]
pub async fn query_gateway(parameters: Query<QueryGatewayParameters>) -> String {
    let rsp = if let Some(config) = versions.get(&parameters.version) {
        Gateserver {
            retcode: 0,
            ip: String::from("192.168.0.90"),
            port: 23301,
            asset_bundle_url: config.asset_bundle_url.clone(),
            ex_resource_url: config.ex_resource_url.clone(),
            lua_url: config.lua_url.clone(),
            lua_version: config.lua_version.clone(),
            ifix_version: String::from("0"),
            pdpbjhfgnjk: true,
            bipcmeeljhj: true,
            hecpclndaac: true,
            nlfkefmfige: true,
            oigmgpfnloj: true,
            pnnionnkbnn: true,
            use_tcp: true,
            ..Default::default()
        }
    } else {
        Gateserver {
            retcode: 9,
            msg: format!("forbidden version: {} or invalid bind", parameters.version),
            ..Default::default()
        }
    };

    let mut buff = Vec::new();
    rsp.encode(&mut buff).unwrap();

    rbase64::encode(&buff)
}
