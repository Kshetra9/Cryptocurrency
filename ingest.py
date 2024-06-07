 # ingest.py
#
import sqlite3
import subprocess
import time


DB_PATH = 'blockchain.db'
# BITCOIN_CLI_PATH = 'target/debug/rust_client'
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

