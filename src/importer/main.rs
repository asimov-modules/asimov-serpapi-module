// This is free and unencumbered software released into the public domain.

use clap::Parser;
use clientele::StandardOptions;

/// asimov-serpapi-importer
#[derive(Debug, Parser)]
#[command(arg_required_else_help = true)]
struct Options {
    #[clap(flatten)]
    flags: StandardOptions,

    /// The output format.
    #[arg(value_name = "FORMAT", short = 'o', long)]
    output: Option<String>,

    urls: Vec<String>,
}

#[cfg(feature = "std")]
fn main() -> Result<clientele::SysexitsError, Box<dyn std::error::Error>> {
    use asimov_serpapi_module::{api::SerpApi, find_engine_for, jq};
    use clientele::SysexitsError::*;
    use std::io::stdout;

    // Load environment variables from `.env`:
    clientele::dotenv().ok();

    // Expand wildcards and @argfiles:
    let args = clientele::args_os()?;

    // Parse command-line options:
    let options = Options::parse_from(args);

    // Handle the `--version` flag:
    if options.flags.version {
        println!("{} {}", env!("CARGO_PKG_NAME"), env!("CARGO_PKG_VERSION"));
        return Ok(EX_OK);
    }

    // Handle the `--license` flag:
    if options.flags.license {
        print!("{}", include_str!("../../UNLICENSE"));
        return Ok(EX_OK);
    }

    // Configure logging & tracing:
    #[cfg(feature = "tracing")]
    asimov_module::init_tracing_subscriber(&options.flags).expect("failed to initialize logging");

    if options.urls.is_empty() {
        return Ok(EX_OK);
    }

    // Obtain the SerpApi API key from the environment:
    let Ok(manifest) = asimov_module::ModuleManifest::read_manifest("serpapi")
        .inspect_err(|e| eprintln!("failed to read module manifest: {e}"))
    else {
        return Ok(EX_CONFIG);
    };

    // Obtain the SerpApi API key from the environment:
    let Ok(api_key) = manifest
        .variable("key", None)
        .inspect_err(|e| eprintln!("failed to get API key: {e}"))
    else {
        return Ok(EX_CONFIG); // not configured
    };
    let api = SerpApi::new(api_key.into());

    // Process each of the given URL arguments:
    for url in options.urls {
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
