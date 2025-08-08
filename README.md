# Bit Casino -- Reward Microservice

> [!NOTE]
> This service is currently **stable** but under development.

A **REST API** written in **Rust** handling **daily bonus claims** and **daily streaks** for **Bit Casino** - a virtual gambling simulator.

### Features

## How to use this repository

This service is not very useful on its own. It relies upon the [**Player Microservice**](https://github.com/b1gd3vd0g/bit-casino-player-ms), [**Currency Microservice**](https://github.com/b1gd3vd0g/bit-casino-currency-ms), and a **Redis** database.

To test this API alongside the whole environment, you can follow the instructions in the [Infrastructure](https://github.com/b1gd3vd0g/bit-casino-infra) repository to test all services locally using **Docker Compose**.

You can then interact via the frontend at `localhost:60000` or call the integrated reward microservice directly at `localhost:60602`.

## Functionality

The reward microservice currently supports the following functions:

- Check your daily bonus streak and availability.
- Claim your daily bonus when it is available.

## Related Repositories

- [Player Microservice](https://github.com/b1gd3vd0g/bit-casino-player-ms) - Handles account creation and player authentication.
- [Currency Microservice](https://github.com/b1gd3vd0g/bit-casino-currency-ms) - Handles bit wallet creation and safe transactions.
- [Slots Microservice](https://github.com/b1gd3vd0g/bit-casino-slots-ms) - Handles the backend for the custom slot machine game **Byte Builder**.
- [Frontend](https://github.com/b1gd3vd0g/bit-casino-frontend) - A react app creating a user-friendly interface with which to interact with the backend.
- [Infrastructure](https://github.com/b1gd3vd0g/bit-casino-infra) - Allows for integration testing locally using **docker compose**.
