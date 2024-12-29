
import { Command } from 'commander';
import { existsSync } from 'fs';
import { analyzeReplay } from './analyzeReplay';

const program = new Command();

program
 .name('analyzer')
 .description('CLI tool for file analysis')
 .version('1.0.0');

program.command('analyze')
 .description('Analyze a file')
 .argument('<filepath>', 'path to file')
 .action(async (filepath: string) => {
   if (!existsSync(filepath)) {
     console.error('File not found:', filepath);
     process.exit(1);
   }

   console.log(await analyzeReplay(filepath));
   // Add analysis logic here
 });

program.parse();