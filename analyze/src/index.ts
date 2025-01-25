import { Command } from "commander";
import { existsSync, writeFile, writeFileSync } from "fs";
import { analyzeReplay } from "./analyzeReplayFs";
import { readFile } from "fs/promises";
import { getDataBlocks } from "./analyzeReplay";
import { debugKeypress } from "./debugKeypress";
import { dumpObjects } from "./dumpObjects";

const program = new Command();

program.name("analyzer").description("CLI tool for file analysis").version("1.0.0");

program
  .command("analyze")
  .description("Analyze a file")
  .argument("<filepath>", "path to file")
  .action(async (filepath: string) => {
    if (!existsSync(filepath)) {
      console.error("File not found:", filepath);
      process.exit(1);
    }

    console.log(await analyzeReplay(filepath));
    writeFileSync("output.txt", JSON.stringify(await analyzeReplay(filepath)), null, 2);
    // Add analysis logic here
  });

program
  .command("dump")
  .description("Output file hex")
  .argument("<filepath>", "path to file")
  .option("-b, --block <index>", "output specific block by index")
  .action(async (filepath: string, options: { block: string }) => {
    if (!existsSync(filepath)) {
      console.error("File not found:", filepath);
      process.exit(1);
    }

    const content = await readFile(filepath);
    const hexData = Buffer.from(content.buffer).toString("hex");
    const blocks = getDataBlocks(hexData);

    if (options.block) {
      console.log(blocks[parseInt(options.block)]);
    } else {
      console.log(blocks.join("\n")); 
    }
    // Add analysis logic here
  });

program
  .command("coords")
  .description("Output file hex")
  .argument("<filepath>", "path to file")
  .action(async (filepath: string) => {
    if (!existsSync(filepath)) {
      console.error("File not found:", filepath);
      process.exit(1);
    }

    const d = await analyzeReplay(filepath, true);
    console.log("x,y,z,rx,rw,ry,rz,left");
    for (const c of d.coords!.rows) {
        console.log(`${c.x},${c.y},${c.z},${c.rx.toFixed(5)},${c.rw.toFixed(5)},${c.ry.toFixed(5)},${c.rz.toFixed(5)},${c.left ? '1' : '0'},${c.right ? '1' : '0'}`);
    }
  });

program
  .command("debug-keypress")
  .description("Output data for keypress ideas")
  .action(async () => {
    return await debugKeypress();
  })

program
  .command("dump-objects")
  .argument("<filepath>", "path to Object_Data.txt file")
  .argument("[objectNames...]", "Names of object to dump (optional)")
  .description("Dump the data from Object_Data.txt in json")
  .action(async (filepath: string, objectNames: Array<string>) => {
    await dumpObjects(filepath, objectNames);
  })

program.parse();
