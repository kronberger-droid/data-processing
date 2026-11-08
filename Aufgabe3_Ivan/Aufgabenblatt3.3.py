
import threading
import multiprocessing
import time

from Aufgabenblatt3 import matrix_multiplikation, matrix_erzeugen


# Berechnung mit funktion aus aufgabe 1
def berechnung(aufgabe):
    A, B = aufgabe
    return matrix_multiplikation(A, B)


# Threading
def thread_berechnung(i, aufgabe, ergebnisse):
    ergebnisse[i] = berechnung(aufgabe)


if __name__ == "__main__":


    aufgaben = matrix_erzeugen(10, 200)
    anzahl_prozesse = min(10, multiprocessing.cpu_count())

    print("Verfügbare Prozessoren:", multiprocessing.cpu_count())

    # Aufgabe 3a: Threading
    start = time.perf_counter()

    threads = []
    ergebnisse_thread = [None] * len(aufgaben)

    for i, aufgabe in enumerate(aufgaben):
        t = threading.Thread(
            target=thread_berechnung,
            args=(i, aufgabe, ergebnisse_thread)
        )
        threads.append(t)
        t.start()

    for t in threads:
        t.join()

    end = time.perf_counter()
    zeit_thread = end - start

    print("Threading:", zeit_thread, "Sekunden")


    # Aufgabe 3b: Multiprocessing
    start = time.perf_counter()

    with multiprocessing.Pool(processes=anzahl_prozesse) as pool:
        ergebnisse_multi = pool.map(berechnung, aufgaben)

    end = time.perf_counter()
    zeit_multi = end - start

    print("Multiprocessing:", zeit_multi, "Sekunden")


    # vergleich
    print("Ergebnisse gleich:",
          ergebnisse_thread == ergebnisse_multi)

    print("Speedup Multiprocessing gegenüber Threading:",
          zeit_thread / zeit_multi)
