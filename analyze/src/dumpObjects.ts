/*
Parse the filepath and output objects that match the spec. For example, the objectName might be Check_Point_1


Slope_Arrow_Right {
uniform_scale	= 1;
loc	= {1123.46,0,847.318};
level_inclusion_mask	= -1;
offset	= {0,0,0};
quat	= {1,{0,0,0}};
melt_height	= 0;
animated_mesh_texture_id	= 0;
};


Check_Point_1 {
uniform_scale	= 1;
loc	= {1032.96,0,847.318};
level_inclusion_mask	= -1;
offset	= {0,0,0};
quat	= {1,{0,0,0}};
melt_height	= 0;
};

Check_Point_1 {
uniform_scale	= 1;
loc	= {1200,0,847.318};
level_inclusion_mask	= -1;
offset	= {0,0,0};
quat	= {1,{0,0,0}};
melt_height	= 0;
};

*/

import { readFile } from 'fs/promises';
import { GameConfigParser } from './GameConfigParser/parser';
import { toString } from './GameConfigParser/stringify';

export async function dumpObjects(filepath: string, typeFilter: Array<string> | undefined, options?: { format?: string }) {
  const file = await readFile(filepath, 'utf8');
  let objects = new GameConfigParser(file).parse();
  if (typeFilter?.length) {
    objects = objects.filter((object) => typeFilter.includes(object.type));
  }
  if (options?.format === 'json') {
    console.log(JSON.stringify(objects, null, 2));
  } else {
    console.log(toString(objects))
  }
  return objects;
}
