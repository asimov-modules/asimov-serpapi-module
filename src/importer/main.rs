// This is free and unencumbered software released into the public domain.

#[cfg(feature = "std")]
fn main() -> Result<clientele::SysexitsError, Box<dyn std::error::Error>> {
    use asimov_serpapi_module::{api::SerpApi, find_engine_for, jq};
    use clientele::SysexitsError::*;
    use std::io::stdout;

    // Load environment variables from `.env`:
    clientele::dotenv().ok();

    // Expand wildcards and @argfiles:
    let args = clientele::args_os()?;

    // Configure logging:
    #[cfg(feature = "tracing")]
    tracing_subscriber::fmt()
        .with_writer(std::io::stderr)
        .with_max_level(tracing_subscriber::filter::LevelFilter::WARN)
        .init();

    // Parse URLs from command-line arguments:
    let urls: Vec<String> = args
        .iter()
        .skip(1)
        .map(|arg| arg.to_string_lossy().into())
        .collect();
    if urls.is_empty() {
        return Ok(EX_OK);
    }

    // Obtain the SerpApi API key from the environment:
    let manifest = match asimov_module::ModuleManifest::read_manifest("serpapi") {
        Ok(manifest) => manifest,
        Err(e) => {
            eprintln!("failed to read module manifest: {e}");
            return Ok(EX_CONFIG);
        }
    };

    // Obtain the SerpApi API key from the environment:
    let api_key = match manifest.variable("serpapi-key", None) {
        Ok(api_key) => api_key,
        Err(e) => {
            eprintln!("failed to get API key: {e}");
            return Ok(EX_CONFIG); // not configured
        }
    };
    let api = SerpApi::new(api_key.into());

    // Process each of the given URL arguments:
    for url in urls {
        // Find the appropriate engine ID based on the URL prefix:
        let Some(engine) = find_engine_for(&url) else {
            return Ok(EX_UNAVAILABLE); // not supported
        };

        // Send the request and block while waiting for the response:
        let response = match engine.id {
            "bing" => api.search_bing(&url.parse().map_err(|_| EX_DATAERR)?)?,
            "duckduckgo" => api.search_duckduckgo(&url.parse().map_err(|_| EX_DATAERR)?)?,
            "google" => api.search_google(&url.parse().map_err(|_| EX_DATAERR)?)?,
            _ => {
                return Ok(EX_UNAVAILABLE); // not supported
            }
        };
        let response = jq::search().filter_json_str(response)?;

        // Serialize the response data:
        if cfg!(feature = "pretty") {
            colored_json::write_colored_json(&response, &mut stdout())?;
            println!();
        } else {
            println!("{}", serde_json::to_string(&response).unwrap());
        }
    }

    Ok(EX_OK)
}

#[cfg(not(feature = "std"))]
fn main() {
    unimplemented!("asimov-serpapi-importer requires the 'std' feature")
}
