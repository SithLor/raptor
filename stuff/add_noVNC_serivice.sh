#!/bin/bash

# Variables
SERVICE_FILE="/etc/systemd/system/novnc.service"
NOVNC_EXEC="/usr/bin/noVNC/utils/novnc_proxy"
VNC_ADDRESS="localhost:5900"
LISTEN_PORT="6080"
USERNAME="robotics"

# Check if the noVNC executable exists
if [ ! -f "$NOVNC_EXEC" ]; then
    echo "noVNC executable not found at $NOVNC_EXEC. Please install noVNC."
    exit 1
fi

# Create the systemd service file
echo "Creating systemd service file..."
sudo bash -c "cat > $SERVICE_FILE <<EOL
[Unit]
Description=noVNC Service
After=network.target

[Service]
Type=simple
ExecStart=$NOVNC_EXEC --vnc $VNC_ADDRESS --listen $LISTEN_PORT
User=$USERNAME
Restart=on-failure

[Install]
WantedBy=multi-user.target
EOL"

# Reload the systemd daemon
echo "Reloading systemd daemon..."
sudo systemctl daemon-reload

# Enable the noVNC service
echo "Enabling noVNC service..."
sudo systemctl enable novnc.service

# Start the noVNC service
echo "Starting noVNC service..."
sudo systemctl start novnc.service

# Check the service status
echo "Checking noVNC service status..."
sudo systemctl status novnc.service

echo "noVNC service setup complete."