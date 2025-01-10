# Rust backend with the postgreSQL

## Setup the database
Run Docker (PostgresSQL and pgAdmin4)
```
docker-compose up -d
```

Open browser
```
http://localhost:5050/
```
- Log-in via username and password in `docker-compose.yml`
- Select the table to display

## Run server
```
cd ~/app-register
cargo run
```

## API
- **GET**
```
curl http://localhost:3000/robot
```

- **POST**
```
curl -X POST http://0.0.0.0:3000/robot -H 'Content-Type: application/json' -d '{
    "serial": "ARVPOSTMAN112",
    "name": "Robot999",
    "organization": "ARV-EIEI",
    "robot_type": "F"
}'
```

- **PUT**
```
curl -X PUT http://0.0.0.0:3000/robot/ARVPOSTMAN112 -H 'Content-Type: application/json' -d '{
    "name": "Robot555",
    "organization": "ARV",
    "robot_type": "AIR"
}'
```

- **DELETE**
```
curl -X DELETE http://127.0.0.1:3000/robot/ARVPOSTMAN112
```