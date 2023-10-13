import {
  PropsWithChildren,
  ReactElement,
  useCallback,
  useContext,
  useEffect,
  useState,
} from 'react';
import { createContext } from 'react';

const KeyboardContext = createContext<{
  isUp: boolean;
  isLeft: boolean;
  isDown: boolean;
  isRight: boolean;
}>({
  isUp: false,
  isLeft: false,
  isDown: false,
  isRight: false,
});

export const useKeyboard = () => useContext(KeyboardContext);

export default function KeyboardProvider(
  props: PropsWithChildren,
): ReactElement {
  const [isUp, setIsUp] = useState<boolean>(false);
  const [isLeft, setIsLeft] = useState<boolean>(false);
  const [isDown, setIsDown] = useState<boolean>(false);
  const [isRight, setIsRight] = useState<boolean>(false);

  const handleKeyDown = useCallback<EventListener>((event) => {
    switch ((event as KeyboardEvent).code) {
      case 'KeyA':
        setIsLeft(true);
        break;
      case 'ArrowLeft':
        setIsLeft(true);
        break;
      case 'KeyW':
        setIsUp(true);
        break;
      case 'ArrowUp':
        setIsUp(true);
        break;
      case 'KeyD':
        setIsRight(true);
        break;
      case 'ArrowRight':
        setIsRight(true);
        break;
      case 'KeyS':
        setIsDown(true);
        break;
      case 'ArrowDown':
        setIsDown(true);
        break;
    }
  }, []);
  const handleKeyUp = useCallback<EventListener>((event) => {
    switch ((event as KeyboardEvent).code) {
      case 'KeyA':
        setIsLeft(false);
        break;
      case 'ArrowLeft':
        setIsLeft(false);
        break;
      case 'KeyW':
        setIsUp(false);
        break;
      case 'ArrowUp':
        setIsUp(false);
        break;
      case 'KeyD':
        setIsRight(false);
        break;
      case 'ArrowRight':
        setIsRight(false);
        break;
      case 'KeyS':
        setIsDown(false);
        break;
      case 'ArrowDown':
        setIsDown(false);
        break;
    }
  }, []);

  useEffect(() => {
    document.addEventListener('keydown', handleKeyDown);
    document.addEventListener('keyup', handleKeyUp);

    return function cleanup() {
      document.removeEventListener('keydown', handleKeyDown);
      document.removeEventListener('keyup', handleKeyUp);
    };
  }, [handleKeyDown, handleKeyUp]);

  return (
    <KeyboardContext.Provider
      value={{
        isUp,
        isLeft,
        isDown,
        isRight,
      }}
    >
      <div>{props.children}</div>
    </KeyboardContext.Provider>
  );
}
