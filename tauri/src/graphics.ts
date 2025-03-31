import { Entity, EntityKind } from "./bindings";

export const getEntityColor = (entity: Entity): number => {
  switch (entity.kind.type) {
    case "Player":
      return 0x00ff00;
    case "Npc":
      return 0xff0000;
    case "Wall":
      return 0xdddddd;
    default:
      return 0x000000;
  }
};

export const sortEntitiesByLayer = (entities: Entity[]) => {
  const layers: Record<EntityKind["type"], Entity[]> = {
    Floor: [],
    Wall: [],
    Item: [],
    Npc: [],
    Player: [],
  };

  entities.forEach((entity) => {
    layers[entity.kind.type].push(entity);
  });

  return [
    ...layers.Floor,
    ...layers.Wall,
    ...layers.Item,
    ...layers.Npc,
    ...layers.Player,
  ];
};
