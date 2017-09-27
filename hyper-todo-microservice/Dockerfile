FROM scorpil/rust:nightly

WORKDIR /app

ADD . /app

RUN cargo build --verbose

EXPOSE 8000

CMD ["cargo", "run"]
