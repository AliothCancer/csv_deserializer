# csv_deserializer

### Abstract
`csv_deserializer` is a lightweight Rust library engineered for the ingestion and manipulation of small-to-medium-scale Comma-Separated Values (CSV) datasets. Its primary functional objective is to serve as an ergonomic bridge between raw CSV data and the broader Rust ecosystem—specifically `std`, `ndarray`, and `Polars` by abstracting the verbosity typically associated with these high-performance frameworks.

### Core Functionalities
* **Automated Type Inference**: The library autonomously identifies column data types, including integers, floating-point numbers, strings, and nullable variants.
* **Idiomatic Data Exposure**: Columns are exposed via standard library (`std`) types, minimizing the cognitive load required to interface with proprietary APIs.
* **Dynamic Heterogeneous Manipulation**: It facilitates streamlined operations such as type-based selection, mapping, and filtering, which are often syntactically cumbersome in more rigid structures like `ndarray`.
* **Interoperability**: Through the utilization of optional feature flags, the library enables seamless conversion to `ndarray` and `Polars` structures without imposing heavy dependency overhead on users who do not require such integrations.

### Scope and Limitations
The library is explicitly not engineered for high-concurrency performance or the processing of "Big Data" volumes, for which `Polars` suits. Furthermore, it avoids the implementation of complex statistical analysis or linear algebra, deferring those specialized computations to the `ndarray` ecosystem.

### Target Audience
The tool is optimized for developers managing modest datasets who require idiomatic data preprocessing and transformation. It is particularly effective for users seeking to prepare data for specialized analytical tools without the prerequisite of mastering the native, and often complex, I/O implementations of those tools.
