# MicroCI/CD

## Overview

This is a simple Continuous Integration and Continuous Deployment (CICD) tool designed to automate the deployment process through straightforward configuration. The tool simplifies the setup and execution of deployment pipelines, ensuring that your code is automatically built, tested, and deployed to your environment with minimal effort.

## Features

- **Automated Deployments**: Automatically deploy your code based on predefined configurations.
- **Simple Configuration**: Set up deployment pipelines easily with a simple configuration file.
- **Flexibility**: Supports various deployment environments and stages.
- **Extensibility**: Easily extend the tool to include custom scripts or additional stages in your pipeline.

## Getting Started

### Prerequisites

- Ensure you have the necessary permissions and credentials to deploy to your target environment.
- Make sure your environment meets the system requirements (Rust).

### Installation

1. Clone the repository:
    ```bash
    git clone https://gitee.com/waterib/micro-cicd.git
    ```
2. Navigate to the project directory:
    ```bash
    cd micro-cicd
    ```
3. Run program:
    ```bash
    cargo run
    ```

### Usage

1. Modify the `config/task` file. The left side represents the task value, and the right side specifies the script to be executed.
    Example:
    ```properties
    webbuild=/usr/local/yourproject/pushandstart.sh start
    ngrestart=/usr/local/nginx/sbin/nginx restart
    ```
2. Configure GitHub, Gitee, or other code repository hooks to point to the URL.
    Example: `http://121.36.99.228:7777/task/webbuild`


The tool will execute the pipeline stages defined in your configuration file.

## Contributing

Contributions are welcome! Please open an issue or submit a pull request with your changes.

## Thanks

Thanks to [JetBrains](https://www.jetbrains.com/) for their support.

[RustRover](./assets/rr.png)