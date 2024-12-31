import { Command } from "commander";
import { existsSync, writeFile, writeFileSync } from "fs";
import { analyzeReplay, getDataBlocks } from "./analyzeReplay";
import { readFile } from "fs/promises";

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
  .action(async (filepath: string) => {
    if (!existsSync(filepath)) {
      console.error("File not found:", filepath);
      process.exit(1);
    }

    const content = await readFile(filepath);
    const hexData = Buffer.from(content.buffer).toString("hex");
    console.log(getDataBlocks(hexData).join("\n"));
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
    console.log("x,y,z,rx,rw,ry,rz");
    for (const c of d.coords!.rows) {
        console.log(`${c.x},${c.y},${c.z},${c.rx},${c.rw},${c.ry},${c.rz}`);
    }
  });
program.parse();
