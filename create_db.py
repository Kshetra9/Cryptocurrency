# create_db.py

import sqlite3

DB_PATH = 'blockchain.db'

def create_database():
    conn = sqlite3.connect(DB_PATH)
    cursor = conn.cursor()
    cursor.execute('''
    CREATE TABLE IF NOT EXISTS blockchain (
    id INTEGER PRIMARY KEY AUTOINCREMENT,
    block_height INTEGER,
    network_hash_rate REAL,
    difficulty REAL,
    mempool_size INTEGER,
    timestamp DATETIME DEFAULT CURRENT_TIMESTAMP
    );
    ''')
    conn.commit()
    conn.close()

if __name__ == '__main__':
    create_database()
