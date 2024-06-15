import init, {
  getGameState,
  openCell,
  toggleFlag,
} from "../pkg/minesweeper.js";

const render = () => {
  let gameData = getGameState()
    .split("\n")
    .map((row) => row.trim().split(" "));

  const app = document.getElementById("app");
  app.innerHTML = ""; // Clear the app
  app.className = "app";
  app.style.display = "inline-grid";
  app.style.gridTemplate = `repeat(${gameData.length}, 1fr) / repeat(${gameData[0].length}, 1fr)`;
  app.style.columnGap = "0.7em";
  app.style.rowGap = "0.7em";

  gameData.forEach((row, columnIndex) => {
    row.forEach((cell, rowIndex) => {
      const cellEl = document.createElement("a");
      cellEl.href = "#";
      cellEl.className = "cell";
      cellEl.textContent = cell === "0" ? "" : cell;

      cellEl.addEventListener("click", (e) => {
        e.preventDefault();
        openCell(rowIndex, columnIndex);
        // Re-render the game
        render();
      });

      cellEl.addEventListener("contextmenu", (e) => {
        e.preventDefault();
        toggleFlag(rowIndex, columnIndex);
        // Re-render the game
        render();
      });

      app.appendChild(cellEl);
    });
  });
};

async function run() {
  await init();

  render();
}

run();
