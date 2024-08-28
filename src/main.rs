use std::net::SocketAddr;
use std::error::Error;
use hyper::server::conn::Http;
use hyper::service::service_fn;
use hyper::{header, Body, Method, Request, Response, StatusCode};
use serde::{Deserialize, Serialize};
use tokio::net::TcpListener;

const INTERNAL_SERVER_ERROR: &str = "Server Error";

#[derive(Serialize, Deserialize)]
struct Artifact {
    id: String,
    country_of_origin: String,
    name: String,
    whereabout:String,
    year_of_discovery: u16,
}

fn get_artifact_list() -> Response<Body> {
    let artifacts: [Artifact; 4] = [
        Artifact {
            id: "1".to_owned(),
            country_of_origin: "Armenia".to_owned(),
            name: "Pottery 12B Oregon".to_owned(),
            whereabout: "British Museum".to_owned(),
            year_of_discovery: 2015,
        },
        Artifact {
            id: "2".to_owned(),
            country_of_origin: "Turkey".to_owned(),
            name: "Mud Statue Female Goddess H9 Kenneth".to_owned(),
            year_of_discovery: 2003,
            whereabout: "Chinese National Museum - Beijing Branch".to_owned(),
        },
        Artifact {
            id: "3".to_owned(),
            whereabout: "Lost".to_owned(),
            country_of_origin: "Greece".to_owned(),
            name: "Hourglass 1765 Poinlace".to_owned(),
            year_of_discovery: 1979,
        },
        Artifact {
            id: "4".to_owned(),
            whereabout: "Egypt Cairo Museum".to_owned(),
            country_of_origin: "Egypt".to_owned(),
            name: "Hieratic Payprus 2000 BCE Tara".to_owned(),
            year_of_discovery: 1988,
        },
    ];

    match serde_json::to_string(&artifacts) {
        Ok(json) => Response::builder()
            .header(header::CONTENT_TYPE, "application/json")
            .body(Body::from(json))
            .unwrap(),
        Err(_) => Response::builder()
            .status(StatusCode::INTERNAL_SERVER_ERROR)
            .body(INTERNAL_SERVER_ERROR.into())
            .unwrap(),
    }
}

fn get_artifact_by_id(artifact_id: &String) -> Response<Body> {
    let artifacts: [Artifact; 4] = [
        Artifact {
            id: "1".to_owned(),
            country_of_origin: "Armenia".to_owned(),
            name: "Pottery 12B Oregon".to_owned(),
            year_of_discovery: 2015,
            whereabout: "British Museum".to_owned(),
        },
        Artifact {
            id: "2".to_owned(),
            whereabout: "Chinese National Museum - Beijing Branch".to_owned(),
            country_of_origin: "Turkey".to_owned(),
            name: "Mud Statue Female Goddess H9 Kenneth".to_owned(),
            year_of_discovery: 2003,
        },
        Artifact {
            id: "3".to_owned(),
            country_of_origin: "Greece".to_owned(),
            name: "Hourglass 1765 Poinlace".to_owned(),
            year_of_discovery: 1979,
            whereabout: "Lost".to_owned(),
        },
        Artifact {
            id: "4".to_owned(),
            whereabout: "Egypt Cairo Museum".to_owned(),
            country_of_origin: "Egypt".to_owned(),
            name: "Hieratic Payprus 2000 BCE Tara".to_owned(),
            year_of_discovery: 1988,
        },
    ];

    let artifact_index_option = artifacts.iter().position(|x| &x.id == artifact_id);

    if artifact_index_option.is_none() {
        return Response::new(Body::from("Car not found"));
    }

    let artifact = &artifacts[artifact_index_option.unwrap()];

    match serde_json::to_string(artifact) {
        Ok(json) => Response::builder()
            .header(header::CONTENT_TYPE, "application/json")
            .body(Body::from(json))
            .unwrap(),
        Err(_) => Response::builder()
            .status(StatusCode::INTERNAL_SERVER_ERROR)
            .body(INTERNAL_SERVER_ERROR.into())
            .unwrap(),
    }
}

async fn artifacts_handler(req: Request<Body>) -> Result<Response<Body>, Box<dyn Error + Send + Sync>> {
    let path = req.uri().path().to_owned();
    let path_segments = path.split("/").collect::<Vec<&str>>();
    let base_path = path_segments[1];

    match (req.method(), base_path) {
        (&Method::GET, "artifacts") => {
            if path_segments.len() <= 2 {
                let res = get_artifact_list();
                return Ok(res);
            }

            let artifact_id = path_segments[2];

            if artifact_id.trim().is_empty() {
                let res = get_artifact_list();
                return Ok(res);
            } else {
                let res = get_artifact_by_id(&artifact_id.to_string());
                Ok(res)
            }
        }

        _ => {
            let mut not_found = Response::default();
            *not_found.status_mut() = StatusCode::NOT_FOUND;
            Ok(not_found)
        }
    }
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error + Send + Sync>> {
    let addr = SocketAddr::from(([127, 0, 0, 1], 3030));

    let listener = TcpListener::bind(addr).await?;
    println!("Listening on http://{}", addr);
    loop {
        let (stream, _) = listener.accept().await?;

        tokio::task::spawn(async move {
            if let Err(err) = Http::new()
                .serve_connection(stream, service_fn(artifacts_handler))
                .await
            {
                println!("Error serving connection: {:?}", err);
            }
        });
    }
}