#!/usr/bin/env bash
set -x
set -eo pipefail

# Check if `sqlx` has installed
if ! [ -x "$(command -v psql)" ]; then
  echo >&2 "Error: psql is not installed."
  exit 1
fi

# Check if `sqlx` has been installed
if ! [ -x "$(command -v sqlx)" ]; then
  echo >&2 "Error: sqlx is not installed."
  echo >&2 "Use:"
  echo >&2 "  cargo install --version='~0.7.1' sqlx-cli \
--no-default-features --features rustls,postgres,mysql"
  echo >&2 "to install it"
  exit 1
fi

# Setup constants
DB_USER="${POSTGRESS_USER:=postgres}"
DB_PASSWORD="${POSTGRES_PASSWORD:=password}"
DB_NAME="${POSTGRES_DB:=newsletter}"
DB_PORT="${POSTGRES_PORT:=5435}"
DB_HOST="${POSTGRES_HOST:=localhost}"

# Launch postgres using Docker
if [[ -z "${SKIP_DOCKER}" ]]
then
  docker run\
  -e POSTGRES_USER=${DB_USER}\
  -e POSTGRES_PASSWORD=${DB_PASSWORD}\
  -e POSTGRES_DB=${DB_NAME}\
  -p "${DB_PORT}":5432\
  -d postgres -N 1000
fi

# Keep pinging Postgres until it's ready to accept commands
export PGPASSWORD="${DB_PASSWORD}"
until psql -h "${DB_HOST}" -U "${DB_USER}" -p ${DB_PORT} -d "postgres" -c '\q'; do
 >&2 echo "Postgres is still unavalable -sleeping"
 sleep 5
done

>&2 echo "Postgres is up and running on port ${DB_PORT}"

DATABASE_URL=postgres://${DB_USER}:${DB_PASSWORD}@${DB_HOST}:${DB_PORT}/${DB_NAME}
export DATABASE_URL
sqlx database create
sqlx migrate run

>&2 echo "Postgres has been migrated, ready to go!"