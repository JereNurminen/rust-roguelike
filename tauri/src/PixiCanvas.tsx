import React, { useRef, useEffect } from "react";
import * as PIXI from "pixi.js";
import { Entity } from "./bindings";
import { Viewport } from "pixi-viewport";
import { getEntityColor, sortEntitiesByLayer } from "./graphics";
import { useWindowSize } from "./hooks/useWindowSize";

export const PixiCanvas = ({
  entities,
  rememberedEntities,
}: {
  entities: Entity[];
  rememberedEntities: Entity[];
}) => {
  const containerRef = useRef<HTMLDivElement>(null);
  const appRef = useRef<PIXI.Application | null>(null);
  const viewportRef = useRef<Viewport | null>(null);
  const entityRef = useRef<Entity[]>([]);
  entityRef.current = entities;

  const [windowWidth, windowHeight] = useWindowSize();

  const TILE_SIZE = 16;

  // Helper function to find entity at given coordinates
  const findEntityAtPosition = (worldX: number, worldY: number) => {
    const tileX = Math.floor(worldX / TILE_SIZE);
    const tileY = Math.floor(worldY / TILE_SIZE);

    console.debug("Looking for an entity at", tileX, tileY, entityRef.current);

    const entity = entityRef.current.find(
      (e) => e.pos && e.pos.x === tileX && e.pos.y === tileY,
    );

    return entity;
  };

  useEffect(() => {
    const f = async () => {
      if (!containerRef.current || appRef.current) return;

      const app = new PIXI.Application();
      await app.init({
        width: 800,
        height: 600,
        backgroundColor: 0x000000,
        resizeTo: containerRef.current,
      });

      const viewport = new Viewport({
        screenWidth: window.innerWidth,
        screenHeight: window.innerHeight,
        worldWidth: 1000,
        worldHeight: 1000,
        events: app.renderer.events,
      });

      app.stage.addChild(viewport);
      viewport.drag().pinch().wheel().decelerate().moveCenter(0, 0);

      viewport.interactive = true;
      app.stage.interactive = true;

      // Add click handlers to both viewport and stage
      viewport.addEventListener("click", (event) => {
        const worldPos = viewport.toWorld(event.global);
        console.log("Viewport  clicked in world position ", worldPos);
        const entity = findEntityAtPosition(worldPos.x, worldPos.y);
        if (entity) {
          console.log("Clicked entity:", entity);
        }
      });

      containerRef.current?.appendChild(app.canvas);
      appRef.current = app;
      viewportRef.current = viewport;
    };
    void f();

    return () => {
      appRef.current?.destroy(true, { children: true });
    };
  }, []);

  useEffect(() => {
    if (!appRef.current || !viewportRef.current) return;

    console.debug(entities);

    viewportRef.current.removeChildren();

    // Draw entities
    sortEntitiesByLayer(entities).forEach((e) => {
      if (e.visible && e.pos) {
        const graphics = new PIXI.Graphics();
        const color = getEntityColor(e);

        const screenX = e.pos.x * TILE_SIZE;
        const screenY = e.pos.y * TILE_SIZE;

        graphics
          .fill(color)
          .rect(screenX, screenY, TILE_SIZE, TILE_SIZE)
          .endFill();

        // Make each entity graphic interactive
        graphics.interactive = true;
        graphics.addEventListener("click", () => {
          console.log("Clicked directly on entity:", e);
        });

        viewportRef.current!.addChild(graphics);
      }
    });

    // Draw entities
    sortEntitiesByLayer(rememberedEntities).forEach((e) => {
      if (e.visible && e.pos && e.last_seen_at) {
        const graphics = new PIXI.Graphics();
        const color = getEntityColor(e);

        const screenX = e.last_seen_at.x * TILE_SIZE;
        const screenY = e.last_seen_at.y * TILE_SIZE;

        graphics
          .fill({ color, alpha: 0.5 })
          .rect(screenX, screenY, TILE_SIZE, TILE_SIZE)
          .endFill();

        viewportRef.current!.addChild(graphics);
      }
    });
  }, [entities]);

  useEffect(() => {
    //appRef.current?.resize();
    //viewportRef.current?.resize();
  }, [viewportRef, windowWidth, windowHeight]);

  return <div id="pixi-canvas" ref={containerRef} />;
};

export default PixiCanvas;
