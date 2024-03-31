# Loadbalancer

## Description

The goal of this project is to create a load balancer (like the name 👀) to understand some applications like nginx or traefik load balancer

*the creation is only for my personal knowledge to understand rust and there ecosystem like the tokio runtime is better to use [pingora loadbalancer](https://github.com/cloudflare/pingora) for production service*

## Tasks

- [x] Install tokio runtime
- [x] Create a tcp server to accept request
- [x] Create a Loadbalancer implementation
- [x] Create bidirectional connection with tokio io
- [ ] Add cli configuration
- [ ] Use **-p** or **--port** to customize the port of the server
- [ ] Use **--servers** to add servers for load balancing
