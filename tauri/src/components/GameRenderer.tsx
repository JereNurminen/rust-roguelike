import { Stage, Container, Graphics } from '@pixi/react';
import { useCallback } from 'react';
import * as PIXI from 'pixi.js';

const TILE_SIZE = 32;

interface Entity {
  id: number;
  kind: string;
  pos: { x: number; y: number } | null;
}

interface GameRendererProps {
  entities: Entity[];
  selectedTile: { x: number; y: number } | null;
  onTileClick: (x: number, y: number) => void;
}

export const GameRenderer = ({ entities, selectedTile, onTileClick }: GameRendererProps) => {
  const drawGrid = useCallback((g: PIXI.Graphics) => {
    g.clear();
    g.lineStyle(1, 0x333333);
    
    // Draw vertical lines
    for (let x = 0; x < 800; x += TILE_SIZE) {
      g.moveTo(x, 0);
      g.lineTo(x, 600);
    }
    
    // Draw horizontal lines
    for (let y = 0; y < 600; y += TILE_SIZE) {
      g.moveTo(0, y);
      g.lineTo(800, y);
    }
  }, []);

  const drawEntities = useCallback((g: PIXI.Graphics) => {
    g.clear();
    entities.forEach(entity => {
      if (entity.pos) {
        const x = entity.pos.x * TILE_SIZE;
        const y = entity.pos.y * TILE_SIZE;
        
        switch (entity.kind) {
          case 'Player':
            g.beginFill(0x00FF00);
            break;
          case 'Npc':
            g.beginFill(0xFF0000);
            break;
          case 'Wall':
            g.beginFill(0x808080);
            break;
          default:
            g.beginFill(0xFFFFFF);
        }
        
        g.drawCircle(x + TILE_SIZE/2, y + TILE_SIZE/2, TILE_SIZE/3);
        g.endFill();
      }
    });
  }, [entities]);

  const handleClick = (event: PIXI.FederatedPointerEvent) => {
    const x = Math.floor(event.global.x / TILE_SIZE);
    const y = Math.floor(event.global.y / TILE_SIZE);
    onTileClick(x, y);
  };

  return (
    <Stage width={800} height={600} options={{ backgroundColor: 0x000000 }}>
      <Container interactive={true} onclick={handleClick}>
        <Graphics draw={drawGrid} />
        <Graphics draw={drawEntities} />
        {selectedTile && (
          <Graphics
            draw={useCallback((g: PIXI.Graphics) => {
              g.clear();
              g.lineStyle(2, 0xFFFF00);
              g.drawRect(
                selectedTile.x * TILE_SIZE,
                selectedTile.y * TILE_SIZE,
                TILE_SIZE,
                TILE_SIZE
              );
            }, [selectedTile])}
          />
        )}
      </Container>
    </Stage>
  );
};
