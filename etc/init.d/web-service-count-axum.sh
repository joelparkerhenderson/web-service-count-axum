#!/bin/sh

### BEGIN INIT INFO
# Provides:          web-service-count-axum
# Required-Start:    $local_fs $network
# Required-Stop:     $local_fs
# Default-Start:     2 3 4 5
# Default-Stop:      0 1 6
# Short-Description: Web service count axum
# Description:       Web service example that provides a count function that is implemented via Rust Axum
### END INIT INFO

case "$1" in
  start)
    echo "Start web-service-count-axum"
    PORT=10001 /opt/web-service-count-axum/target/release/web-service-count-axum
    ;;
  stop)
    echo "Stop web-service-count-axum"
    pgrep '[w]eb-service-count-axum' | xargs kill
    ;;
  *)
    echo "Usage: /etc/init.d/web-service-count-axum {start|stop}"
    exit 1
    ;;
esac

exit 0
