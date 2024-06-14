 # ingest.py
#
import sqlite3
import subprocess
import time


DB_PATH = 'blockchain.db'
RUST_CLIENT_PATH = 'rust_client/target/release/rust_client.exe'
FETCH_INTERVAL = 60


def get_metrics():
    try:
        result = subprocess.run([RUST_CLIENT_PATH], stdout=subprocess.PIPE, stderr=subprocess.PIPE, check=True)
    except subprocess.CalledProcessError as e:
        print(f"Error: {e.stderr.decode('utf-8')}")
        return None

    output = result.stdout.decode('utf-8').strip().split('\n')
    metrics = {}
    print(output)
    for line in output:
        key, value = line.split(": ")
        metrics[key.strip()] = value.strip()
    return metrics

def insert_metrics(metrics):
    try:
      with sqlite3.connect(DB_PATH) as conn:
        cursor = conn.cursor()
        print(metrics)
        cursor.execute('INSERT INTO blockchain (block_height, network_hash_rate, difficulty, mempool_size) VALUES (?, ?, ?, ?)',
                      (metrics['block_height'], metrics['network_hash_rate'], metrics['difficulty'], metrics['mempool_size']))
        conn.commit()
        conn.close()
    except sqlite3.Error as e:
        print(f"Database error: {e}")

def main():
    while True:
        metrics = get_metrics()
        if metrics:
            insert_metrics(metrics)
        time.sleep(FETCH_INTERVAL)

if __name__ == '__main__':
    main()