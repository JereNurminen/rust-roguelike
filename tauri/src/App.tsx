import { useState, useEffect } from "react";
import * as pixi from "pixi.js";
import { invoke } from "@tauri-apps/api/core";
import "./App.less";
import api from "./api";
import { Direction, Entity, Result } from "./bindings";
import PixiCanvas from "./PixiCanvas";

function App() {
  const [entities, setEntities] = useState<Entity[]>([]);
  const [rememberedEntities, setRememberedEntities] = useState<Entity[]>([]);

  useEffect(() => {
    const f = async () => {
      const initialWorld = await api.getGameState();
      if (initialWorld.status == "ok") {
        const entities = Object.values(initialWorld.data.world.entities).filter(
          (x) => !!x,
        );
        updateGameState(entities, []);
      }
    };
    void f();
  }, []);

  const updateGameState = async (
    entities: Entity[],
    rememberedEntities: Entity[],
  ) => {
    setEntities(entities);
    setRememberedEntities(rememberedEntities);
  };

  const handleKeyPress = async (event: KeyboardEvent) => {
    let direction: Direction | null = null;
    switch (event.key) {
      case "h":
        direction = "West";
        break;
      case "j":
        direction = "South";
        break;
      case "k":
        direction = "North";
        break;
      case "l":
        direction = "East";
        break;
    }

    if (direction) {
      //await invoke("move_player", { direction });
      const result = await api.movePlayer(direction);
      if (result.status === "ok") {
        await updateGameState(
          Object.values(result.data.world.entities).filter((x) => !!x),
          Object.values(result.data.remembered_entities).filter((x) => !!x),
        );
      }
    }
  };

  useEffect(() => {
    window.addEventListener("keydown", handleKeyPress);
    return () => window.removeEventListener("keydown", handleKeyPress);
  }, []);

  return (
    <main id="container">
      <div id="top-bar"></div>
      <div id="game-container">
        <PixiCanvas
          entities={entities}
          rememberedEntities={rememberedEntities}
        />
      </div>
      <div id="side-panel"></div>
      <div id="bottom-bar"></div>
    </main>
  );
}

export default App;
