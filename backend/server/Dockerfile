FROM rust:1.81.0

WORKDIR /usr/src/dashishoyu
COPY . .

RUN apt -y update \
    && apt install -y jq \
    && cargo install sea-orm-cli \
    && chmod 777 init.sh

CMD ["bash", "cargo", "run", "--bin", "server"]

ENTRYPOINT ["/usr/src/dashishoyu/init.sh"]
