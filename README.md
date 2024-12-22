# Finance Data Fetcher example

This is a simple Rust Wasm example that fetches financial data from an open finance data API, creates an xls file, and returns it for download. Its purpose is to show how you can use `wasi:http/outgoing-handler` in a component.

## Prerequisites

- `cargo` 1.75
- [`wash`](https://wasmcloud.com/docs/installation) 0.27.0
- `wasmtime` >=25.0.0 (if running with wasmtime)

## Building

```bash
wash build
```

## Running with wasmtime

You must have wasmtime >=25.0.0 for this to work. Make sure to follow the build step above first.

```bash
wasmtime serve -Scommon ./build/finance_fetcher_s.wasm
```

## Running with wasmCloud

Ensuring you've built your component with `wash build`, you can launch wasmCloud and deploy the full application with the following commands. Once the application reports as **Deployed** in the application list, you can use `curl` to send a request to the running HTTP server.

```shell
wash up -d
wash app deploy ./wadm.yaml
wash app get
curl http://127.0.0.1:8000
```

## Using the Finance Data Fetcher

1. Visit the home page.
2. Fill out the input form with the following fields:
   - A list of valid stock tickers to retrieve the financial history
   - A start and finish date range to pull the history
   - Any other relevant fields for searching in the financial API
3. Click the search button to initiate the query.
4. The queried data will be processed and streamed into a new xls file.
5. The xls file will be returned for you to download.

## Adding Capabilities

To learn how to extend this example with additional capabilities, see the [Adding Capabilities](https://wasmcloud.com/docs/tour/adding-capabilities?lang=rust) section of the wasmCloud documentation.

## Running the Application and Using the Form

1. Build the project using `wash build`.
2. Run the application using `wasmtime` or `wasmCloud` as described above.
3. Open your web browser and navigate to `http://127.0.0.1:3030`.
4. Fill out the form with the following fields:
   - A list of valid stock tickers to query
   - A start and finish date range
5. Click the submit button to initiate the query.
6. The queried data will be processed and returned to the screen.
