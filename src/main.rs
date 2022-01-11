use warp::{
    http::StatusCode,
    Filter
};
use serde::{Deserialize};
use serde_json;
use serde_yaml;
use std::env;
use std::fs;
use std::result;
use std::convert::Infallible;
use overseerd::errors::{new_error, ErrorKind, Result};

#[derive(Debug, Clone, Deserialize, PartialEq)]
pub struct Config {
    #[serde(rename(deserialize = "services"))]
    services: Vec<Service>,
}

#[derive(Debug, Clone, Deserialize, PartialEq)]
enum ServiceType {
    Systemd,
    Docker
}

#[derive(Debug, Clone, Deserialize, PartialEq)]
pub struct Service {
    name: String,

    #[serde(rename(deserialize = "type"))]
    service_type: ServiceType
}

pub fn parse_config(raw: String) -> Result<Config> {
    return serde_yaml::from_str(&raw)
                .map_err(|err| new_error(ErrorKind::ConfigParsingError(err)));
}

pub fn load_config(location: String) -> Result<Config> {
    return fs::read_to_string(location)
                .map_err(|err| new_error(ErrorKind::ConfigReadingError(err)))
                .and_then(parse_config)
}

fn with_config(
    config: Config,
) -> impl Filter<Extract = (Config,), Error = Infallible> + Clone {
    warp::any().map(move || config.clone())
}

#[derive(Debug, Deserialize)]
pub struct GetStatusOpts {
    pub generate: Option<String>,
    pub duration_seconds: Option<String>,
}

pub async fn get_status(
    get_status_opts: GetStatusOpts, 
    config: Config
) -> result::Result<impl warp::Reply, Infallible> {
    log::debug!("get_status: {:?} // {:?}", get_status_opts, config);
    Ok(warp::reply::with_status(
        "hey dude!".to_string(),
        StatusCode::OK,
    ))
}

#[tokio::main]
async fn main() {

    if env::var_os("RUST_LOG").is_none() {
        // Set `RUST_LOG=jwtd=debug` to see debug logs,
        // info - only shows access logs.
        env::set_var("RUST_LOG", "jwtd=debug");
    }
    pretty_env_logger::init();

    let location = env::var("CONFIG_LOCATION".to_string())
        .map_err(|_| new_error(ErrorKind::MissingConfigError(
            "Environment variable 'CONFIG_LOCATION' not set; unable to read configuration".to_string())))
        .unwrap();

    let config = load_config(location).unwrap();
    log::info!("Config loaded");

    let get_status = warp::path!("status")
                    .and(warp::get())
                    .and(warp::query::<GetStatusOpts>())
                    .and(with_config(config.clone()))
                    .and_then(get_status);

    let port = env::var("PORT")
                    .map(|a| match a.parse() {
                        Ok(v) => v,
                        _ => 8080
                    })
                    .unwrap_or_else(|_err| {
                        log::info!("Port not provided, fallback on default");
                        8080
                    });

    let routes = get_status;
    log::info!("Server starting on port {}", port);
    warp::serve(routes)
        .run(([0, 0, 0, 0], port))
        .await;
}

#[cfg(test)]
mod tests {
    // Note this useful idiom: importing names from outer (for mod tests) scope.
    use super::*;

    #[test]
    fn test_parse_config() {
        let cfg = parse_config(r#"
            services:
                - name: patroni
                  type: Systemd
        "#.to_string()).unwrap();

        let empty:Vec<Service> = Vec::new();
        assert_eq!(cfg.services.get(0).unwrap().name, "patroni");
        assert_eq!(cfg.services.get(0).unwrap().service_type, ServiceType::Systemd);
    }

}