#!/usr/bin/env bash
set -x
set -eo pipefail

# Check if `sqlx` has installed
if ! [ -x "$(command -v mysql)" ]; then
  echo >&2 "Error: mysql is not installed."
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
DB_USER="${MYSQL_USER:=root}"
DB_PASSWORD="${MYSQL_ROOT_PASSWORD:=kakeepoo}"
DB_NAME="${MYSQL_DB:=regagro_3_0}"
DB_PORT="${MYSQL_PORT:=3306}"
DB_HOST="${MYSQL_HOST:=127.0.0.1}"

# Launch MySql using Docker
if [[ -z "${SKIP_DOCKER}" ]]
then
  docker run\
  -e MYSQL_ROOT_PASSWORD=${DB_PASSWORD}\
  -e MYSQL_DATABASE=${DB_NAME}\
  -p "${DB_PORT}":3306\
  -d mysql
fi

# Keep pinging MySql until it's ready to accept commands
export MYSQL_PASSWORD="${DB_PASSWORD}"
until mysqladmin ping -h "${DB_HOST}"  --password=kakeepoo -P "${DB_PORT}"; do
 >&2 echo "MySQL is still unavalable -sleeping"
 sleep 5
done

>&2 echo "MySQL is up and running on port ${DB_PORT}"

DATABASE_URL=mysql://${DB_USER}:${DB_PASSWORD}@${DB_HOST}:${DB_PORT}/${DB_NAME}
export DATABASE_URL
sqlx database create
sqlx migrate run

>&2 echo "MySQL has been migrated, ready to go!"