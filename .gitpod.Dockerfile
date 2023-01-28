FROM gitpod/workspace-full:latest

# Install custom tools, runtime, etc.
RUN brew install exercism
RUN exercism configure --token=470b6230-f9f3-4ce7-8279-8ec1f4357a37
RUN exercism configure -w /workspace/exercismUN exercism configure -w /workspace/exercism