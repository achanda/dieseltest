Run this by

```
docker-compose down -v && docker-compose up --build
```

Then create a new user

```
curl -X POST http://localhost:3000/users \
    -H "Content-Type: application/json" \
    -d '{"name":"John Doe","email":"john.doe@example.com"}'
```

Sample error

```
ieseltest-postgres-1   | 2025-03-08 04:42:24.273 UTC [1] LOG:  starting PostgreSQL 15.12 (Debian 15.12-1.pgdg120+1) on aarch64-unknown-linux-gnu, compiled by gcc (Debian 12.2.0-14) 12.2.0, 64-bit
dieseltest-postgres-1   | 2025-03-08 04:42:24.274 UTC [1] LOG:  listening on IPv4 address "0.0.0.0", port 5432
dieseltest-postgres-1   | 2025-03-08 04:42:24.274 UTC [1] LOG:  listening on IPv6 address "::", port 5432
dieseltest-postgres-1   | 2025-03-08 04:42:24.275 UTC [1] LOG:  listening on Unix socket "/var/run/postgresql/.s.PGSQL.5432"
dieseltest-postgres-1   | 2025-03-08 04:42:24.277 UTC [64] LOG:  database system was shut down at 2025-03-08 04:42:24 UTC
dieseltest-postgres-1   | 2025-03-08 04:42:24.279 UTC [1] LOG:  database system is ready to accept connections
dieseltest-pgbouncer-1  | Starting /usr/bin/pgbouncer /etc/pgbouncer/pgbouncer.ini...
dieseltest-pgbouncer-1  | 2025-03-08 04:42:29.279 UTC [1] LOG kernel file descriptor limit: 1048576 (hard: 1048576); max_client_conn: 100, max expected fd use: 132
dieseltest-pgbouncer-1  | 2025-03-08 04:42:29.279 UTC [1] LOG listening on 0.0.0.0:6432
dieseltest-pgbouncer-1  | 2025-03-08 04:42:29.280 UTC [1] LOG listening on unix:/tmp/.s.PGSQL.6432
dieseltest-pgbouncer-1  | 2025-03-08 04:42:29.280 UTC [1] LOG process up: PgBouncer 1.24.0, libevent 2.1.12-stable (epoll), adns: evdns2, tls: OpenSSL 3.3.3 11 Feb 2025
dieseltest-pgbouncer-1  | 2025-03-08 04:42:29.454 UTC [1] LOG C-0xffffa58263e0: demo_db/db_user@192.168.0.4:57390 login attempt: db=demo_db user=db_user tls=no replication=no
dieseltest-pgbouncer-1  | 2025-03-08 04:42:29.460 UTC [1] LOG S-0xffffa57ae5e0: demo_db/db_user@192.168.0.2:5432 new connection to server (from 192.168.0.3:33330)
dieseltest-rust_app-1   | Running migration 2023-01-01-000000_create_users
dieseltest-pgbouncer-1  | 2025-03-08 04:42:29.471 UTC [1] LOG C-0xffffa58263e0: demo_db/db_user@192.168.0.4:57390 closing because: client close request (age=0s)
dieseltest-rust_app-1   | 2025-03-08T04:42:29.480523Z  INFO rust_app: Logging system initialized successfully
dieseltest-rust_app-1   | 2025-03-08T04:42:29.480535Z  INFO rust_app: Retrieving DATABASE_URL from environment...
dieseltest-rust_app-1   | 2025-03-08T04:42:29.480537Z  INFO rust_app: Using database URL: postgres://db_user:db_password@pgbouncer:6432/demo_db
dieseltest-rust_app-1   | 2025-03-08T04:42:29.480539Z  INFO rust_app: Creating database connection manager...
dieseltest-rust_app-1   | 2025-03-08T04:42:29.480540Z  INFO rust_app: Connection manager created successfully
dieseltest-rust_app-1   | 2025-03-08T04:42:29.480541Z  INFO rust_app: Building database connection pool with PgBouncer settings...
dieseltest-pgbouncer-1  | 2025-03-08 04:42:29.481 UTC [1] LOG C-0xffffa58263e0: demo_db/db_user@192.168.0.4:57394 login attempt: db=demo_db user=db_user tls=no replication=no
dieseltest-rust_app-1   | 2025-03-08T04:42:29.483326Z  INFO rust_app: Connection pool created successfully with max_size=10, min_idle=1
dieseltest-rust_app-1   | 2025-03-08T04:42:29.483340Z  INFO rust_app: Testing database connection...
dieseltest-rust_app-1   | 2025-03-08T04:42:29.484119Z  INFO rust_app: Successfully connected to database
dieseltest-rust_app-1   | 2025-03-08T04:42:29.484131Z  INFO rust_app: Attempting to create fake users for testing...
dieseltest-rust_app-1   | 2025-03-08T04:42:29.484133Z  INFO rust_app: Starting fake user creation process
dieseltest-rust_app-1   | 2025-03-08T04:42:29.484616Z  INFO rust_app: Successfully acquired database connection for fake user creation
dieseltest-rust_app-1   | 2025-03-08T04:42:29.484654Z  INFO rust_app: Generating 10 fake user records...
dieseltest-rust_app-1   | 2025-03-08T04:42:29.484687Z  INFO rust_app: Successfully generated 10 fake user records in memory
dieseltest-pgbouncer-1  | 2025-03-08 04:42:29.485 UTC [1] LOG C-0xffffa58266a0: demo_db/db_user@192.168.0.4:57396 login attempt: db=demo_db user=db_user tls=no replication=no
dieseltest-rust_app-1   | 2025-03-08T04:42:29.484695Z  INFO rust_app: Inserting fake users in batches of 5...
dieseltest-rust_app-1   | 2025-03-08T04:42:29.484813Z  INFO rust_app: Processing batch 1 with 5 users
dieseltest-pgbouncer-1  | 2025-03-08 04:42:29.485 UTC [1] LOG S-0xffffa57ae8a0: demo_db/db_user@192.168.0.2:5432 new connection to server (from 192.168.0.3:33336)
dieseltest-rust_app-1   | 2025-03-08T04:42:29.486275Z  INFO rust_app: Successfully inserted 5 users in batch 1
dieseltest-rust_app-1   | 2025-03-08T04:42:29.486339Z  INFO rust_app: Processing batch 2 with 5 users
dieseltest-postgres-1   | 2025-03-08 04:42:29.487 UTC [76] ERROR:  unnamed prepared statement does not exist
dieseltest-pgbouncer-1  | 2025-03-08 04:42:29.487 UTC [1] LOG C-0xffffa58266a0: demo_db/db_user@192.168.0.4:57396 closing because: client close request (age=0s)
dieseltest-postgres-1   | 2025-03-08 04:42:29.488 UTC [76] ERROR:  unnamed prepared statement does not exist
dieseltest-rust_app-1   | 2025-03-08T04:42:29.488612Z ERROR r2d2: unnamed prepared statement does not exist
dieseltest-rust_app-1   | 2025-03-08T04:42:29.488642Z ERROR rust_app: Failed to insert user batch 2: unnamed prepared statement does not exist
dieseltest-rust_app-1   | 2025-03-08T04:42:29.491078Z  INFO rust_app: Continuing with next batch instead of returning error
dieseltest-rust_app-1   | 2025-03-08T04:42:29.491081Z  INFO rust_app: Attempting to count total users in database...
dieseltest-pgbouncer-1  | 2025-03-08 04:42:29.492 UTC [1] LOG C-0xffffa5826960: demo_db/db_user@192.168.0.4:57400 login attempt: db=demo_db user=db_user tls=no replication=no
dieseltest-rust_app-1   | 2025-03-08T04:42:29.493167Z  INFO rust_app: Successfully created 5 fake users in total
dieseltest-rust_app-1   | 2025-03-08T04:42:29.493317Z  INFO rust_app: Fake user creation process completed
dieseltest-rust_app-1   | 2025-03-08T04:42:29.493545Z  INFO rust_app: Creating shared application state with database pool...
dieseltest-rust_app-1   | 2025-03-08T04:42:29.493682Z  INFO rust_app: Application state created successfully
dieseltest-rust_app-1   | 2025-03-08T04:42:29.493717Z  INFO rust_app: Configuring CORS policy...
dieseltest-rust_app-1   | 2025-03-08T04:42:29.493963Z  INFO rust_app: CORS configured to allow any origin, method, and header
dieseltest-rust_app-1   | 2025-03-08T04:42:29.494186Z  INFO rust_app: Building application router with routes...
dieseltest-rust_app-1   | 2025-03-08T04:42:29.494484Z  INFO rust_app: Router configured with /users GET/POST and /health endpoints
dieseltest-rust_app-1   | 2025-03-08T04:42:29.494492Z  INFO rust_app: Preparing to start server on 0.0.0.0:3000
dieseltest-rust_app-1   | 2025-03-08T04:42:29.494494Z  INFO rust_app: Binding to address 0.0.0.0:3000...
dieseltest-rust_app-1   | 2025-03-08T04:42:29.494724Z  INFO rust_app: Successfully bound to 0.0.0.0:3000
dieseltest-rust_app-1   | 2025-03-08T04:42:29.494738Z  INFO rust_app: Starting HTTP server... 🚀
dieseltest-postgres-1   | 2025-03-08 04:42:29.495 UTC [77] ERROR:  unnamed prepared statement does not exist
dieseltest-rust_app-1   | 2025-03-08T04:42:29.495450Z ERROR r2d2: unnamed prepared statement does not exist
dieseltest-pgbouncer-1  | 2025-03-08 04:42:29.495 UTC [1] LOG C-0xffffa5826960: demo_db/db_user@192.168.0.4:57400 closing because: client close request (age=0s)
dieseltest-pgbouncer-1  | 2025-03-08 04:42:29.903 UTC [1] LOG C-0xffffa5826960: demo_db/db_user@192.168.0.4:57404 login attempt: db=demo_db user=db_user tls=no replication=no
dieseltest-pgbouncer-1  | 2025-03-08 04:42:29.905 UTC [1] LOG C-0xffffa58266a0: demo_db/db_user@192.168.0.4:57402 login attempt: db=demo_db user=db_user tls=no replication=no
dieseltest-postgres-1   | 2025-03-08 04:42:29.910 UTC [76] ERROR:  unnamed prepared statement does not exist
dieseltest-postgres-1   | 2025-03-08 04:42:29.911 UTC [77] ERROR:  unnamed prepared statement does not exist
dieseltest-pgbouncer-1  | 2025-03-08 04:42:29.912 UTC [1] LOG C-0xffffa5826960: demo_db/db_user@192.168.0.4:57404 closing because: client close request (age=0s)
dieseltest-rust_app-1   | 2025-03-08T04:42:29.912843Z ERROR r2d2: unnamed prepared statement does not exist
dieseltest-pgbouncer-1  | 2025-03-08 04:42:29.915 UTC [1] LOG C-0xffffa58266a0: demo_db/db_user@192.168.0.4:57402 closing because: client close request (age=0s)
dieseltest-rust_app-1   | 2025-03-08T04:42:29.915661Z ERROR r2d2: unnamed prepared statement does not exist
dieseltest-pgbouncer-1  | 2025-03-08 04:42:30.720 UTC [1] LOG C-0xffffa5826960: demo_db/db_user@192.168.0.4:57406 login attempt: db=demo_db user=db_user tls=no replication=no
dieseltest-pgbouncer-1  | 2025-03-08 04:42:30.720 UTC [1] LOG C-0xffffa58266a0: demo_db/db_user@192.168.0.4:57408 login attempt: db=demo_db user=db_user tls=no replication=no
dieseltest-postgres-1   | 2025-03-08 04:42:30.727 UTC [77] ERROR:  unnamed prepared statement does not exist
dieseltest-rust_app-1   | 2025-03-08T04:42:30.727918Z ERROR r2d2: unnamed prepared statement does not exist
dieseltest-pgbouncer-1  | 2025-03-08 04:42:30.727 UTC [1] LOG C-0xffffa5826960: demo_db/db_user@192.168.0.4:57406 closing because: client close request (age=0s)
dieseltest-pgbouncer-1  | 2025-03-08 04:42:30.728 UTC [1] LOG C-0xffffa58266a0: demo_db/db_user@192.168.0.4:57408 closing because: client close request (age=0s)
dieseltest-postgres-1   | 2025-03-08 04:42:30.728 UTC [76] ERROR:  unnamed prepared statement does not exist
dieseltest-rust_app-1   | 2025-03-08T04:42:30.728847Z ERROR r2d2: unnamed prepared statement does not exist
dieseltest-pgbouncer-1  | 2025-03-08 04:42:32.234 UTC [1] LOG C-0xffffa58266a0: demo_db/db_user@192.168.0.4:57412 login attempt: db=demo_db user=db_user tls=no replication=no
dieseltest-pgbouncer-1  | 2025-03-08 04:42:32.237 UTC [1] LOG C-0xffffa5826960: demo_db/db_user@192.168.0.4:57410 login attempt: db=demo_db user=db_user tls=no replication=no
```
