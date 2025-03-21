# Use the Alpine base image
FROM docker.io/alpine:latest

# Install garage during build time
RUN apk add --no-cache garage

# Set the ENTRYPOINT to run "garage server" when the container starts
# ENTRYPOINT ["garage", "server"]

