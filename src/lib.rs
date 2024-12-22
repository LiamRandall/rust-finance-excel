mod bindings {
    use crate::FinanceFetcher;

    wit_bindgen::generate!({
        with: {
            "wasi:clocks/monotonic-clock@0.2.2": ::wasi::clocks::monotonic_clock,
            "wasi:http/incoming-handler@0.2.2": generate,
            "wasi:http/outgoing-handler@0.2.2": ::wasi::http::outgoing_handler,
            "wasi:http/types@0.2.2": ::wasi::http::types,
            "wasi:io/error@0.2.2": ::wasi::io::error,
            "wasi:io/poll@0.2.2": ::wasi::io::poll,
            "wasi:io/streams@0.2.2": ::wasi::io::streams,
        }
    });

    export!(FinanceFetcher);
}

use std::io::{Read as _, Write as _};
use bindings::exports::wasi::http::incoming_handler::Guest;
use wasi::http::types::*;
use reqwest::Client;
use chrono::NaiveDate;
// Remove Writer, XlsxWriter since calamine no longer supports writing.
use calamine::{Xlsx, open_workbook};
use warp::Filter;
use handlebars::Handlebars;

// ----------------------------
// 1) Define the data structures
// ----------------------------

// Placeholder type that your form-parsing logic will fill out
#[derive(Debug)]
struct FormData {
    symbol: String,
    // Add more fields as needed
}

// This is the data structure returned by your finance query
#[derive(serde::Deserialize)]
struct FinanceResponse {
    // Example fields
    symbol: String,
    price: f64,
    date: String,
}

// ----------------------------
// 2) Implement your Guest trait
// ----------------------------

struct FinanceFetcher;

impl Guest for FinanceFetcher {
    fn handle(request: IncomingRequest, response_out: ResponseOutparam) {
        // Parse input form data
        let form_data = parse_form_data(request);

        // Query financial data using an open finance data API
        let finance_data = query_financial_data(&form_data);

        // Create and stream xls file with queried data
        let xls_file = create_xls_file(&finance_data);

        // Modify response to return xls file for download
        let response = OutgoingResponse::new(Fields::new());
        response.set_status_code(200).unwrap();
        let response_body = response.body().unwrap();

        // Write xls_file bytes to the response
        let mut write_stream = response_body.write().unwrap();
        ResponseOutparam::set(response_out, Ok(response));
        write_stream.write_all(&xls_file).unwrap();
        drop(write_stream);

        OutgoingBody::finish(response_body, None)
            .expect("failed to finish response body");
    }
}

// ----------------------------
// 3) Parse Form Data
// ----------------------------
fn parse_form_data(request: IncomingRequest) -> FormData {
    // TODO: Your logic to extract form data from the request
    // For now, return a placeholder
    FormData {
        symbol: "AAPL".to_string(),
    }
}

// ----------------------------
// 4) Query Finance Data
// ----------------------------
fn query_financial_data(_form_data: &FormData) -> FinanceResponse {
    // TODO: Use reqwest to call an external API, parse JSON, etc.
    // Returning dummy data for now:
    FinanceResponse {
        symbol: "AAPL".to_string(),
        price: 123.45,
        date: "2024-12-31".to_string(),
    }
}

// ----------------------------
// 5) Create XLS File
// ----------------------------
fn create_xls_file(finance_data: &FinanceResponse) -> Vec<u8> {
    // Since calamine doesn't actually write files, you need a different library
    // to generate Excel files. For demonstration, we'll just return some bytes.

    // TODO: Replace with real XLSX writer logic from another crate if needed.
    // e.g. rust_xlsxwriter or xlsxwriter-rs.

    // For now, pretend this is our "file" in memory:
    let fake_bytes = format!(
        "symbol: {}, price: {}, date: {}\n",
        finance_data.symbol, finance_data.price, finance_data.date
    ).into_bytes();

    fake_bytes
}

// Serve the index.html file
async fn serve_index() -> Result<impl warp::Reply, warp::Rejection> {
    let html = include_str!("index.html");
    Ok(warp::reply::html(html))
}

// Handle form submissions
async fn handle_form_submission(form: warp::filters::body::Form<FormData>) -> Result<impl warp::Reply, warp::Rejection> {
    let form_data = form.into_inner();
    let finance_data = query_financial_data(&form_data);
    let xls_file = create_xls_file(&finance_data);
    Ok(warp::reply::with_header(xls_file, "Content-Disposition", "attachment; filename=\"finance_data.xlsx\""))
}

// Main function to start the server
#[tokio::main]
async fn main() {
    let index_route = warp::path::end().and_then(serve_index);
    let form_route = warp::path("submit").and(warp::post()).and(warp::body::form()).and_then(handle_form_submission);

    let routes = index_route.or(form_route);

    warp::serve(routes).run(([127, 0, 0, 1], 3030)).await;
}
