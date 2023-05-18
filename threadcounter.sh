# !/bin/bash
# this scrips shows how many threads the tui is running, as it gets bumped up significantly when visiting a new page (it's downloading the immages)

while true; do
    PID=$(pgrep youtube-tui)

    if [ -n "$PID" ]; then
        THREADS=$(ps hH p $PID | wc -l)

        echo "youtube-tui is running with $THREADS threads."
    else
        echo "youtube-tui is not currently running."
    fi

    sleep 0.05
done
