// This is free and unencumbered software released into the public domain.

#[cfg(feature = "std")]
fn main() -> Result<clientele::SysexitsError, Box<dyn std::error::Error>> {
    use asimov_module::getenv;
    use asimov_serpapi_module::{api::SerpApi, find_engine_for};
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
    let Some(api_key) = getenv::var_secret("SERPAPI_KEY") else {
        return Ok(EX_CONFIG); // not configured
    };
    let api = SerpApi::new(api_key);

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

        // Serialize the response data:
        if cfg!(feature = "pretty") {
            let response_json: serde_json::Value = serde_json::from_str(&response)?;
            colored_json::write_colored_json(&response_json, &mut stdout())?;
            println!();
        } else {
            println!("{}", response);
        }
    }

    Ok(EX_OK)
}

#[cfg(not(feature = "std"))]
fn main() {
    unimplemented!("asimov-serpapi-fetcher requires the 'std' feature")
}
