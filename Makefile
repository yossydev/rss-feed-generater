# Makefile

# Docker image name
IMAGE_NAME=gcr.io/your-project-id/rss-feed-generator

# Build the Docker image
build:
	docker build -t $(IMAGE_NAME) .

# Push the Docker image to the Container Registry
push:
	docker push $(IMAGE_NAME)

# Run the Docker container locally
run:
	docker run --rm -p 8080:8080 $(IMAGE_NAME)

# Shortcut for building and running the container
build-and-run: build run
