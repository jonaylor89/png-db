# PNG Database Web Demo

This is a web-based demonstration of the PNG Database project, compiled to WebAssembly.

## Features

- **Create Database**: Create a new PNG database with custom schema
- **Load Database**: Upload and load existing PNG database files
- **Insert Data**: Add JSON data at specific coordinates
- **Query Data**: Search using SQL-like WHERE clauses
- **Download**: Download the database as a PNG file

## Usage

1. **Create a new database**:
   - Set width and height (e.g., 256x256)
   - Define schema in JSON format: `{"name": "string", "age": "number", "active": "boolean"}`
   - Click "Create Database"

2. **Insert some data**:
   - Enter coordinates (x, y)
   - Enter JSON data: `{"name": "Alice", "age": 30, "active": true}`
   - Click "Insert Data"

3. **Query the data**:
   - Use WHERE clauses like: `WHERE age > 25 AND active = true`
   - Or coordinate-based queries: `WHERE x > 100 AND y < 200`
   - Click "Run Query" or "List All Data"

4. **Download**:
   - Click "Download PNG Database" to save the database as a PNG file
   - The PNG file contains all your data in compressed text chunks

## Technical Details

- Built with Rust compiled to WebAssembly
- Uses the same core PNG database engine as the CLI version
- Stores JSON data in PNG zTXt (compressed text) chunks
- Works entirely in the browser - no server required

## Development

To rebuild the WASM package:

```bash
# From the project root
./build-wasm.sh

# Or manually:
wasm-pack build --target web --features wasm --no-default-features
cp -r pkg/* web/
```

To serve locally:

```bash
cd web
python3 -m http.server 8000
# Then open http://localhost:8000
```