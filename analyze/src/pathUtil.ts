import { readdir } from "fs/promises";
import { join, relative } from "path";
import { stat } from "fs/promises";

export async function findDatFiles(dirPath: string): Promise<string[]> {
  const files = await readdir(dirPath);
  const results = await Promise.all(
    files.map(async (file) => {
      const fullPath = join(dirPath, file);
      const stats = await stat(fullPath);

      if (stats.isDirectory()) {
        return findDatFiles(fullPath);
      }

      return file.endsWith(".dat") ? [relative(process.cwd(), fullPath).replace(/\\/g, "/")] : [];
    })
  );

  return results.flat();
}
