# Bitcoin Block Height Explorer

This project demonstrates a minimal end-to-end flow of a Bitcoin block height explorer. It includes:

- A Rust client to fetch the current Bitcoin block height.
- A Python ingestion script to store block heights in a SQLite database.
- A Flask web application to display the latest block height.

## Prerequisites

- Bitcoin Core
- Rust and Cargo
- Python 3.x
- SQLite

## Setup Instructions

### Step 1: Install and Configure Bitcoin Core

1. **Download and Install Bitcoin Core**:
   - Download from [Bitcoin Core website](https://bitcoin.org/en/download).
   - Install and run Bitcoin Core.

2. **Configure Bitcoin Core**:
   - Locate the `bitcoin.conf` file, typically found in `/Users/<user>/Library/Application Support/Bitcoin` on Mac.
   - Add the following configuration to `bitcoin.conf`:
     ```plaintext
     server=1
     rpcuser=username
     rpcpassword=password
     rpcport=8332
     rpcallowip=127.0.0.1
     ```

### Step 2: Setup Rust and Cargo

1. **Install Rust and Cargo**:
   - Follow the instructions at [rust-lang.org](https://www.rust-lang.org/tools/install).

2. **Create the Rust Client**:
   - Create a new Rust project:
     ```
     cargo new rust_client
     cd rust_client
     ```

3. **Edit `Cargo.toml`**:
   - Add dependencies:
     ```toml
     [dependencies]
     bitcoincore-rpc = "0.19.0"
     ```

4. **Edit `src/main.rs`**:
   - Replace the contents with the following code:
     ```rust
     use bitcoincore_rpc::{Auth, Client, RpcApi};

     fn main() {
         let rpc = Client::new(
             "http://127.0.0.1:8332",
             Auth::UserPass("yourrpcuser".to_string(), "yourrpcpassword".to_string()),
         ).expect("Error creating RPC client");

         match rpc.get_block_count() {
             Ok(block_count) => println!("Block count: {}", block_count),
             Err(e) => eprintln!("Error getting block count: {}", e),
         }
     }
     ```

5. **Build the Rust Client**:
   - Build the executable:
     ```
     cargo build --release
     ```

### Step 3: Create SQLite Database and Table

1. **Create a SQLite Database and Table**:
   - Run the following script to create the database and table:
     ```python
     import sqlite3

     DB_PATH = 'blockchain.db'

     conn = sqlite3.connect(DB_PATH)
     cursor = conn.cursor()
     cursor.execute('''
     CREATE TABLE IF NOT EXISTS blockchain (
         id INTEGER PRIMARY KEY AUTOINCREMENT,
         block_height INTEGER NOT NULL
     )
     ''')
     conn.commit()
     conn.close()
     ```

### Step 4: Setup Python Ingestion Script

1. **Install Required Python Libraries**:
   - Install SQLite3 (part of Python standard library) and Flask:
     ```
     pip install flask
     ```

2. **Create `ingest.py`**:
   - Add the following content:
    ```python
    import sqlite3
    import subprocess
    import time
    DB_PATH = 'blockchain.db'

    def get_block_height():
        result = subprocess.run(['/Users/kshetrahegde/Downloads/Bitcoin-rust/rust_client/target/release/./rust_client'], stdout=subprocess.PIPE, stderr=subprocess.PIPE)
        if result.returncode != 0:
            print(f"Error: {result.stderr.decode('utf-8')}")
            return None
        return int(result.stdout.decode('utf-8').strip().split(': ')[1])
    def insert_block_height(block_height):
        conn = sqlite3.connect(DB_PATH)
        cursor = conn.cursor()
        cursor.execute('INSERT INTO blockchain (block_height) VALUES (?)', (block_height,))
        conn.commit()
        conn.close()
    def main():
        while True:
            block_height = get_block_height()
            if block_height is not None:
                insert_block_height(block_height)
            time.sleep(60)  # Fetch every 60 seconds
    if __name__ == '__main__':
        main()
     ```

3. **Run the Ingestion Script**:
   - Start the ingestion process:
     ```
     python ingest.py
     ```

### Step 5: Setup Flask Web Application

1. **Create `app.py`**:
   - Add the following content:
     ```python
     from flask import Flask, render_template, jsonify
     import sqlite3

     app = Flask(__name__)
     DB_PATH = 'blockchain.db'

     def get_latest_block_height():
         conn = sqlite3.connect(DB_PATH)
         cursor = conn.cursor()
         cursor.execute('SELECT block_height FROM blockchain ORDER BY id DESC LIMIT 1')
         row = cursor.fetchone()
         conn.close()
         return row[0] if row else None

     @app.route('/')
     def index():
         return render_template('index.html')

     @app.route('/block_height')
     def block_height():
         block_height = get_latest_block_height()
         if block_height is not None:
             return jsonify({'block_height': block_height})
         else:
             return jsonify({'error': 'No data available'}), 500

     if __name__ == '__main__':
         app.run(debug=True)
     ```

2. **Create `templates/index.html`**:
   - Add the following content:
     ```html
     <!DOCTYPE html>
     <html lang="en">
     <head>
         <meta charset="UTF-8">
         <meta name="viewport" content="width=device-width, initial-scale=1.0">
         <title>Bitcoin Block Height</title>
         <script>
             async function fetchBlockHeight() {
                 try {
                     const response = await fetch('/block_height');
                     const data = await response.json();
                     if (data.block_height !== undefined) {
                         document.getElementById('block-height').innerText = data.block_height;
                     } else {
                         document.getElementById('block-height').innerText = 'Error fetching data';
                     }
                 } catch (error) {
                     console.error('Error fetching data:', error);
                     document.getElementById('block-height').innerText = 'Error fetching data';
                 }
             }
             setInterval(fetchBlockHeight, 60000); // Update every 60 seconds
             window.onload = fetchBlockHeight;
         </script>
     </head>
     <body>
         <h1>Latest Bitcoin Block Height</h1>
         <p id="block-height">Loading...</p>
     </body>
     </html>
     ```

3. **Run the Flask Application**:
   - Start the Flask web server:
     ```
     python app.py
     ```

4. **Access the Application**:
   - Open a web browser and navigate to `http://localhost:5000/` to view the latest block height.

By following these instructions, you will have a fully functioning Bitcoin block height explorer that fetches data using a Rust client, stores it in a SQLite database, and displays it in real-time on a Flask web interface.
