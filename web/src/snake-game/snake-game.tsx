// @ts-nocheck
import {ReactElement, useState} from "react";
import {WorldBuilder} from "../snake-lib";
import {Container, Sprite, Stage, Text} from "@pixi/react";
import styles from './snake-game.module.scss';

export default function SnakeGame(): ReactElement {
    const [world] = useState(WorldBuilder.new(5, 5).build())

    return (
        <Stage width={500} height={500} className={styles.Stage}>
            <Sprite
                image="https://pixijs.io/pixi-react/img/bunny.png"
                x={400}
                y={270}
                anchor={{ x: 0.5, y: 0.5 }}
            />
            <Container x={400} y={330}>
                <Text text="Hello World" anchor={{ x: 0.5, y: 0.5 }} />
            </Container>
        </Stage>
    )
}