---
title: 2D Zoznamy
author: Michal Spano
header-includes: |
    \usepackage[utf8]{inputenc}
    \usepackage[slovak]{babel}
    \usepackage{fancyhdr}
    \usepackage{gensymb}
    \usepackage{textcomp}
    \pagestyle{fancy}
    \fancyhead[CO,CE]{Príklady Doučovanie Python}
    \fancyfoot[CO,CE]{Materiály Doučovanie - Michal Spano, © 2026}
    \fancyfoot[LE,RO]{\thepage}
papersize: a4
fontsize: 12pt
colorlinks: true
geometry: margin=2cm
output: pdf_document
---

## Tabuľka čísel

Máš nasledujúci 2D zoznam:

```python
zz = [[1, 2, 3],
      [4, 5, 6],
      [7, 8, 9]]
```

Bez spustenia programu odpovedz na otázky a potom si odpovede over:

1. Čo vypíše `print(zz[0])`?
2. Čo vypíše `print(zz[1][1])`?
3. Čo vypíše `print(zz[2][1:])`?

> **Otázka**: Prečo sa na prístup k jednému prvku používajú *dve* hranatné zátvorky? Čo predstavuje každá z nich?

## Úprava tabuľky

Vychádzaj z rovnakej tabuľky `zz` ako v predošlej úlohe. Vykonaj nasledujúce operácie **v tomto poradí** a po každej z nich vypíš celý zoznam `zz`:

1. Pridaj nový riadok `[10, 11, 12]`.
2. Otočte poradie prvkov v druhom riadku.
3. Vlož hodnotu `100` na druhú pozíciu prvého riadku.
4. Vymaž predposlednú hodnotu z prvého riadku.
5. Vymaž celý tretí riadok.

> **Tip**: Využi metódy `append`, `reverse`, `insert` a `pop`.

## Súčet riadkov

Vytvor funkciu `sucty_riadkov(tabulka)`, ktorá pre každý riadok 2D zoznamu vypíše jeho súčet. Otestuj ju na tabuľke:

```python
tabulka = [[1, 2, 3],
           [4, 5, 6],
           [7, 8, 9]]
```

Očakávaný výstup:

```
Riadok 1: 6
Riadok 2: 15
Riadok 3: 24
```

> **Tip**: Použi *vnorené slučky* — vonkajšia prechádza riadky, vnútorná prvky v riadku.

## Hľadanie hodnoty

Vytvor funkciu `najdi(tabulka, hodnota)`, ktorá nájde zadanú `hodnota` v 2D zozname a vypíše, v ktorom riadku a stĺpci sa nachádza (indexy začínajú od $1$). Ak sa hodnota nenachádza, vypíše `"Nenašlo sa."`.

Otestuj na tabuľke z predošlej úlohy s hodnotami `5`, `9` a `99`.

Očakávaný výstup pre `najdi(tabulka, 5)`:

```
Hodnota 5 je na riadku 2, stĺpci 2.
```

> **Otázka**: Čo sa stane, ak sa rovnaká hodnota nachádza na viacerých miestach? Ako by si upravil funkciu, aby vypísala *všetky* výskyty?

## Náhodná tabuľka

Vytvor 2D zoznam o rozmere $5 \times 5$ obsahujúci náhodné celé čísla od $1$ po $50$. Tabuľku vypíš prehľadne — každý riadok na samostatný riadok, čísla oddelené medzerou\footnote{Tip: Použi `random.randint(a, b)` a dve vnorené slučky — jednu pre riadky, druhú pre stĺpce.}.

Očakávaný výstup (čísla sa líšia):

```
12  4 37 19  8
 3 45  2 28 11
...
```

> **Tip**: Na zarovnanie čísel použi formátovanie reťazca, napr. `f"{cislo:3}"`.

## Piškvorky — výpis

Máš nasledujúci 2D zoznam predstavujúci hraciu plochu piškvoriek:

```python
p = [['X', ' ', 'X'],
     ['O', 'O', 'O'],
     [' ', 'X', 'X']]
```

Vytvor funkciu `vypis_piskvorky(piskvorky)`, ktorá pomocou *vnorených slučiek* vypíše hraciu plochu do konzoly. Každý symbol oddeľ medzerou.

Očakávaný výstup:

```
X   X
O O O
  X X
```

> **Otázka**: Prečo je výhodnejšie uložiť hraciu plochu do 2D zoznamu namiesto deviatich samostatných premenných?

## Piškvorky — výhra

Rozšír program z predošlej úlohy o funkciu `skontroluj_vyhrу(piskvorky, hrac)`, ktorá skontroluje, či daný hráč (`'X'` alebo `'O'`) vyhral. Hráč vyhrá, ak má obsadený celý riadok, celý stĺpec alebo jednu z dvoch uhlopriečok\footnote{Tip: Skontroluj zvlášť všetky riadky, potom všetky stĺpce, potom obe uhlopriečky.}.

Otestuj funkciu na týchto prípadoch:

```python
p1 = [['O', 'O', 'O'],
      ['X', ' ', 'X'],
      [' ', 'X', ' ']]  # O vyhral (riadok)

p2 = [['X', 'O', ' '],
      ['X', 'O', ' '],
      ['X', ' ', 'O']]  # X vyhral (stĺpec)
```

## Transpozícia tabuľky

Vytvor funkciu `transponuj(tabulka)`, ktorá vráti *transponovanú* tabuľku — riadky a stĺpce sa navzájom vymenia. Funkcia nesmie modifikovať pôvodný zoznam.

Otestuj na tabuľke:

```python
t = [[1, 2, 3],
     [4, 5, 6]]
```

Očakávaný výsledok:

```python
[[1, 4],
 [2, 5],
 [3, 6]]
```

> **Otázka**: Aké rozmery má transponovaná tabuľka, ak má pôvodná $m$ riadkov a $n$ stĺpcov?

## Maximá riadkov

Vytvor funkciu `maxima_riadkov(tabulka)`, ktorá vráti nový *jednorozmerný* zoznam obsahujúci maximálnu hodnotu z každého riadku tabuľky. Použi funkciu `max()`.

Otestuj na náhodnej tabuľke $3 \times 4$ a vypíš pôvodnú tabuľku aj zoznam maxím.

Očakávaný výstup (čísla sa líšia):

```
Tabuľka:
 7 12  3 45
18  2 33  9
 1 55 27  4

Maximá: [45, 33, 55]
```

## Pasca na míny

Vytvor jednoduchú *hru na míny*. Hraciu plochu $4 \times 4$ reprezentuj ako 2D zoznam, kde `'M'` označuje mínu a `'.'` prázdne pole. Míny rozmiestnene náhodne (celkovo $4$ míny).

Vytvor funkciu `vypis_plochu(plocha)` a funkciu `odkry_pole(plocha, riadok, stlpec)`, ktorá:

- Ak je pole mína, vypíše `"Bum! Šlápol si na mínu."`,
- Inak vypíše `"Bezpečné pole."` a označí ho ako odkryté (napr. zmeň `'.'` na `'O'`).

> **Tip**: Náhodné rozmiestnenie mín môžeš riešiť tak, že vygeneruješ náhodné indexy `(riadok, stlpec)` a skontroluješ, či tam mínu ešte nemáš.

# Grafické príklady

Tu sa trochu pohráme s Tkinter-om, ktorý nám pomôže s vizualizáciou.

## Vizualizácia tabuľky

Vytvor program, ktorý zobrazí ľubovoľný 2D zoznam čísel ako mriežku v Tkinter-i. Každé pole nakresli ako štvorec `60x60` px a doprostred neho vypíš hodnotu. Otestuj na tabuľke:

```python
tabulka = [[1,  2,  3,  4],
           [5,  6,  7,  8],
           [9, 10, 11, 12]]
```

> **Tip**: Na vykreslenie štvorca použi `create_rectangle`, na text `create_text`. Súradnice bunky na pozícii `(r, s)` sú `x = s * 60`, `y = r * 60`.

## Tepelná mapa

Vytvor program, ktorý zobrazí 2D zoznam $8 \times 8$ náhodných čísel od $0$ po $100$ ako *tepelnú mapu*. Každé pole zafarbí odtieňom od modrej (malé hodnoty) po červenú (veľké hodnoty)\footnote{Tip: Pre hodnotu $v$ v rozsahu $0$–$100$ vypočítaj červenú zložku ako $\lfloor 2.55 \cdot v \rfloor$ a modrú ako $\lfloor 2.55 \cdot (100 - v) \rfloor$. Farbu zostav ako hex reťazec: \texttt{f"\#{r:02x}00\{b:02x\}".}}. Rozmer každého poľa je `50x50` px.

> **Otázka**: Čo by sa zmenilo, keby si použil rozsah čísel $0$–$1000$ namiesto $0$–$100$? Ako by si upravil výpočet farby?

## Násobilka

Vytvor program, ktorý zobrazí násobilku $10 \times 10$ ako farebnú mriežku v Tkinter-i. Hodnotu každej bunky vypočítaj ako súčin jej riadkového a stĺpcového indexu (začínaj od $1$). Bunky so súčinom väčším ako $50$ zafarbí na `"salmon"`, ostatné na `"lightyellow"`. Doprostred každej bunky vypíš výsledok.

> **Otázka**: Ktoré bunky budú červené? Vieš to určiť ešte pred spustením programu?

## Stĺpcový diagram

Máš 2D zoznam, kde každý vnútorný zoznam predstavuje výsledky jedného žiaka v $5$ predmetoch:

```python
znamky = [[1, 2, 1, 3, 2],
          [3, 3, 2, 4, 3],
          [1, 1, 2, 1, 2],
          [4, 3, 3, 2, 4]]
```

Vytvor program, ktorý pre každého žiaka vypočíta priemer jeho známok a zobrazí **stĺpcový diagram** v Tkinter-i — každý stĺpec zodpovedá jednému žiakovi. Výška stĺpca nech je úmerná priemeru (plátno má výšku `300` px, maximum diagramu je `5`)\footnote{Tip: Výška stĺpca pre priemer $p$: $\mathit{vyska} = (p / 5) \cdot 250$. Stĺpec kreslíme od spodku plátna nahor.}. Pod každý stĺpec vypíš číslo žiaka.

> **Otázka**: Ako by si upravil program, ak by počet žiakov alebo predmetov nebol vopred známy?
