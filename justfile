default: (deploy "silk")

set dotenv-load := true

pkg := "silk"

deploy pkg:
    cargo b --release --package {{pkg}}
    rsync -uvz  target/release/{{pkg}} ${REMOTE_ADDR}:~/.cargo/bin/{{pkg}}
    ssh ${REMOTE_ROOT_ADDR} systemctl restart {{pkg}}