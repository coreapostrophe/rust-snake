import React, {useEffect, useState} from 'react';
import './app.scss';
import init from "./snake-lib"
import SnakeGame from "./snake-game/snake-game";

function App() {
    const [libMounted, setLibMounted] = useState<boolean>(false);

    useEffect(() => {
        init().then(() => setLibMounted(true));
    }, []);

    return (
        <div className="App">
            {libMounted && <SnakeGame />}
        </div>
    );
}

export default App;
