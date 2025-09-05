# Login Server in Rust (Academic Project)

<img src="https://img.shields.io/badge/Rust-2021-orange.svg" alt="Rust 2021">
<img src="https://img.shields.io/badge/license-AGPL--3.0-blue.svg" alt="AGPL-3.0 License">
<img src="https://img.shields.io/badge/status-academic--only-red.svg" alt="Academic Only">
<img src="https://img.shields.io/badge/build-cargo-green.svg" alt="Cargo Build">

## Project

This repository contains a high-performance login server implemented in Rust, designed with both HTTP and gRPC interfaces. It incorporates features such as per-IP rate limiting and Redis caching to demonstrate modern backend development practices.

**IMPORTANT DISCLAIMER:**
This project is **purely academic and educational** in nature. It is a portfolio piece created for didactic purposes only and **does not meet production standards**. It is **not intended for, and must never be used in, a production environment**.

## Repository Policy

This repository is maintained as a personal portfolio piece. As such, it **does not accept pull requests or issues**.

## License

This project is licensed under the **AGPL-3.0 License**. See the full [AGPL-3.0 License](LICENSE) for more details.

## Load Testing with Vegeta

The project includes sample scripts and configurations for load testing the HTTP endpoint using [Vegeta](https://github.com/tsenart/vegeta). These tests demonstrate the server's performance under various load conditions.

### Test Files:

- `scripts/bench_http.sh`: A Bash script to automate HTTP load testing with Vegeta. It includes logic to use a local Vegeta installation or a Dockerized version.
- `scripts/bench_grpc.sh`: A Bash script to automate gRPC load testing with ghz. It includes logic to use a local ghz installation or a Dockerized version.
- `scripts/schema.sql`: SQL script to create the necessary tables and seed initial data for the MariaDB database.

To run the HTTP benchmark, you can use the provided scripts. Ensure your Docker environment is running and the `login` service is accessible (e.g., at `http://localhost:8080`).

For more detailed usage and options, refer to the comments in the `bench_http.sh` and `bench_grpc.sh` scripts.
