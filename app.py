# app.py

from flask import Flask, jsonify , render_template
import sqlite3

app = Flask(__name__)
DB_PATH = 'blockchain.db'

@app.route('/api/blockchain_metrics', methods=['GET'])
def get_blockchain_metrics():
    conn = sqlite3.connect(DB_PATH)
    cursor = conn.cursor()
    cursor.execute('SELECT block_height, network_hash_rate, difficulty, mempool_size FROM blockchain ORDER BY id DESC LIMIT 1')
    row = cursor.fetchone()
    conn.close()

    keys = ['block_height', 'network_hash_rate', 'difficulty', 'mempool_size']
    values = row or [None] * 4

    return dict(zip(keys, values))

@app.route('/metrics')
def metrics():
    return jsonify(get_blockchain_metrics())

@app.route('/')
def index():
    return render_template('index.html')

if __name__ == '__main__':
    app.run(debug=True)