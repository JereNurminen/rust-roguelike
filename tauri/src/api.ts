import { commands, Direction } from "./bindings";

export default {
  getGameState: commands.getGameState,
  movePlayer: (direction: Direction) => commands.movePlayer(direction),
};
