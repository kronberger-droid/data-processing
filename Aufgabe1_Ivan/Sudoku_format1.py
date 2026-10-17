
from pathlib import Path

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


eingabe = Path("Aufgabe1_Ivan/daten/sudokus")
ausgabe = Path("Aufgabe1_Ivan/daten/geloste sudokus")
ausgabe.mkdir(parents=True, exist_ok=True)

for datei in eingabe.glob("*.sudo"):

    zahlen = datei.read_text().strip()

    sudoku = [
        [int(zahlen[r*9+c]) for c in range(9)]
        for r in range(9)
    ]

    if sudoku_solve(sudoku):
        ziel = ausgabe / datei.name
        ziel.write_text("".join(str(n) for row in sudoku for n in row))
        print(f"gelost: {datei.name}")
    else:
        print(f"keine losung: {datei.name}")
