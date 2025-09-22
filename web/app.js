import init, { WebPngDatabase } from './png_db.js';

let currentDatabase = null;
let wasmInitialized = false;

// Initialize WASM module
async function initWasm() {
    if (!wasmInitialized) {
        try {
            showLoading(true);
            await init();
            wasmInitialized = true;
            console.log('WASM module initialized');
        } catch (error) {
            showError(`Failed to initialize WASM: ${error}`);
        } finally {
            showLoading(false);
        }
    }
}

// Wait for DOM to be ready, then initialize
document.addEventListener('DOMContentLoaded', async () => {
    await initWasm();
});

window.createDatabase = async function() {
    if (!wasmInitialized) {
        await initWasm();
        if (!wasmInitialized) return;
    }

    try {
        showLoading(true);
        clearError();

        const width = parseInt(document.getElementById('width').value);
        const height = parseInt(document.getElementById('height').value);
        const schemaInput = document.getElementById('schema').value.trim();

        if (!schemaInput) {
            throw new Error('Schema is required');
        }

        // Validate JSON
        const schemaObj = JSON.parse(schemaInput);

        currentDatabase = new WebPngDatabase(width, height, schemaInput);

        showDatabaseInfo(`New database created (${width}x${height})`, schemaObj, 0);
        showSections(true);
        showResults(`✅ Database created successfully!`);

    } catch (error) {
        showError(`Failed to create database: ${error.message}`);
    } finally {
        showLoading(false);
    }
}

window.loadDatabase = async function(fileInput) {
    if (!wasmInitialized) {
        await initWasm();
        if (!wasmInitialized) return;
    }

    const file = fileInput.files[0];
    if (!file) return;

    try {
        showLoading(true);
        clearError();

        const arrayBuffer = await file.arrayBuffer();
        const uint8Array = new Uint8Array(arrayBuffer);

        currentDatabase = WebPngDatabase.from_png_bytes(uint8Array);

        const dimensions = currentDatabase.get_dimensions();
        const schema = JSON.parse(currentDatabase.get_schema());
        const rowCount = currentDatabase.get_row_count();

        showDatabaseInfo(`Database loaded: ${file.name}`, schema, rowCount, dimensions[0], dimensions[1]);
        showSections(true);
        showResults(`✅ Database loaded successfully! Found ${rowCount} rows.`);

    } catch (error) {
        showError(`Failed to load database: ${error.message}`);
        currentDatabase = null;
        showSections(false);
    } finally {
        showLoading(false);
    }
}

window.insertData = async function() {
    if (!currentDatabase) {
        showError('No database loaded');
        return;
    }

    try {
        showLoading(true);
        clearError();

        const x = parseInt(document.getElementById('insertX').value);
        const y = parseInt(document.getElementById('insertY').value);
        const dataInput = document.getElementById('insertData').value.trim();

        if (!dataInput) {
            throw new Error('Data is required');
        }

        // Validate JSON
        JSON.parse(dataInput);

        currentDatabase.insert(x, y, dataInput);

        const rowCount = currentDatabase.get_row_count();
        updateRowCount(rowCount);
        showResults(`✅ Data inserted at (${x}, ${y}). Total rows: ${rowCount}`);

        // Clear the form
        document.getElementById('insertData').value = '';

    } catch (error) {
        showError(`Failed to insert data: ${error.message}`);
    } finally {
        showLoading(false);
    }
}

window.queryData = async function() {
    if (!currentDatabase) {
        showError('No database loaded');
        return;
    }

    try {
        showLoading(true);
        clearError();

        const whereClause = document.getElementById('queryWhere').value.trim();
        if (!whereClause) {
            throw new Error('WHERE clause is required');
        }

        const resultsJson = currentDatabase.query(whereClause);
        const results = JSON.parse(resultsJson);

        if (results.length === 0) {
            showResults('No results found');
        } else {
            let html = `<h3>Found ${results.length} result${results.length === 1 ? '' : 's'}:</h3>`;
            results.forEach(row => {
                html += `
                    <div class="result-item">
                        <strong>Position (${row.x}, ${row.y}):</strong>
                        <pre>${JSON.stringify(row.data, null, 2)}</pre>
                    </div>
                `;
            });
            showResults(html);
        }

    } catch (error) {
        showError(`Query failed: ${error.message}`);
    } finally {
        showLoading(false);
    }
}

window.listAllData = async function() {
    if (!currentDatabase) {
        showError('No database loaded');
        return;
    }

    try {
        showLoading(true);
        clearError();

        const resultsJson = currentDatabase.list_all();
        const results = JSON.parse(resultsJson);

        if (results.length === 0) {
            showResults('Database is empty');
        } else {
            let html = `<h3>All ${results.length} row${results.length === 1 ? '' : 's'}:</h3>`;
            results.forEach(row => {
                html += `
                    <div class="result-item">
                        <strong>Position (${row.x}, ${row.y}):</strong>
                        <pre>${JSON.stringify(row.data, null, 2)}</pre>
                    </div>
                `;
            });
            showResults(html);
        }

    } catch (error) {
        showError(`Failed to list data: ${error.message}`);
    } finally {
        showLoading(false);
    }
}

window.downloadDatabase = async function() {
    if (!currentDatabase) {
        showError('No database loaded');
        return;
    }

    try {
        showLoading(true);
        clearError();

        const pngBytes = currentDatabase.to_png_bytes();
        const blob = new Blob([pngBytes], { type: 'image/png' });
        const url = URL.createObjectURL(blob);

        const a = document.createElement('a');
        a.href = url;
        a.download = 'database.png';
        a.click();

        URL.revokeObjectURL(url);
        showResults('✅ Database downloaded successfully!');

    } catch (error) {
        showError(`Failed to download database: ${error.message}`);
    } finally {
        showLoading(false);
    }
}

// Helper functions
function showLoading(show) {
    document.getElementById('loading').style.display = show ? 'block' : 'none';
}

function showError(message) {
    const errorDiv = document.getElementById('error');
    errorDiv.textContent = message;
    errorDiv.style.display = 'block';
}

function clearError() {
    const errorDiv = document.getElementById('error');
    errorDiv.style.display = 'none';
}

function showResults(content) {
    const resultsDiv = document.getElementById('results');
    if (typeof content === 'string') {
        resultsDiv.innerHTML = content;
    } else {
        resultsDiv.textContent = content;
    }
}

function showSections(show) {
    document.getElementById('insertSection').style.display = show ? 'block' : 'none';
    document.getElementById('querySection').style.display = show ? 'block' : 'none';
    document.getElementById('downloadSection').style.display = show ? 'block' : 'none';
}

function showDatabaseInfo(title, schema, rowCount, width = null, height = null) {
    const dbInfo = document.getElementById('dbInfo');
    let html = `<h3>${title}</h3>`;
    if (width && height) {
        html += `<p><strong>Dimensions:</strong> ${width}x${height}</p>`;
    }
    html += `<p><strong>Schema:</strong> <code>${JSON.stringify(schema)}</code></p>`;
    html += `<p><strong>Rows:</strong> <span id="rowCount">${rowCount}</span></p>`;
    dbInfo.innerHTML = html;
}

function updateRowCount(count) {
    const rowCountSpan = document.getElementById('rowCount');
    if (rowCountSpan) {
        rowCountSpan.textContent = count;
    }
}