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
use calamine::{Xlsx, open_workbook, Writer, XlsxWriter};

#[derive(serde::Deserialize)]
struct FinanceResponse {
    // Define the structure of the financial data response
}

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
        let mut write_stream = response_body.write().unwrap();
        ResponseOutparam::set(response_out, Ok(response));
        write_stream.write_all(&xls_file).unwrap();
        drop(write_stream);
        OutgoingBody::finish(response_body, None).expect("failed to finish response body");
    }
}

fn parse_form_data(request: IncomingRequest) -> FormData {
    // Implement form data parsing logic
}

fn query_financial_data(form_data: &FormData) -> FinanceResponse {
    // Implement financial data querying logic using reqwest
}

fn create_xls_file(finance_data: &FinanceResponse) -> Vec<u8> {
    // Implement xls file creation logic using calamine
}
