
# Aufgabe 1


def matrix_multiplikation(A, B):
    zeilen_A = len(A)
    spalten_A = len(A[0])
    spalten_B = len(B[0])

    if spalten_A != len(B):
        raise ValueError("dimension passt nicht")

    C = [[0 for _ in range(spalten_B)]
         for _ in range(zeilen_A)]

    for i in range(zeilen_A):
        for j in range(spalten_B):
            for k in range(spalten_A):
                C[i][j] += A[i][k] * B[k][j]

    return C


# Aufgabe 2
import random
import time


def matrix_erzeugen(anzahl=10, groesse=200):
    aufgaben = []

    for _ in range(anzahl):
        A = [[random.randint(0, 9) for _ in range(groesse)]
             for _ in range(groesse)]

        B = [[random.randint(0, 9) for _ in range(groesse)]
             for _ in range(groesse)]

        aufgaben.append((A, B))

    return aufgaben


# Aufgabe 3
import threading
import multiprocessing


# berechnung
def berechnung(aufgabe):
    A, B = aufgabe
    return matrix_multiplikation(A, B)


def thread_berechnung(i, aufgabe):
    ergebnisse_thread[i] = berechnung(aufgabe)


if __name__ == "__main__":

    A = [[1, 2],
         [3, 4]]

    B = [[5, 6],
         [7, 8]]

    print(matrix_multiplikation(A, B))

    aufgaben = matrix_erzeugen(10, 200)

    start = time.perf_counter()

    for A, B in aufgaben:
        C = matrix_multiplikation(A, B)

    end = time.perf_counter()

    print("Laufzeit:", end - start, "Sekunden")


    # Threading
    start = time.perf_counter()

    threads = []
    ergebnisse_thread = [None] * len(aufgaben)

    for i, aufgabe in enumerate(aufgaben):
        t = threading.Thread(target=thread_berechnung, args=(i, aufgabe))
        threads.append(t)
        t.start()

    for t in threads:
        t.join()

    end = time.perf_counter()
    print("Threading:", end - start, "Sekunden")


    # Multiprocessing
    start = time.perf_counter()

    with multiprocessing.Pool(processes=10) as pool:
        ergebnisse_multi = pool.map(berechnung, aufgaben)

    end = time.perf_counter()

    print("Multiprocessing:", end - start, "Sekunden")
    print("Ergebnisse gleich:", ergebnisse_thread == ergebnisse_multi)
