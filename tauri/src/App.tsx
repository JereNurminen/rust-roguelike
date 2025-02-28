import { useState, useEffect } from "react";
import { invoke } from "@tauri-apps/api/core";
import { GameRenderer } from "./components/GameRenderer";
import "./App.css";
import api from "./api";
import { Entity } from "./bindings";

function App() {
  const [entities, setEntities] = useState<Entity[]>([]);
  const [selectedTile, setSelectedTile] = useState<{
    x: number;
    y: number;
  } | null>(null);

  useEffect(() => {
    // Initial game state
    updateGameState();
  }, []);

  const updateGameState = async () => {
    const state = await api.getGameState();
    if (state.status === "ok") {
      setEntities(Object.values(state.data.world.entities).filter((x) => !!x));
    }
  };

  const handleTileClick = async (x: number, y: number) => {
    setSelectedTile({ x, y });
    const entitiesAtPos = await invoke<Entity[]>("get_entities_at_position", {
      x,
      y,
    });
    if (entitiesAtPos.length > 0) {
      console.log("Selected entities:", entitiesAtPos);
    }
  };

  const handleKeyPress = async (event: KeyboardEvent) => {
    let direction = null;
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
      await invoke("move_player", { direction });
      await updateGameState();
    }
  };

  useEffect(() => {
    window.addEventListener("keydown", handleKeyPress);
    return () => window.removeEventListener("keydown", handleKeyPress);
  }, []);

  return (
    <main className="container">
      <GameRenderer
        entities={entities}
        selectedTile={selectedTile}
        onTileClick={handleTileClick}
      />
    </main>
  );
}

export default App;
