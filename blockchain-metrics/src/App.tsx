

// src/App.tsx

import React from 'react';
import './App.css';
import Metrics from './components/Metrics';

const App: React.FC = () => {
    return (
        <div className="App">
            <header className="App-header">
                <Metrics />
            </header>
        </div>
    );
};

export default App;
