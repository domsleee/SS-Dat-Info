import { readdir } from "fs/promises";
import { join, relative } from 'path';
import { stat } from "fs/promises";
import { REPLAY_FOLDER } from "./types";
import { findDatFiles } from "./pathUtil";
import { analyzeReplay } from "./analyzeReplayFs";

export async function debugKeypress() {
    const basePath = join(REPLAY_FOLDER, "tests/keypress");
    const files = await findDatFiles(basePath);

    const relevant = [
        "NOPRESS.dat",
        "KLEFT.dat",
        "KRIGHT.dat",
        "KFORWARD.dat",
        "KFORLEFT.dat",
        "KFORRIGHT.dat",
        "KSHIFTLEFT.dat",
        "KSHIFTRIGHT.dat",
        "KSHIFTFORLEFT.dat",
        "KSHIFTFORRIGHT.dat",
    ];

    const getName = (file: string) => file.split('/').at(-1) ?? '';
    for (const file of files.sort((a,b) => relevant.indexOf(getName(a)) - relevant.indexOf(getName(b)))) {
        if (relevant.includes(getName(file))) {
            const track = file.split('/').at(-2);
            const name = getName(file);
            const coords = await analyzeReplay(file, true);
            console.log(`${name} (${track})`)
            console.log(coords.coords?.rows[302].raw)
            console.log(coords.coords?.rows[402].raw);
            console.log(coords.coords?.rows[502].raw);
            console.log()
        }
    }


}
