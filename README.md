# Yossy.dev Blog RSS Feed Generator

This project generates an RSS feed for the yossy.dev blog. It uses Rust and Actix-web to scrape the latest articles from the blog and outputs them in a standard RSS feed format.

## Features

- Automatically scrapes the latest articles from the blog
- Provides article information in RSS feed format
- Accessible via a web server for RSS feed access

## Technology Stack

- Language: Rust
- Framework: Actix-web
- Other Tools: scraper, chrono, ureq

## Setup

The project is run using Docker.

### Clone the Project

```bash
git clone git@github.com:yossydev/rss-feed-generator.git
cd rss-feed-generator
```

### Compilation

```bash
make build
```

### Starting the Local Server

```bash
make run
```

## Deployment

This project is intended to be deployed on Fly.io. A fly.toml is included in the project, so you can deploy it using the Fly.io CLI tool.

### Login to Fly.io

```bash
flyctl auth login
```

### Deploy the Application

```bash
flyctl deploy
```

## CI/CD

This project can be integrated with Fly.io's CI/CD pipeline. Set FLY_API_TOKEN as a secret in your CI/CD environment and use it in the deployment process.

```bash
flyctl auth token
```

## License

This project is published under the MIT license.
