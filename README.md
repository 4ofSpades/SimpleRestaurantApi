# SimpleRestaurantApi

## Prerequisites
In order to run this project the following is needed:
- A Rust compiler 
- Docker (for Postgres)

## How to run
This repo contains 2 projects for the SimpleRestaurantApi: A client project and a server project. Both need to be compiled. 

The server makes use of a Postgres database. The simplest way to get this up and running is to instantiate a Docker container using the following command:
```
docker run --name simple-restaurant-api-db -p 7878:7878 -e POSTGRES_PASSWORD=mysecretpassword -d postgres
```