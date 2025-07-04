# LrnGit-daemon

Daemon for my git clone.

Create a lrngitd.service file in /etc/systemd/ and put this content:

```
[Unit]
Description=lrngit daemon
After=network.target
StartLimitIntervalSec=0
[Service]
Type=simple
Restart=always
RestartSec=1
User=ubuntu
ExecStart=/home/ubuntu/.cargo/bin//lrngitd
NoNewPrivileges=false
PrivateTmp=false
ProtectSystem=full
ProtectHome=false
Environment="PATH=/usr/local/sbin:/usr/local/bin:/usr/sbin:/usr/bin:/sbin:/bin:/usr/games:/usr/local/games:/snap/bin:/home/ubuntu/.cargo/bin/"

# for network socket
CapabilityBoundingSet=CAP_NET_BIND_SERVICE
AmbientCapabilities=CAP_NET_BIND_SERVICE

[Install]
WantedBy=multi-user.target
```
