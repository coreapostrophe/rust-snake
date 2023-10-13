// @ts-nocheck
import {
  MutableRefObject,
  ReactElement,
  useCallback,
  useMemo,
  useRef,
} from 'react';
import { Graphics, Stage } from '@pixi/react';
import styles from './snake-game.module.scss';
import { SnakeEngine, SnakeEngineBuilder } from '../snake-lib';

export const GAME_BACKGROUND = 0xffffff;

export function WorldGrid(props): ReactElement {
  const { cell_grid, cell_size } = props;

  const graphic = useCallback(
    (g: PixiGraphics) => {
      cell_grid.forEach((row, rowIndex) => {
        const offsetRow = rowIndex % 2 === 0;
        row.forEach(([x, y], cellIndex) => {
          const colorTuple = [0xf2d9d7, 0xdbc0b8];
          if (cellIndex % 2 === 0) {
            g.beginFill(colorTuple[+offsetRow], 1);
          } else {
            g.beginFill(colorTuple[+!offsetRow], 1);
          }
          g.drawRect(x, y, cell_size, cell_size);
        });
      });
    },
    [cell_grid, cell_size],
  );

  return <Graphics draw={graphic} />;
}

export function Snake(props): ReactElement {
  const { snake_body, cell_size } = props;

  const graphic = useCallback(
    (g: PixiGraphics) => {
      g.beginFill(0x2f7362, 1);
      snake_body.forEach(([x, y]) => {
        g.drawRect(x, y, cell_size, cell_size);
      });
    },
    [cell_size, snake_body],
  );

  return <Graphics draw={graphic} />;
}

export default function SnakeGame(): ReactElement {
  const snakeEngine: MutableRefObject<SnakeEngine> = useRef(
    SnakeEngineBuilder.new().set_window(500, 500).set_world(25, 25).build(),
  );

  const cell_grid = useMemo(() => snakeEngine.current.generate_cells(), []);
  const snake_body = useMemo(() => snakeEngine.current.generate_snake(), []);
  const cell_size = useMemo(
    () => snakeEngine.current.world().get('cell_size'),
    [],
  );

  const { windowWidth, windowHeight } = useMemo(
    () => ({
      windowWidth: snakeEngine.current.window().get('width'),
      windowHeight: snakeEngine.current.window().get('height'),
    }),
    [],
  );

  return (
    <Stage
      width={windowWidth}
      height={windowHeight}
      className={styles.Stage}
      options={{ backgroundColor: GAME_BACKGROUND }}
    >
      <WorldGrid cell_grid={cell_grid} cell_size={cell_size} />
      <Snake snake_body={snake_body} cell_size={cell_size} />
    </Stage>
  );
}
