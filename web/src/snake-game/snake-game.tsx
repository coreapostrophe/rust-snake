// @ts-nocheck
import { ReactElement, useCallback, useRef } from 'react';
import { Container, Graphics, Stage, Text } from '@pixi/react';
import styles from './snake-game.module.scss';
import { SnakeEngine, SnakeEngineBuilder } from '../snake-lib';

export const GAME_BACKGROUND = 0xffffff;

export function WorldGrid(): ReactElement {
  const graphic = useCallback((g: PixiGraphics) => {
    g.clear();
    g.beginFill(0xff3300);
    g.lineStyle(4, 0xffd900, 1);
    g.moveTo(50, 50);
    g.lineTo(250, 50);
    g.lineTo(100, 100);
    g.lineTo(50, 50);
    g.endFill();
    g.lineStyle(2, 0x0000ff, 1);
    g.beginFill(0xff700b, 1);
    g.drawRect(50, 150, 120, 120);
    g.lineStyle(2, 0xff00ff, 1);
    g.beginFill(0xff00bb, 0.25);
    g.drawRoundedRect(150, 100, 300, 100, 15);
    g.endFill();
    g.lineStyle(0);
    g.beginFill(0xffff0b, 0.5);
    g.drawCircle(470, 90, 60);
    g.endFill();
  }, []);

  return <Graphics draw={graphic} />;
}

export default function SnakeGame(): ReactElement {
  const snakeEngine: SnakeEngine = useRef(
    SnakeEngineBuilder.new().set_window(500, 500).set_world(9, 9).build(),
  );

  console.log(snakeEngine.current.window());
  console.log(snakeEngine.current.world());

  return (
    <Stage
      width={500}
      height={500}
      className={styles.Stage}
      options={{ backgroundColor: GAME_BACKGROUND }}
    >
      <WorldGrid />
      <Container x={250} y={250}>
        <Text text="Hello World" anchor={{ x: 0.5, y: 0.5 }} />
      </Container>
    </Stage>
  );
}
