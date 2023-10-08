import {ReactElement, useState} from "react";
import {WorldBuilder} from "../snake-lib";

export default function SnakeGame(): ReactElement {
    const [world] = useState(WorldBuilder.new(5, 5).build())
    
    return (
        <div>
            Game works!
            <div>width: {world.width()}</div>
            <div>height: {world.height()}</div>
        </div>
    )
}