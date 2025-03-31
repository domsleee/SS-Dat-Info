import { GameObject, GameValue } from "./types";

export function toString(objects: Array<GameObject>): string {
  return objects.map(
    object => objectToString(object)
  ).join('\n\n');
}

function objectToString(object: GameObject): string {
  const maxKeyLength = Object.keys(object.properties).reduce((max, key) => Math.max(max, key.length), 0);
  const propertiesStr = Object.entries(object.properties)
    .map(([key, value]) => {
      return `  ${key.padEnd(maxKeyLength, ' ')} = ${valueToString(value)};`;
    })
    .join('\n');

  return `${object.type} {\n${propertiesStr}\n};`;
}

function valueToString(value: GameValue): string {
  if (Array.isArray(value)) {
    return `{${value.map(v => valueToString(v)).join(',')}}`;
  }
    
  if (typeof value === 'string') {
    return `${value}`;
  }
    
  return String(value);
}