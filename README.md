cargo add sqlx shuttle-shared-db --features sqlx/macros,shuttle-shared-db/postgres,shuttle-shared-db/sqlx
cargo install sqlx-cli

rustup component add rustfmt
rustup component add clippy

sqlx migrate add create_users
sqlx migrate add create_images

## Docker
https://docs.docker.com/engine/install/ubuntu/

# https://docs.docker.com/engine/install/linux-postinstall/
sudo usermod -aG docker $USER


python3 -c 'import secrets, string; print("API Secret:", "".join(secrets.choice(string.ascii_letters + string.digits + string.punctuation) for _ in range(64)))'
