# Smolbin
A lightweight pastebin alternative that also supports link shortening.

## Features
- Pastes
- Url shortening
- Automatic expiry

## Setup
- Get the [latest](https://github.com/Sushi-Mampfer/Smolbin/releases/latest) release
- Set the `PORT` env(or leave it unset for port 8080)
- (If you're using nest you can use [nest-setup](https://github.com/Sushi-Mampfer/nest-setup) to create the service for you)
- Run it

## Service(for linux)
(If you're using nest you can use [nest-setup](https://github.com/Sushi-Mampfer/nest-setup) to create the service for you) \
Don't forget to replace the paths and port.
```
[Unit]
Description=smolbin

[Service]
WorkingDirectory=%h/smolbin
Environment="PORT=xxx"
[Service]
ExecStart=%h/smolbin/smolbin
Restart=on-failure

[Install]
WantedBy=default.target
```