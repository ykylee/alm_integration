FROM docker.io/library/python:3.12-slim

ENV PYTHONDONTWRITEBYTECODE=1
ENV PYTHONUNBUFFERED=1

WORKDIR /workspace

RUN apt-get update \
  && apt-get install -y --no-install-recommends build-essential \
  && rm -rf /var/lib/apt/lists/*

CMD ["bash", "-lc", "python -m pip install --upgrade pip && python -m pip install -e '.[dev]' && pytest"]
