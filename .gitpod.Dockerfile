# You can find the new timestamped tags here: https://hub.docker.com/r/gitpod/workspace-full/tags
FROM gitpod/workspace-full:2023-01-16-03-31-28

# Install custom tools, runtime, etc.
RUN brew install exercism
RUN exercism configure --token=470b6230-f9f3-4ce7-8279-8ec1f4357a37
RUN exercism configure -w /workspace/exercism