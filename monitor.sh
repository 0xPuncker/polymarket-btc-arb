#!/bin/bash
# Monitor polymarket-btc-arb logs

# Check if process is running
ps aux | grep polymarket_btc_arb | grep -v grep

# Follow live logs (if running in background)
if [ -f /tmp/polymarket-btc-arb.log ]; then
    tail -f /tmp/polymarket-btc-arb.log
else
    echo "No log file found. Process may be running without log output redirection."
fi

# Or use process tool:
# process poll ember-shore
