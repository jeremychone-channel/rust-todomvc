
Rust Programming Web Development Tutorial for building a simple Web Application, todoMVC, from scratch with WARP, SQLX, Database (PostgreSQL), and Native Web Components.

YouTube tutorials: 

- Part 1 (database / model access) - https://youtu.be/VIig9IcQ-w8
- Part 2 (web / warp filters) - https://youtu.be/plKzUo8F6Mg
- Part 3 (frontend / #FrameworkLess) - https://youtu.be/DkR0tCBPqYc

## Run the example

```sh
# Terminal 1 - start postgresql
docker run --rm -p 5432:5432 -e "POSTGRES_PASSWORD=postgres" --name pg postgres:14

# Terminal 2 - build frontend
cd frontend
npm run build

# Terminal 3 - build backend
cd backend
cargo run -- ../frontend/web-folder
```

- Those terminals can be part of the VSCode terminals
- Watch commands below for live development

## Dev Test 

```sh
# Test for model
cargo watch -q -c -w src/ -x 'test model_ -- --test-threads=1 --nocapture'

# Test for web
cargo watch -q -c -w src/ -x 'test web_ -- --test-threads=1 --nocapture'
```

## Live Dev

```sh
# Terminal 1 - build & watch the backend code
cd backend
cargo watch -q -c -w src/ -x 'run -- ../frontend/web-folder'

# Terminal 2 - build & watch the frontend
cd frontend
npm build -- -w
```

## DB

```sh
# Start the database
docker run --rm -p 5432:5432 -e "POSTGRES_PASSWORD=postgres" --name pg postgres:14

# optional psql (other terminal) 
docker exec -it -u postgres pg psql
```

## License - MIT OR Apache-2.0