#!/bin/bash

case "$1" in
  start)
    echo "Starting PostgreSQL database..."
    docker compose up -d
    echo "PostgreSQL is now running at localhost:5432"
    ;;
  stop)
    echo "Stopping PostgreSQL database..."
    docker compose stop
    ;;
  restart)
    echo "Restarting PostgreSQL database..."
    docker compose restart
    ;;
  reset)
    echo "Resetting PostgreSQL database (this will delete all data)..."
    docker compose down -v
    docker compose up -d
    echo "PostgreSQL has been reset and is now running at localhost:5432"
    ;;
  status)
    docker compose ps
    ;;
  *)
    echo "Usage: $0 {start|stop|restart|reset|status}"
    exit 1
    ;;
esac

exit 0
