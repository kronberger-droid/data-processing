
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

# Ausgabeordner
ausgabe.mkdir(parents=True, exist_ok=True)


# Datei laden
def sudoku_laden(datei):
    with open(datei, "r", encoding="utf-8") as f:
        inhalt = f.read()

    zahlen = "".join(inhalt.split())

    if len(zahlen) != 81 or any(z not in "0123456789" for z in zahlen):
        raise ValueError("Ungültiges Sudoku-Format")

    return [
        [int(zahlen[r * 9 + c]) for c in range(9)]
        for r in range(9)
    ]


# widersprüchliche Vorgaben prüfen
def sudoku_gueltig(b):
    for r in range(9):
        for c in range(9):
            n = b[r][c]

            if n == 0:
                continue

            b[r][c] = 0

            gueltig = (
                n not in b[r]
                and all(b[i][c] != n for i in range(9))
                and all(
                    b[i][j] != n
                    for i in range(r//3*3, r//3*3+3)
                    for j in range(c//3*3, c//3*3+3)
                )
            )

            b[r][c] = n

            if not gueltig:
                return False

    return True


# Gelöstes Sudoku speichern
def sudoku_speichern(sudoku, datei):
    with open(datei, "w", encoding="utf-8") as f:
        for r, row in enumerate(sudoku):

            # Leerzeile nach jedem dritten Block
            if r > 0 and r % 3 == 0:
                f.write("\n")

            for c, zahl in enumerate(row):
                f.write(str(zahl))

                if c < 8:
                    if (c + 1) % 3 == 0:
                        f.write("  ")
                    else:
                        f.write(" ")

            f.write("\n")


# Sudoku-Dateien suchen
dateien = sorted(
    list(eingabe.glob("*.dat")) +
    list(eingabe.glob("*.sudo"))
)

geloest = 0
fehler = 0

# Dateien bearbeiten
for datei in dateien:
    try:
        sudoku = sudoku_laden(datei)


        if not sudoku_gueltig(sudoku):
            print(f"Ungültiges Sudoku: {datei.name}")
            fehler += 1
            continue

        if sudoku_solve(sudoku):


            ziel = ausgabe / f"{datei.stem}_geloest.dat"


            sudoku_speichern(sudoku, ziel)

            print(f"Gelöst: {datei.name}")
            geloest += 1

        else:
            print(f"Keine Lösung: {datei.name}")
            fehler += 1

    except (ValueError, OSError) as e:
        print(f"Fehler in {datei.name}: {e}")
        fehler += 1


# Zusammenfassung

print(f"Dateien insgesamt: {len(dateien)}")
print(f"Erfolgreich gelöst: {geloest}")
print(f"Fehler / ungelöst: {fehler}")

