import React, { ReactElement } from 'react';
import { Box, Flex, IconButton, Kbd, Text } from '@chakra-ui/react';
import { ArrowDown, ArrowLeft, ArrowRight, ArrowUp } from 'react-feather';
import { useKeyboard } from './keyboard-provider/keyboard-provider';

export default function KeyboardInterface(): ReactElement {
  const { isDown, isLeft, isRight, isUp } = useKeyboard();

  return (
    <Flex mt={4} flexDirection="column" gap={8}>
      <Text opacity={0.6} fontSize="sm">
        <strong>Up: </strong>
        <Kbd>W</Kbd>
        <span> or </span>
        <Kbd>Arrow Up</Kbd>
        <span> | </span>
        <strong>Left: </strong>
        <Kbd>A</Kbd>
        <span> or </span>
        <Kbd>Arrow Left</Kbd>
        <span> | </span>
        <strong>Down: </strong>
        <Kbd>S</Kbd>
        <span> or </span>
        <Kbd>Arrow Down</Kbd>
        <span> | </span>
        <strong>Right: </strong>
        <Kbd>D</Kbd>
        <span> or </span>
        <Kbd>Arrow Right</Kbd>
        <span> | </span>
        <strong>Reset: </strong>
        <Kbd>space</Kbd>
      </Text>
      <Flex flexDirection="column" gap={1} justifyContent="center">
        <Box>
          <IconButton
            pointerEvents="none"
            aria-label="up button"
            icon={<ArrowUp />}
            variant={isUp ? 'solid' : 'outline'}
          />
        </Box>
        <Flex alignItems="center" justifyContent="center" gap={1}>
          <IconButton
            pointerEvents="none"
            aria-label="up button"
            icon={<ArrowLeft />}
            variant={isLeft ? 'solid' : 'outline'}
          />
          <IconButton
            pointerEvents="none"
            aria-label="up button"
            icon={<ArrowDown />}
            variant={isDown ? 'solid' : 'outline'}
          />
          <IconButton
            pointerEvents="none"
            aria-label="up button"
            icon={<ArrowRight />}
            variant={isRight ? 'solid' : 'outline'}
          />
        </Flex>
      </Flex>
    </Flex>
  );
}
