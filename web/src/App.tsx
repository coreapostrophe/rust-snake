import React, {useEffect, useState} from 'react';
import './app.scss';
import init, {greet} from "./snake-lib"

function App() {
    const [libMounted, setLibMounted] = useState<boolean>(false);

    useEffect(() => {
        init().then(() => setLibMounted(true));
    }, []);

    return (
        <div className="App">
            {libMounted && greet("World")}
        </div>
    );
}

export default App;
