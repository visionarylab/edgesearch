use std::fs::File;
use std::io::Read;
use std::path::PathBuf;

use structopt::StructOpt;

use edgesearch::build::{build, BuildConfig, DocumentEncoding};
use edgesearch::deploy::{deploy, DeployConfig};
use edgesearch::test::start_server;

use crate::Cli::{Build, Deploy, Test};

#[derive(StructOpt)]
enum Cli {
    Build {
        #[structopt(long, default_value = "text")] document_encoding: DocumentEncoding,
        #[structopt(long, parse(from_os_str))] document_terms: PathBuf,
        #[structopt(long, parse(from_os_str))] documents: PathBuf,
        #[structopt(long, default_value = "512")] maximum_query_bytes: usize,
        #[structopt(long)] maximum_query_results: usize,
        #[structopt(long, default_value = "50")] maximum_query_terms: usize,
        #[structopt(long, parse(from_os_str))] output_dir: PathBuf,
    },
    Deploy {
        #[structopt(long)] account_email: String,
        #[structopt(long)] account_id: String,
        #[structopt(long, parse(from_os_str))] default_results: PathBuf,
        #[structopt(long)] global_api_key: String,
        #[structopt(long)] name: String,
        #[structopt(long)] namespace: Option<String>,
        #[structopt(long, parse(from_os_str))] output_dir: PathBuf,
        #[structopt(long)] upload_data: bool,
    },
    Test {
        #[structopt(long, parse(from_os_str))] default_results: PathBuf,
        #[structopt(long, default_value = "text")] document_encoding: DocumentEncoding,
        #[structopt(long, parse(from_os_str))] output_dir: PathBuf,
        #[structopt(long)] port: usize,
    },
}

fn main() {
    let args = Cli::from_args();

    match args {
        Build {
            document_encoding,
            document_terms,
            documents,
            maximum_query_bytes,
            maximum_query_results,
            maximum_query_terms,
            output_dir,
        } => {
            build(BuildConfig {
                document_encoding,
                document_terms_source: File::open(document_terms).expect("open document terms file"),
                documents_source: File::open(documents).expect("open documents file"),
                maximum_query_bytes,
                maximum_query_results,
                maximum_query_terms,
                output_dir,
            });
        }
        Deploy {
            account_email,
            account_id,
            default_results: default_results_path,
            global_api_key,
            name,
            namespace,
            output_dir,
            upload_data,
        } => {
            let mut default_results = String::new();
            File::open(default_results_path).expect("open default results file").read_to_string(&mut default_results).expect("read default results file");
            deploy(DeployConfig {
                account_email,
                account_id,
                default_results,
                global_api_key,
                name,
                namespace,
                output_dir,
                upload_data,
            });
        }
        Test {
            default_results: default_results_path,
            document_encoding,
            output_dir,
            port,
        } => {
            let mut default_results = String::new();
            File::open(default_results_path).expect("open default results file").read_to_string(&mut default_results).expect("read default results file");
            start_server(output_dir, port, default_results);
        }
    };
}