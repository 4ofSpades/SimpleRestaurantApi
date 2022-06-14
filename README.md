# SimpleRestaurantApi

## Prerequisites
In order to run this project the following is needed:
- A Rust compiler 
- Docker (for Postgres)

*Note: to keep things simple the addresses and ports used cannot be chosen by the user. In order for this application to be able to run, localhost ports `5050`, `7878`, and `55004` should not be used by other processes.*

## How to run
This section contains the steps on how to run the application and see what it is doing.

### Postgres setup
This application makes use of a PostgreSQL database. For demostration purposes, a Docker container is used for this. In order to set up and start the database container, run the following command:
```
 docker run --name some-postgres -p 55004:5432 -e POSTGRES_HOST_AUTH_METHOD=trust -d postgres
```

In case this container has already been created, simply use the `docker start` command instead.

To observe the database data, a Docker container for PgAdmin4 can be created using the following command:
```
docker run -p 5050:80 -e PGADMIN_DEFAULT_EMAIL=pgadmin4@pgadmin.org -e PGADMIN_DEFAULT_PASSWORD=admin -d --name pgadmin4 dpage/pgadmin4
```

Once up and running, use a web browser to open PgAdmin at `http://localhost:5050/`, using `pgadmin4@pgadmin.org` as username/email and `admin` as password. Click the option to register a new server (any name suffices) and fill in the following on the connection tab:
- Host: `host.docker.internal`
- Port: The port of the postgres container
- Username: The username of the postgres container, default is 'postgres'
- Password: The password of the postgres container

### Compilation
This repo contains 2 projects for the SimpleRestaurantApi: A client project and a server project. To get started, compile and run the project in the folder `server`. This, as the name implies, should start the server. Within this process, the server will connect to the database and create necessary tables for it. Once the database is ready for storing orders, the server binds to an address that can be used by clients to send HTTP requests to. A simple test would be to again open a web browser and go to `http://localhost:7878`.

Next up is the client. Essentially the only thing the client provides is a set of methods to make API calls to the server with, so that it can be used by other implementations (such as a frontend or another service). However, for this demonstration the main file can be used to simulate calls with. A default set of operations has already been added to it, just like any additonal service would make use of it, any calls provided by the client module could be called here in any sequence (feel free to play around with it). Once the client has been compiled and starts running, the terminal running the server will print out it having received a request.

## Developer notes
The docs folder contains a version of database that is more suitable for production. If compared with the simplified version, it can be seen that some values are slightly outdated (e.g. created_at uses TIMESTAMP now), but the design should for the most part remain the same. Due to time constraints, I have not created a specific test class, but used the client project main function as a simple test instead.

I recommend looking over the previous commits too. There were several problems with certain packages, hence why the design changd significantly over time. That is also the reason why I used certain primitive types that are probably larger than necessary is mainly because of constraints by tokio_postgres.

## Update notes
*Update 2022/06/15*
I added a skeleton for tests as every production ready part should include it. I'll do my best to implement meaningful tests as soon as possible. For the time being, I'll include test changes to a separate branch.
