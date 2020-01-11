# Docker image exposing Huawei B525 router VPN features as a Web API

[![Build Status](https://travis-ci.org/jeje/huawei-b525-vpn-connection-api.svg?branch=master)](https://travis-ci.org/rust-lang-nursery/failure)

Huawei's B525 routers don't expose APIs for VPN management.

This Docker image uses a Chrome headless browser in order to mimick user
interactions with the VPN in the admin console pages of the router.

## API Documentation

The API is fairly simple. It does *NOT* respect REST principles (especially HTTP)
verbs, on purpose, in order to be able to control this API for a Home Automation
solution in an easier way.

### Get VPN connection status
* URL: http://localhost:8000/vpn
* Result: `0` (disconnected) or `1` (connected)

### Establish VPN connection
* URL: http://localhost:8000/vpn/activate
* Result: `"OK"`
* Notes: the VPN connection settings have to be setup priorly

### Disconnect VPN connection
* URL: http://localhost:8000/vpn/deactivate
* Result: `"OK"`

## Setup

A few environment variables are needed in order to use this Docker image:

* `ROUTER_IP`: the IP address of the router
* `ROUTER_LOGIN`: the login of the user
* `ROUTER_PASSWORD`: the password of the user 

This container runs a Web API on port `8000` that will need to be mapped to the
Docker host.

Typical run instructions:

```shell
docker run -d --memory=512m \
        -e ROUTER_IP=<YOUR_ROUTER_IP> \
        -e ROUTER_LOGIN=<YOUR_ROUTER_LOGIN> \
        -e ROUTER_PASSWORD=<YOUR_ROUTER_PASSWORD> \
        -p 8000:8000 \
        jeje/huawei-b525-vpn-connection-api
```

## License

This project is licensed under the terms of the MIT License or the Apache License
2.0, at your choosing.
