import time

def cpu_werte():
    daten = {}

    with open("/proc/stat", "r") as datei:
        for zeile in datei:
            if not zeile.startswith("cpu"):
                continue

            teile = zeile.split()
            daten[teile[0]] = [int(x) for x in teile[1:]]

    return daten


vorher = cpu_werte()

while True:
    time.sleep(5)

    aktuel = cpu_werte()

    for cpu in aktuel:
        differenzen = [
            neu - alt
            for neu, alt in zip(aktuell[cpu], vorher[cpu])
        ]
        gesamt = sum(differenzen)

        if gesamt == 0:
            continue

        prozent = [
            wert / gesamt * 100
            for wert in differenzen
        ]

        print(cpu, [round(x, 2) for x in prozent])

    print()
    vorher = aktuel