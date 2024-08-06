# Makefile

IMAGE_NAME=gcr.io/your-project-id/rss-feed-generator
CONTAINER_NAME=rss-feed-generator-container

# Build the Docker image
build:
	docker build -t $(IMAGE_NAME) .

# Push the Docker image to the Container Registry
push:
	docker push $(IMAGE_NAME)

# Run the Docker container locally
run:
	docker run --rm --name $(CONTAINER_NAME) -p 8080:8080 $(IMAGE_NAME)

logs:
	docker logs $(CONTAINER_NAME)

stop:
	docker stop $(CONTAINER_NAME)

# Shortcut for building and running the container
build-and-run: build run
