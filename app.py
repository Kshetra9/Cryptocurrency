# app.py

from flask import Flask, jsonify , render_template
import sqlite3

app = Flask(__name__)
DB_PATH = 'blockchain.db'

def get_latest_block_height():
    conn = sqlite3.connect(DB_PATH)
    cursor = conn.cursor()
    cursor.execute('SELECT block_height, timestamp FROM blockchain ORDER BY id DESC LIMIT 1')
    result = cursor.fetchone()
    conn.close()
    return result if result else (None, None)

# Function to update the block height in the SQLite database
# def update_db_block_height(height):
#     conn = sqlite3.connect('block_height.db')
#     cursor = conn.cursor()
#     cursor.execute('INSERT INTO block_height (height) VALUES (?)', (height,))
#     conn.commit()
#     conn.close()


@app.route('/')
# def index():
#     block_height, timestamp = get_latest_block_height()
#     return jsonify(block_height=block_height, timestamp=timestamp)

def index():
    # Get the latest block height from Bitcoin Core
    block_height,timestamp  = get_latest_block_height()
    # if block_height is not None:
    #     # Update the block height in the SQLite database
    #     update_db_block_height(block_height)
    # else:
        # If unable to get the latest block height from Bitcoin Core, retrieve it from the database
    return render_template('index.html',  block_height=block_height, timestamp=timestamp)

@app.route('/block_height')
def block_height():
    block_height, timestamp = get_latest_block_height()
    return jsonify({'blockHeight': block_height, 'timestamp':timestamp})

if __name__ == '__main__':
    app.run(debug=True, host='0.0.0.0')
