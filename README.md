# Karya Analytics

**Karya Analytics** is a powerful analytics platform that is part of the **Karya.dev** ecosystem. The main focus of this
project is to provide a robust system for creating and managing analytics workflows, with end-to-end support for event
management. It enables listening to events from multiple sources, applying transformations, filtering, and validation
rules, and finally, routing them to various destinations.

---

## Key Features

- **Event Source Integration**: Seamlessly listen to events from multiple supported sources (e.g., APIs, services, or
  other platforms).
- **Transformations**: Apply custom transformations to incoming events to map data into desired formats.
- **Filtering and Validation**: Define rules to filter out unwanted data and validate events based on specific
  conditions.
- **Event Routing**: Send processed data to various destinations, such as databases, messaging systems, or external
  services.
- **Scalability**: Built for handling high-volume, real-time data pipelines efficiently.
- **Extensibility**: Modular design to allow the addition of custom integrations and transformations.

---

## Use Cases

1. **Real-Time Analytics**: Process and visualize incoming data streams from various sources in near real-time.
2. **Event-Driven Systems**: Enable systems to react to specific business events as they are processed.
3. **ETL Pipelines**: Extract, transform, and load events for further business insights and decision-making.
4. **Data Validation/Compliance**: Validate incoming data for accuracy, consistency, or compliance with custom rules.

---

## Getting Started

**Karya Analytics** is organized as a monorepo, containing multiple related packages that work together. The main package is `karya-analytics`, located in the `packages/analytics` directory.

To start using **karya-analytics**, you can set up the system by following these steps:

### Prerequisites

- **Rust**: Ensure you have Rust 1.86.0 or later installed. You can get it
  from [Rust's official website](https://www.rust-lang.org/).
- **Cargo**: Included with Rust installation for dependency management and building.
- **Message broker/database (if needed)**: Depending on your setup, you may need technologies like Kafka, RabbitMQ, or a
  database system for destinations.

### Installation

1. Clone the repository:
   ```bash
   git clone https://github.com/dev-karya/analytics.git -o karya-analytics
   cd karya-analytics
   ```

2. Build all packages in the monorepo:
   ```bash
   cargo build --release
   ```

   Or build a specific package:
   ```bash
   cargo build --release -p karya-analytics
   ```

3. Run a specific package:
   ```bash
   cargo run -p karya-analytics
   ```

---

## Configuration

The **Karya Analytics** platform is highly configurable, allowing you to define the behavior for handling events.
Configuration includes:

- **Sources**: Specify where events should be listened from.
- **Transformations**: Define rules for transforming events into the desired format.
- **Filters**: Filter unwanted events based on predefined rules.
- **Destinations**: Define where processed events should be sent.

TODO: link to be added
Detailed configuration options can be found in the [docs](#).

---

## Project Structure

This project is organized as a monorepo, containing multiple related packages:

- **packages/analytics**: The main analytics package
  - **src**: Contains all application source code, including modules for event sources, transformations, filters, and
    destinations.
  - **tests**: Unit tests for ensuring functionality.
  - **examples**: Example configurations and usage scenarios.

The monorepo structure allows for better code sharing, consistent versioning, and simplified dependency management across related packages.

---

## Roadmap

The development roadmap for **Karya Analytics** is available in the [ROADMAP.md](ROADMAP.md) document. It outlines our plans for:

1. **Event Collector & ETL**: REST API support, event governance, lifecycle management, multi-source support, and event transformation.
2. **Event Streaming and Processing**: Kafka integration, event processor implementation, and destination management.
3. **Analytics Platform**: Custom analytics capabilities and future enhancements.

---

## Contributing

Contributions to **Karya Analytics** are welcome! To get started:

1. Fork the repository.
2. Create a new branch for your feature or bug fix.
3. Submit a pull request describing your changes.

---

## License

This project is licensed under the [MIT License](LICENSE). Feel free to use, modify, and distribute it as per the terms
of the license.

---

## About Karya.dev

**Karya.dev** is a platform designed to empower businesses with high-quality tools for managing content data, events,
and real-time workflows. For more information, visit [Karya.dev](https://karya.dev).
