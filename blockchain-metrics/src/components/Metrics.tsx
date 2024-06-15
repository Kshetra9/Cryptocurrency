// src/components/Metrics.tsx

import React, { useEffect, useState } from 'react';
import axios from 'axios';

interface BlockchainData {
    block_height: number;
    network_hash_rate: number;
    difficulty: number;
    mempool_size: number;
}

const Metrics: React.FC = () => {
    const [metrics, setMetrics] = useState<BlockchainData | null>(null);
    const [loading, setLoading] = useState<boolean>(true);
    const [error, setError] = useState<string | null>(null);

    useEffect(() => {
        const fetchMetrics = async () => {
            try {
                const response = await axios.get<BlockchainData>('http://127.0.0.1:8080/fetch_metrics');
                setMetrics(response.data);
            } catch (err) {
                setError('Failed to fetch metrics');
            } finally {
                setLoading(false);
            }
        };

        fetchMetrics();
    }, []);

    if (loading) {
        return <div>Loading...</div>;
    }

    if (error) {
        return <div>{error}</div>;
    }

    return (
        <div>
            <h1>Blockchain Metrics</h1>
            {metrics && (
                <ul>
                    <li>Block Height: {metrics.block_height}</li>
                    <li>Network Hash Rate: {metrics.network_hash_rate}</li>
                    <li>Difficulty: {metrics.difficulty}</li>
                    <li>Mempool Size: {metrics.mempool_size}</li>
                </ul>
            )}
        </div>
    );
};

export default Metrics;
