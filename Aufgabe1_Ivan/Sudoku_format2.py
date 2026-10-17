
from pathlib import Path


# Sudoku-Solver
def sudoku_solve(b):
    for r in range(9):
        for c in range(9):
            if b[r][c] == 0:
                for n in range(1, 10):
                    if (n not in b[r]
                        and all(b[i][c] != n for i in range(9))
                        and all(b[i][j] != n
                                for i in range(r//3*3, r//3*3+3)
                                for j in range(c//3*3, c//3*3+3))):
                        b[r][c] = n
                        if sudoku_solve(b):
                            return True
                        b[r][c] = 0
                return False
    return True


# Ordner
eingabe = Path("Aufgabe1_Ivan/daten/sudokus")
ausgabe = Path("Aufgabe1_Ivan/daten/geloste sudokus")
ausgabe.mkdir(parents=True, exist_ok=True)


# Alle .dat-Dateien bearbeiten
for datei in eingabe.glob("*.dat"):

    zeilen = datei.read_text().splitlines()

    try:
        sudoku = [
            [int(n) for n in zeile.split()]
            for zeile in zeilen if zeile.strip()
        ]
    except ValueError:
        print(f"ubersprungen: {datei.name}")
        continue


    if len(sudoku) != 9 or any(len(row) != 9 for row in sudoku):
        print(f"ubersprungen: {datei.name}")
        continue


    if sudoku_solve(sudoku):

        ziel = ausgabe / datei.name

        with open(ziel, "w") as f:
            for r, row in enumerate(sudoku):
                if r > 0 and r % 3 == 0:
                    f.write("\n")

                f.write("  ".join(
                    " ".join(map(str, row[i:i+3]))
                    for i in range(0, 9, 3)
                ) + "\n")

        print(f"gelost: {datei.name}")

    else:
        print(f"keine losung: {datei.name}")
