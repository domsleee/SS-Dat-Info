import { GameConfigParser, circleToString, generateCircle } from "analyze/src/exports";

document.addEventListener("DOMContentLoaded", function (): void {
  if (
    localStorage.getItem("darkMode") !== "false"
  ) {
    document.documentElement.classList.add("dark");
  }

  const generateBtn = document.getElementById("generate") as HTMLButtonElement;
  const copyBtn = document.getElementById(
    "copyToClipboard"
  ) as HTMLButtonElement;
  const outputTextarea = document.getElementById(
    "outputResult"
  ) as HTMLTextAreaElement;

  document
    .getElementById("themeToggle")!
    .addEventListener("click", toggleDarkMode);

  generateBtn.addEventListener("click", handleGenerate);

  function handleGenerate() {
    const outputStatus = document.getElementById("outputStatus") as HTMLDivElement;
    const selectObjectNameElement = document.getElementById(
      "selectObjectName"
    ) as HTMLSelectElement;

    const selectedObjectName = selectObjectNameElement.value;

    const radius = (document.getElementById("radiusValue") as HTMLInputElement)
      .value;
    const numObjects = (
      document.getElementById("numObjectsValue") as HTMLInputElement
    ).value;
    const uniformScale = (
      document.getElementById("uniformScaleValue") as HTMLInputElement
    ).value;

    const inputData = (
      document.getElementById("inputObjects") as HTMLTextAreaElement
    ).value;

    try {
      const parser = new GameConfigParser(inputData ?? "");
      const objects = parser.parse();
  
      const outputItems: Array<string> = [];
      let totalObjects = 0;
      for (const object of objects) {
        const myObject = object.properties as unknown as ObjectProperties;
        const circle = generateCircle({
          name: selectedObjectName,
          radius: parseFloat(radius),
          centerLoc: {x: parseFloat(myObject.loc[0]), y: parseFloat(myObject.loc[1]), z: parseFloat(myObject.loc[2])},
          count: parseInt(numObjects),
          uniform_scale: parseFloat(uniformScale),
          quat: { w: parseFloat(myObject.quat[0]), x: parseFloat(myObject.quat[1][0]), y: parseFloat(myObject.quat[1][1]), z: parseFloat(myObject.quat[1][2])}
        });
        outputItems.push(circleToString(circle));
        totalObjects += circle.length;
      }
      outputTextarea.value = outputItems.join("\n\n");
      outputStatus.innerText = `Generated ${totalObjects} objects.`;
    } catch (e) {
      outputStatus.innerText = `Error: ${e}`;
      throw e;
    }
  }

  copyBtn.addEventListener("click", function (): void {
    outputTextarea.select();
    document.execCommand("copy");

    const originalText = this.textContent as string;
    this.textContent = "Copied!";
    this.classList.add("text-green-600", "dark:text-green-400");

    setTimeout(() => {
      this.textContent = originalText;
      this.classList.remove("text-green-600", "dark:text-green-400");
    }, 2000);
  });
});

function toggleDarkMode(): void {
  const html = document.documentElement;
  html.classList.toggle("dark");
  localStorage.setItem("darkMode", html.classList.contains("dark").toString());
}

interface ObjectProperties {
  uniform_scale: string;
  loc: [string, string, string];
  quat: [string, [string, string, string]];
}