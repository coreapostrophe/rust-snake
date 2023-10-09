// @ts-nocheck
import {
  MutableRefObject,
  ReactElement,
  useCallback,
  useMemo,
  useRef,
} from 'react';
import { Container, Graphics, Stage, Text } from '@pixi/react';
import styles from './snake-game.module.scss';
import { SnakeEngine, SnakeEngineBuilder } from '../snake-lib';

export const GAME_BACKGROUND = 0xffffff;

export function WorldGrid(props): ReactElement {
  const { cell_grid, cell_size } = props;

  console.log(cell_size);

  const graphic = useCallback(
    (g: PixiGraphics) => {
      g.clear();
      g.beginFill(0xf3f3f3, 1);
      g.lineStyle(2, 0x737373, 1);
      g.drawRect(50, 150, 150, 150);

      cell_grid.forEach((row) => {
        row.forEach(([x, y]) => {
          g.drawRect(x, y, cell_size, cell_size);
        });
      });
    },
    [cell_grid, cell_size],
  );

  return <Graphics draw={graphic} />;
}

export default function SnakeGame(): ReactElement {
  const snakeEngine: MutableRefObject<SnakeEngine> = useRef(
    SnakeEngineBuilder.new().set_window(500, 500).set_world(9, 9).build(),
  );

  const cell_grid = useMemo(() => snakeEngine.current.generate_cells(), []);
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
    </Stage>
  );
}
