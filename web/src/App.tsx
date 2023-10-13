import React, { useEffect, useState } from 'react';
import './app.scss';
import init from './snake-lib/snakelib';
import SnakeGame from './snake-game/snake-game';
import { Box, Flex, Heading, Text } from '@chakra-ui/react';
import KeyboardProvider from './keyboard-interface/keyboard-provider/keyboard-provider';

function App() {
  const [libMounted, setLibMounted] = useState<boolean>(false);

  useEffect(() => {
    init().then(() => setLibMounted(true));
  }, []);

  return (
    <KeyboardProvider>
      <div className="App">
        <Box p={4}>
          <Heading mb={2}>🐍 rust snake</Heading>
          <Text opacity={0.4}>This is a simple wasm + pixi-js project</Text>
          <Flex
            justifyContent="center"
            py={8}
            alignItems="center"
            flexDirection="column"
          >
            {libMounted && <SnakeGame />}
          </Flex>
        </Box>
      </div>
    </KeyboardProvider>
  );
}

export default App;
