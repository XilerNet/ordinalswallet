# Xiler Domains Synchronization to OrdinalsWallet

Welcome to the documentation for the Xiler Domains Synchronization to OrdinalsWallet project. This closed-source Rust project allows you to synchronize Xiler Domains and send a Discord webhook message when certain events occur. Below, you will find a step-by-step guide to setting up and configuring this project.

## Prerequisites
Before you get started, make sure you have the following prerequisites installed on your system:

* [Rust](https://www.rust-lang.org/tools/install)
* [PostgreSQL](https://www.postgresql.org/download/)
* [Git](https://git-scm.com/downloads)

## Project Configuration
The project uses environment variables for its configuration. You need to set these variables in a `.env` file before running the project. Here is a list of the environment variables:

* **`CREATOR_ADDRESS`**: The BTC address of the collection creator.
* **`CREATOR_SIGNATURE`**: The signature of the creator address.
* **`SLUG`**: The slug for the collection.
* **`DATABASE_URL`**: The URL for your PostgreSQL database, including username, password, host, port, and database name.
* **`WEBHOOK_URL`**: The Discord webhook URL for sending messages.

## Building and Running the Project
To build and run the project, follow these steps:

1. Clone the project repository:
    ```bash
    git clone git@github.com:XilerNet/ordinalswallet.git ordinalswallet-sync
    cd ordinalswallet-sync
    ```
2. Set up the environment variables by copyin the `.env.example` file to `.env` and configuring it as explained in the comments.

3. Build the project using Cargo:
    ```bash
    cargo build --release
    ```
4. Run the project:
    ```bash
    ./target/release/ordinalswallet
    ```

The project should now be up and running, synchronizing Xiler Domains and sending Discord webhook messages when necessary.

## Contributing
This project is closed source, and contributions are not accepted.

## License
Altough this project's LICENCE is MIT, it does not apply until this is open source, as it intended for private use.
