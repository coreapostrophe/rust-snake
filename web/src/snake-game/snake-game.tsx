// @ts-nocheck
import {
  MutableRefObject,
  ReactElement,
  useCallback,
  useEffect,
  useMemo,
  useRef,
  useState,
} from 'react';
import { Graphics, Stage } from '@pixi/react';
import styles from './snake-game.module.scss';
import { SnakeEngine, SnakeEngineBuilder, World } from '../snake-lib';

export const GAME_BACKGROUND = 0xffffff;

export function WorldGrid(props): ReactElement {
  const { cellGrid, cellSize } = props;

  const graphic = useCallback(
    (g: PixiGraphics) => {
      cellGrid.forEach((row, rowIndex) => {
        const offsetRow = rowIndex % 2 === 0;
        row.forEach(([x, y], cellIndex) => {
          const colorTuple = [0xf2d9d7, 0xdbc0b8];
          if (cellIndex % 2 === 0) {
            g.beginFill(colorTuple[+offsetRow], 1);
          } else {
            g.beginFill(colorTuple[+!offsetRow], 1);
          }
          g.drawRect(x, y, cellSize, cellSize);
        });
      });
    },
    [cellGrid, cellSize],
  );

  return <Graphics draw={graphic} />;
}

export function Snake(props): ReactElement {
  const { snakeBody, cellSize } = props;

  const graphic = useCallback(
    (g: PixiGraphics) => {
      if (!snakeBody) return;

      g.beginFill(0x2f7362, 1);
      snakeBody.forEach(([x, y]) => {
        g.drawRect(x, y, cellSize, cellSize);
      });
    },
    [cellSize, snakeBody],
  );

  return <Graphics draw={graphic} />;
}

export default function SnakeGame(): ReactElement {
  const [snakeBody, setSnakeBody] = useState();

  const snakeEngine: MutableRefObject<SnakeEngine> = useRef(
    SnakeEngineBuilder.new().set_window(500, 500).set_world(25, 25).build(),
  );
  const cellGrid = useMemo(() => snakeEngine.current.generate_cells(), []);

  const window: Window = useMemo(() => snakeEngine.current.window(), []);
  const world: World = useMemo(() => snakeEngine.current.world(), []);
  const cellSize = useMemo(() => world.cell_size, [world.cell_size]);
  const generatedSnake = useMemo(
    () => snakeEngine.current.generate_snake(),
    [],
  );

  const { width: windowWidth, height: windowHeight } = window;

  useEffect(() => {
    setSnakeBody(generatedSnake);
  }, [generatedSnake]);

  return (
    <Stage
      width={windowWidth}
      height={windowHeight}
      className={styles.Stage}
      options={{ backgroundColor: GAME_BACKGROUND }}
    >
      <WorldGrid cellGrid={cellGrid} cellSize={cellSize} />
      <Snake snakeBody={snakeBody} cellSize={cellSize} />
    </Stage>
  );
}
