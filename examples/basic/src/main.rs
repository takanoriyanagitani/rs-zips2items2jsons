use std::process::ExitCode;

use std::io;

fn sub_json() -> Result<(), io::Error> {
    rs_zips2items2jsons::stdin2znames2zips2items2jsons2stdout_default()
}

fn sub_zcat() -> Result<(), io::Error> {
    rs_zips2items2jsons::stdin2znames2zips2items2zcat2jsons2stdout_default()
}

fn is_json_gzipped() -> bool {
    std::env::var("ENV_JSON_GZIPPED")
        .ok()
        .map(|s| s.eq("true"))
        .unwrap_or(false)
}

fn sub() -> Result<(), io::Error> {
    match is_json_gzipped() {
        true => sub_zcat(),
        false => sub_json(),
    }
}

fn main() -> ExitCode {
    sub().map(|_| ExitCode::SUCCESS).unwrap_or_else(|e| {
        eprintln!("{e}");
        ExitCode::FAILURE
    })
}
