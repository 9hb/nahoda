# Nahoda - Kryptograficky Generator Nahodnych Cisel

[English version](./en-readme.md)

## O projektu

Tento program implementuje vysoce nepredvidatelny generator nahodnych cisel s nazvem "Nahoda". Generator vyuziva mnoho zdroju entropie a slozite kryptograficke techniky pro zajisteni vysoke kvality nahodnych cisel.

## Pouziti

Program vyzaduje parametry `min` a `max` a volitelne parametr `pocet`:

```
nahoda.exe <min> <max> [pocet]
```

kde:

- `<min>` - minimalni hodnota generovanych cisel (povinne)
- `<max>` - maximalni hodnota generovanych cisel (povinne)
- `[pocet]` - pocet generovanych cisel (nepovinne, vychozi: 1, maximum: 100)

### Priklady pouziti:

```
nahoda.exe 1 100
```

Vygeneruje jedno nahodne cislo v rozsahu 1-100.

```
nahoda.exe 1 6 10
```

Vygeneruje 10 nahodnych cisel v rozsahu 1-6 (simulace hodu kostkou).

## Technicke detaily

### Zdroje entropie

Generator sbira entropii z mnoha zdroju, aby zajistil maximalni nepredvidatelnost:

1. **Systemovy cas** - s nanosekundovou presnosti
2. **Adresy v pameti** - vyuziti adres promennych poskytuje nepredvidatelny zdroj dat
3. **ID procesu** - identifikator aktualniho procesu
4. **Informace o vlakne** - adresa objektu aktualniho vlakna
5. **Environmentalni promenne** - hodnoty promennych prostredi systemu
6. **Adresy zasobniku a haldy** - umisteni v pameti poskytuje dalsi entropii

### Algoritmus michani

Generator pouziva nasledujici techniky pro michani entropie:

1. Prvociselne nasobitele (linearne kongruentni generator)
2. Bitove operace (XOR, rotace, posunuti)
3. Hashovani pro lepsi rovnomernou distribuci
4. Dynamicke aktualizace stavu pri generovani kazdeho cisla
5. Tajny buffer, ktery se vyuziva pro nelinearni operace

### Implementace

Hlavni trida `NahodaGenerator` obsahuje:

- `state` (128 bitu) - uchovava aktualni stav generatoru
- `counter` - pocitadlo generovanych cisel, zajistuje rozdilnost vysledku
- `tajemstvi` - buffer nahodnych hodnot pro slozite michani

#### Hlavni metody:

- `new()` - inicializuje generator nasbiranim entropie z ruznych zdroju
- `dalsi_cislo()` - generuje dalsi nahodne 64-bitove cislo
- `cislo_v_rozsahu(min, max)` - generuje cislo v zadanem rozsahu [min, max]

## Bezpecnost

Generator je navrzeny tak, aby poskytoval vysoce nepredvidatelna cisla. Diky mnozstvi zdroju entropie a slozitemu algoritmu michani je nerealne predpovedet dalsi generovane cislo. Generator je proto vhodny pro kryptograficke ucely, herni aplikace a simulace.

## Kompilace a spusteni

Pro kompilaci projektu pouzijte nasledujici prikazy:

```
cargo build --release
```

Pro spusteni:

```
cargo run --release -- <min> <max> [pocet]
```

nebo:

```
target\release\nahoda.exe <min> <max> [pocet]
```
