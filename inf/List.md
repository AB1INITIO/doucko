---
title: Príklady na zoznamy
author: Michal Spano
header-includes: |
    \usepackage{fancyhdr}
    \usepackage{gensymb}
    \usepackage{textcomp}
    \pagestyle{fancy}
    \fancyhead[CO,CE]{Príklady na zoznamy}
    \fancyfoot[CO,CE]{Michal Spano, © 2026}
    \fancyfoot[LE,RO]{\thepage}
papersize: a4
fontsize: 12pt
colorlinks: true
geometry: margin=2cm
output: pdf_document
---

Cvičenia v tomto dokumente sú zamerané na témy: (a) **zoznamy**, (b) slučky aj
(c) funkcie. Sú rozdelené do 3 kategórii: (i) základné, (ii) pokročilé, (iii)
pre nadšencov, kde (ii)--(iii) sú určené pre pokročilých programátorov. Pre
väčšinu študentov bude zdolanie príkladov z katégorie (i) postačujúce.

# Základné cvičenia

V tejto sekcií sú cvičenia pre začiatočníkov, ktoré sú vytvorené pre všeobecné
testovanie témy *zoznamy* v Python-e.

## Súčet párnych

Napíš funkciu `sucet_parnych(cisla)`, ktorá ako vstup očakáva zoznam s
hodnotami `int` a vypočíta súčet všetkých párnych hodnôt z tohto zoznamu.

```python
def sucet_parnych(cisla):
    # TODO
    pass

# Test cisla = [1, 2, 3, 4, 5, 6, 7, 8, 9, 10] vysledok = sucet_parnych(cisla)
print("Súčet párnych čísel:", vysledok)  # Output: 30 (2 + 4 + 6 + 8 + 10 = 30)
```

## Maximum

Napíš funkciu `najdi_maximum(cisla)`, ktorá ako vstup očakáva zoznam s
hodnotami `int` a nájde maximálnu, resp. najvyššiu, hodnotu z tohto zoznamu.

```python
def najdi_maximum(cisla):
    # TODO
    pass

# Test cisla = [1, 10, 2, 6, 7] vysledok = najdi_maximum(cisla)
print("Maximum:", vysledok)  # Output: 10 ```

Riešenie nesmie používať funkciu `max` zo štandardnej knižnice `Python`-u.

## Prekladanie

Napíš funkciu `preloz_zoznamy(zoznam1, zoznam2)`, ktorá ako vstup očakáva
`zoznam1` a `zoznam2` (t.j. zoznamy s hodnotami `int`) a vytvorý nový zoznam
tak, že skombinuje prvky v následujúcej forme:

```txt[a, b, ...], [c, d, ...] -> [a, c, b, d, ...] ```

Tvoj algoritmus musí správne fungovať pre následovné typy vstupov:

a) `zoznam1` a `zoznam2` majú zhodný počet prvkov; b) `zoznam1` a `zoznam2`
majú rozlišný počet prvkov.

```python def preloz_zoznamy(zoznam1, zoznam2): # TODO pass

# Test 1 - zhodný počet prvkov zoznam1 = [1, 2, 3] zoznam2 = [4, 5, 6] vysledok
= preloz_zoznamy(zoznam1, zoznam2) print("Prelozeny zoznam:", vysledok)  #
Output: [1, 4, 2, 5, 3, 6]

# Test 2 - rozlišný počet prvkov zoznam1 = [1, 2, 3]
zoznam2 = [4, 5, 6, 7, 10]
vysledok = preloz_zoznamy(zoznam1, zoznam2)
print("Prelozeny zoznam:", vysledok)  # Output: [1, 4, 2, 5, 3, 6, 7, 10]

# Test 3 - rozlišný počet prvkov
zoznam1 = [1, 2]
zoznam2 = []
vysledok = preloz_zoznamy(zoznam1, zoznam2)
print("Prelozeny zoznam:", vysledok)  # Output: [1, 2]
```

Riešenie a), b) môže byť napísané do 1 alebo 2 programov. Riešenie nesmie však
používať funkciu `zip` zo štandardnej knižnice `Python`-u.

## Súčet prvkov

Napíš funkciu `sucet_prvkov(cisla)`, ktorá ako vstup očakáva zoznam s hodnotami `int`
a vráti súčet všetkých prvkov tohto zoznamu.

```python
def sucet_prvkov(cisla):
  # TODO
  pass

# Test
cisla = [3, 7, 1, 9, 2]
vysledok = sucet_prvkov(cisla)
print("Súčet prvkov:", vysledok)  # Output: 22

cisla = []
vysledok = sucet_prvkov(cisla)
print("Súčet prvkov:", vysledok)  # Output: 0
```

Riešenie nesmie používať funkciu `sum` zo štandardnej knižnice `Python`-u.

## Počet výskytov

Napíš funkciu `pocet_vyskytov(zoznam, prvok)`, ktorá ako vstup očakáva zoznam a hodnotu
`prvok`, a vráti počet výskytov hodnoty `prvok` v danom zozname.

```python
def pocet_vyskytov(zoznam, prvok):
  # TODO
  pass

# Test
zoznam = [1, 3, 2, 3, 5, 3, 7]
vysledok = pocet_vyskytov(zoznam, 3)
print("Počet výskytov:", vysledok)  # Output: 3

vysledok = pocet_vyskytov(zoznam, 9)
print("Počet výskytov:", vysledok)  # Output: 0
```

Riešenie nesmie používať metódu `.count()`.

## Obrátenie

Napíš funkciu `otoc_zoznam(zoznam)`, ktorá ako vstup očakáva zoznam a vráti nový zoznam
s prvkami v opačnom poradí.

```python
def otoc_zoznam(zoznam):
  # TODO
  pass

# Test
zoznam = [1, 2, 3, 4, 5]
vysledok = otoc_zoznam(zoznam)
print("Obrátený zoznam:", vysledok)  # Output: [5, 4, 3, 2, 1]

zoznam = [7]
vysledok = otoc_zoznam(zoznam)
print("Obrátený zoznam:", vysledok)  # Output: [7]
```

Riešenie nesmie používať funkciu `reversed` ani metódu `.reverse()` zo štandardnej
knižnice `Python`-u.

## Bez duplicít

Napíš funkciu `odstran_duplicity(zoznam)`, ktorá ako vstup očakáva zoznam a vráti nový
zoznam, v ktorom sa každý prvok vyskytuje najviac raz. Poradie prvkov musí zostať zachované
(t.j. zachovaj prvý výskyt každého prvku).

```python
def odstran_duplicity(zoznam):
  # TODO
  pass

# Test
zoznam = [1, 2, 3, 2, 4, 1, 5]
vysledok = odstran_duplicity(zoznam)
print("Bez duplicít:", vysledok)  # Output: [1, 2, 3, 4, 5]

zoznam = [7, 7, 7]
vysledok = odstran_duplicity(zoznam)
print("Bez duplicít:", vysledok)  # Output: [7]
```

## Usporiadaný?

Napíš funkciu `je_usporiadany(zoznam)`, ktorá ako vstup očakáva zoznam s hodnotami `int`
a vráti `True`, ak sú prvky zoznamu usporiadané vzostupne (od najmenšieho po najväčší),
inak vráti `False`.

```python
def je_usporiadany(zoznam):
  # TODO
  pass

# Test
zoznam = [1, 2, 3, 4, 5]
vysledok = je_usporiadany(zoznam)
print("Usporiadaný:", vysledok)  # Output: True

zoznam = [1, 3, 2, 4]
vysledok = je_usporiadany(zoznam)
print("Usporiadaný:", vysledok)  # Output: False

zoznam = [5]
vysledok = je_usporiadany(zoznam)
print("Usporiadaný:", vysledok)  # Output: True
```

## Prienik

Napíš funkciu `spolocne_prvky(zoznam1, zoznam2)`, ktorá ako vstup očakáva dva zoznamy
s hodnotami `int` a vráti nový zoznam obsahujúci iba prvky, ktoré sa nachádzajú v oboch
zoznamoch. Každý spoločný prvok sa vo výsledku vyskytuje najviac raz.

```python
def spolocne_prvky(zoznam1, zoznam2):
  # TODO
  pass

# Test
zoznam1 = [1, 2, 3, 4, 5]
zoznam2 = [3, 4, 5, 6, 7]
vysledok = spolocne_prvky(zoznam1, zoznam2)
print("Spoločné prvky:", vysledok)  # Output: [3, 4, 5]

zoznam1 = [1, 2, 3]
zoznam2 = [4, 5, 6]
vysledok = spolocne_prvky(zoznam1, zoznam2)
print("Spoločné prvky:", vysledok)  # Output: []
```

## Filtrovanie

Napíš funkciu `filtruj_vacsie(cisla, hranica)`, ktorá ako vstup očakáva zoznam s hodnotami
`int` a číslo `hranica`, a vráti nový zoznam obsahujúci iba tie prvky, ktoré sú **väčšie**
ako `hranica`.

```python
def filtruj_vacsie(cisla, hranica):
  # TODO
  pass

# Test
cisla = [1, 8, 3, 10, 5, 7]
vysledok = filtruj_vacsie(cisla, 5)
print("Väčšie ako hranica:", vysledok)  # Output: [8, 10, 7]

vysledok = filtruj_vacsie(cisla, 10)
print("Väčšie ako hranica:", vysledok)  # Output: []
```

## Rotácia

Napíš funkciu `rotuj_zoznam(zoznam, k)`, ktorá ako vstup očakáva zoznam a celé číslo `k`,
a vráti nový zoznam posunutý o `k` pozícií doprava. Prvky, ktoré „vypadnú" z konca
zoznamu, sa objavia na jeho začiatku.

```python
def rotuj_zoznam(zoznam, k):
  # TODO
  pass

# Test
zoznam = [1, 2, 3, 4, 5]
vysledok = rotuj_zoznam(zoznam, 2)
print("Posunutý zoznam:", vysledok)  # Output: [4, 5, 1, 2, 3]

vysledok = rotuj_zoznam(zoznam, 0)
print("Posunutý zoznam:", vysledok)  # Output: [1, 2, 3, 4, 5]

vysledok = rotuj_zoznam(zoznam, 5)
print("Posunutý zoznam:", vysledok)  # Output: [1, 2, 3, 4, 5]
```

## Druhé minimum

Napíš funkciu `druhe_minimum(cisla)`, ktorá ako vstup očakáva zoznam s hodnotami `int`
(so aspoň dvoma rôznymi hodnotami) a vráti druhú najmenšiu hodnotu z tohto zoznamu.

```python
def druhe_minimum(cisla):
  # TODO
  pass

# Test
cisla = [4, 1, 7, 2, 9]
vysledok = druhe_minimum(cisla)
print("Druhé minimum:", vysledok)  # Output: 2

cisla = [5, 5, 3, 1, 1, 2]
vysledok = druhe_minimum(cisla)
print("Druhé minimum:", vysledok)  # Output: 2
```

Riešenie nesmie používať funkciu `min` ani `sorted` zo štandardnej knižnice `Python`-u.

## Vnorené zoznamy

Napíš funkciu `spoj_vnorene(zoznamy)`, ktorá ako vstup očakáva zoznam zoznamov (t.j. každý
prvok vstupu je zoznam celých čísel) a vráti jediný plochý zoznam so všetkými prvkami
v pôvodnom poradí.

```python
def spoj_vnorene(zoznamy):
  # TODO
  pass

# Test
zoznamy = [[1, 2], [3, 4], [5]]
vysledok = spoj_vnorene(zoznamy)
print("Spojený zoznam:", vysledok)  # Output: [1, 2, 3, 4, 5]

zoznamy = [[], [1], [], [2, 3]]
vysledok = spoj_vnorene(zoznamy)
print("Spojený zoznam:", vysledok)  # Output: [1, 2, 3]
```

## Aritm. priemer

Napíš funkciu `priemer(cisla)`, ktorá ako vstup očakáva neprázdny zoznam s hodnotami `int`
a vráti priemer (aritmetický stred) všetkých prvkov ako desatinné číslo (`float`).

```python
def priemer(cisla):
  # TODO
  pass

# Test
cisla = [1, 2, 3, 4, 5]
vysledok = priemer(cisla)
print("Priemer:", vysledok)  # Output: 3.0

cisla = [10, 20, 30]
vysledok = priemer(cisla)
print("Priemer:", vysledok)  # Output: 20.0

cisla = [7]
vysledok = priemer(cisla)
print("Priemer:", vysledok)  # Output: 7.0
```

Riešenie nesmie používať funkciu `sum` ani `len` zo štandardnej knižnice `Python`-u.

## Hľadanie indexu

Napíš funkciu `najdi_index(zoznam, prvok)`, ktorá ako vstup očakáva zoznam a hľadanú
hodnotu `prvok`. Ak sa `prvok` v zozname nachádza, funkcia vráti index jeho **prvého**
výskytu. Ak sa `prvok` v zozname nenachádza, funkcia vráti `-1`.

```python
def najdi_index(zoznam, prvok):
  # TODO
  pass

# Test
zoznam = [10, 20, 30, 20, 40]
vysledok = najdi_index(zoznam, 20)
print("Index:", vysledok)  # Output: 1

vysledok = najdi_index(zoznam, 99)
print("Index:", vysledok)  # Output: -1

vysledok = najdi_index(zoznam, 10)
print("Index:", vysledok)  # Output: 0
```

Riešenie nesmie používať metódu `.index()` ani funkciu `enumerate` zo štandardnej
knižnice `Python`-u.

\newpage

# Pokročilé

## Priebežný súčet

Pre zoznam $a = [a_0, a_1, \ldots, a_{n-1}]$ definujeme **priebežný súčet** ako nový
zoznam $S$, kde každý prvok $S_i$ je definovaný ako:

$$S_i = \sum_{j=0}^{i} a_j$$

Napíš funkciu `priebezny_sucet(cisla)`, ktorá ako vstup očakáva zoznam s hodnotami `int`
a vráti zoznam $S$ podľa vzorca vyššie.

```python
def priebezny_sucet(cisla):
  # TODO
  pass

# Test
cisla = [1, 2, 3, 4, 5]
vysledok = priebezny_sucet(cisla)
print("Priebežný súčet:", vysledok)  # Output: [1, 3, 6, 10, 15]

cisla = [5, -3, 2, 0, 4]
vysledok = priebezny_sucet(cisla)
print("Priebežný súčet:", vysledok)  # Output: [5, 2, 4, 4, 8]

cisla = [7]
vysledok = priebezny_sucet(cisla)
print("Priebežný súčet:", vysledok)  # Output: [7]
```

*Tip: premysli si, ako môžeš pri výpočte $S_i$ využiť už vypočítanú hodnotu $S_{i-1}$.*

## Kĺzavý priemer

Pre zoznam $a = [a_0, a_1, \ldots, a_{n-1}]$ a veľkosť okna $k$ definujeme **kĺzavý
priemer** ako zoznam $M$ s $n - k + 1$ prvkami, kde:

$$M_i = \frac{1}{k} \sum_{j=i}^{i+k-1} a_j, \quad i = 0, 1, \ldots, n-k$$

Napíš funkciu `klzavy_priemer(cisla, k)`, ktorá ako vstup očakáva zoznam s hodnotami
`int` a veľkosť okna $k$, a vráti zoznam $M$ podľa vzorca vyššie.

```python
def klzavy_priemer(cisla, k):
  # TODO
  pass

# Test
cisla = [1, 2, 3, 4, 5]
vysledok = klzavy_priemer(cisla, 3)
print("Kĺzavý priemer:", vysledok)  # Output: [2.0, 3.0, 4.0]

cisla = [10, 20, 30, 40]
vysledok = klzavy_priemer(cisla, 2)
print("Kĺzavý priemer:", vysledok)  # Output: [15.0, 25.0, 35.0]

cisla = [4, 4, 4, 4]
vysledok = klzavy_priemer(cisla, 4)
print("Kĺzavý priemer:", vysledok)  # Output: [4.0]
```

*Tip: kĺzavý priemer sa používa napríklad pri analýze finančných dát alebo signálov —
je to jeden z najjednoduchších spôsobov, ako „vyhladiť" hodnoty v čase.*

## Kódovanie dĺžkou

Nech $a = [a_0, a_1, \ldots, a_{n-1}]$ je zoznam. **Kódovanie dĺžkou** (*run-length
encoding*) rozdelí $a$ na maximálne súvislé úseky rovnakých hodnôt a každý úsek nahradí
dvojicou $(v, c)$, kde $v$ je hodnota a $c$ je jej počet výskytov v danom úseku. Formálne,
výsledok je zoznam dvojíc:

$$[(v_1, c_1),\ (v_2, c_2),\ \ldots,\ (v_m, c_m)]$$

kde $\sum_{i=1}^{m} c_i = n$ a $v_i \neq v_{i+1}$ pre každé $i$.

Napíš funkciu `kodovanie_dlzkou(zoznam)`, ktorá ako vstup očakáva zoznam s hodnotami `int`
a vráti jeho zakódovanú reprezentáciu podľa definície vyššie.

```python
def kodovanie_dlzkou(zoznam):
  # TODO
  pass

# Test
zoznam = [1, 1, 2, 3, 3, 3, 2, 2]
vysledok = kodovanie_dlzkou(zoznam)
print("Kódovanie:", vysledok)  # Output: [(1, 2), (2, 1), (3, 3), (2, 2)]

zoznam = [5, 5, 5, 5]
vysledok = kodovanie_dlzkou(zoznam)
print("Kódovanie:", vysledok)  # Output: [(5, 4)]

zoznam = [1, 2, 3]
vysledok = kodovanie_dlzkou(zoznam)
print("Kódovanie:", vysledok)  # Output: [(1, 1), (2, 1), (3, 1)]
```

*Tip: táto technika sa používa pri kompresii dát — namiesto ukladania $[1, 1, 1, 1, 1]$
stačí uložiť $[(1, 5)]$.*

## Transpozícia

Pre maticu $A$ rozmeru $m \times n$ definujeme jej **transponovanú maticu** $A^T$ rozmeru
$n \times m$ takto:

$$(A^T)_{ij} = A_{ji}$$

Napíš funkciu `transponuj(matica)`, ktorá ako vstup očakáva maticu reprezentovanú ako
zoznam riadkov (každý riadok je zoznam celých čísel rovnakej dĺžky) a vráti $A^T$.

```python
def transponuj(matica):
  # TODO
  pass

# Test
matica = [
  [1, 2, 3],
  [4, 5, 6]
]
vysledok = transponuj(matica)
print("Transponovaná:", vysledok)  # Output: [[1, 4], [2, 5], [3, 6]]

matica = [
  [1, 2],
  [3, 4],
  [5, 6]
]
vysledok = transponuj(matica)
print("Transponovaná:", vysledok)  # Output: [[1, 3, 5], [2, 4, 6]]

matica = [[7]]
vysledok = transponuj(matica)
print("Transponovaná:", vysledok)  # Output: [[7]]
```

*Tip: prvok na pozícii $A_{ij}$ skončí po transpozícii na pozícii $(A^T)_{ji}$ — riadky
a stĺpce sa navzájom vymenia.*

## Caesarova šifra

**Caesarova šifra** je jednoduchá substitučná šifra, ktorá každý znak $z$ abecedy
$\{\texttt{a}, \ldots, \texttt{z}\}$ nahradí znakom posunutým o $p$ pozícií. Formálne,
zakódovaný znak $z'$ je:

$$z' = \left((\text{ord}(z) - \text{ord}(\texttt{a}) + p) \bmod 26\right) + \text{ord}(\texttt{a})$$

kde $\text{ord}(\cdot)$ označuje ASCII hodnotu znaku a operácia $\bmod$ zabezpečuje
wraparound po konci abecedy.

Napíš funkciu `caesarova_sifra(znaky, posun)`, ktorá ako vstup očakáva zoznam malých
písmen anglickej abecedy a celé číslo $p$ (`posun`), a vráti nový zoznam zakódovaných
znakov podľa vzorca vyššie.

```python
def caesarova_sifra(znaky, posun):
  # TODO
  pass

# Test
znaky = ['h', 'e', 'l', 'l', 'o']
vysledok = caesarova_sifra(znaky, 3)
print("Šifra:", vysledok)  # Output: ['k', 'h', 'o', 'o', 'r']

vysledok = caesarova_sifra(znaky, 0)
print("Šifra:", vysledok)  # Output: ['h', 'e', 'l', 'l', 'o']

znaky = ['x', 'y', 'z']
vysledok = caesarova_sifra(znaky, 2)
print("Šifra:", vysledok)  # Output: ['z', 'a', 'b']
```

*Tip: znaky vieš previesť na čísla pomocou funkcie `ord()` a späť pomocou `chr()`.
Vzorec vyššie priamo ukazuje, ako tieto funkcie použiť.*

\newpage

# Pre nadšencov

## Redukcia

Štruktúra $(\mathcal{M},\ \oplus,\ e)$ sa nazýva **monoid**, ak platí:

$$a \oplus (b \oplus c) = (a \oplus b) \oplus c \qquad \text{(asociatívnosť)}$$
$$e \oplus a = a \oplus e = a \qquad \text{(existencia neutrálneho prvku)}$$

Operácia **redukcia** (angl. *fold*) „skladá" zoznam do jednej hodnoty postupnou aplikáciou
$\oplus$ zľava doprava, pričom začína od neutrálneho prvku $e$:

$$\text{zloz}([a_0, a_1, \ldots, a_{n-1}],\ \oplus,\ e)\ =\ (\ldots((e \oplus a_0) \oplus a_1) \oplus \ldots) \oplus a_{n-1}$$

> **Príklad:** Súčet je redukcia s $\oplus = +$ a $e = 0$; súčin s $\oplus = \times$ a $e = 1$;
> maximum s $\oplus = \max$ a $e = -\infty$. Rovnaká funkcia `zloz` pokryje všetky tri prípady.

Napíš funkciu `zloz(zoznam, op, e)`, kde `op` je dvojargumentová funkcia predstavujúca
$\oplus$ a `e` je neutrálny prvok. Funkcia vráti výsledok redukcie zoznamu.

```python
def zloz(zoznam, op, e):
  # TODO
  pass

# Test - súčet
vysledok = zloz([1, 2, 3, 4, 5], lambda a, b: a + b, 0)
print("Súčet:", vysledok)  # Output: 15

# Test - súčin
vysledok = zloz([1, 2, 3, 4, 5], lambda a, b: a * b, 1)
print("Súčin:", vysledok)  # Output: 120

# Test - maximum
vysledok = zloz([3, 1, 4, 1, 5, 9], lambda a, b: a if a > b else b, float('-inf'))
print("Maximum:", vysledok)  # Output: 9

# Test - prázdny zoznam
vysledok = zloz([], lambda a, b: a + b, 0)
print("Prázdny:", vysledok)  # Output: 0
```

## Zobrazenie

Zoznamy tvoria **funktor**: existuje operácia, ktorá aplikuje funkciu $f : A \to B$
na každý prvok zoznamu bez zmeny jeho štruktúry. Táto operácia (angl. *map*) musí
spĺňať dva **funktoriálne zákony**:

$$\text{zobraz}(z,\ \text{id}) = z \qquad \text{(zákon identity)}$$
$$\text{zobraz}(z,\ f \circ g) = \text{zobraz}(\text{zobraz}(z,\ g),\ f) \qquad \text{(zákon kompozície)}$$

kde $\text{id}(x) = x$ a $(f \circ g)(x) = f(g(x))$.

> **Príklad:** Zoznam teplôt v stupňoch Celzia $[0, 20, 37, 100]$ prevedieš na Fahrenheit
> jedným volaním `zobraz(cisla, lambda c: c * 9/5 + 32)` — bez jediného cyklu v hlavnom kóde.

Napíš funkciu `zobraz(zoznam, f)`, ktorá aplikuje funkciu `f` na každý prvok zoznamu
a vráti nový zoznam výsledkov. Overte oba funktoriálne zákony na konkrétnych vstupoch.

```python
def zobraz(zoznam, f):
  # TODO
  pass

# Test - prevod Celsius -> Fahrenheit
cisla = [0, 20, 37, 100]
vysledok = zobraz(cisla, lambda c: c * 9/5 + 32)
print("Fahrenheit:", vysledok)  # Output: [32.0, 68.0, 98.6, 212.0]

# Test - zákon identity
zoznam = [1, 2, 3]
assert zobraz(zoznam, lambda x: x) == zoznam  # musí platiť

# Test - zákon kompozície
f = lambda x: x + 1
g = lambda x: x * 2
assert zobraz(zoznam, lambda x: f(g(x))) == zobraz(zobraz(zoznam, g), f)  # musí platiť
print("Funktoriálne zákony platia.")
```

## Reťazenie

**Listová monáda** modeluje výpočet s viacerými možnými výsledkami naraz. Jej kľúčová
operácia *bind* (angl. *flatMap*) aplikuje funkciu $f : A \to [B]$ na každý prvok zoznamu
a výsledné zoznamy skonkatenuje do jedného:

$$\text{retaz}([a_0, \ldots, a_{n-1}],\ f)\ =\ f(a_0)\ +\ f(a_1)\ +\ \ldots\ +\ f(a_{n-1})$$

kde $+$ označuje konkatenáciu zoznamov. Platí tiež identita monády:

$$\text{retaz}(z,\ \lambda\, x.\ [x])\ =\ z$$

> **Príklad:** Máš zoznam tried `["1A", "1B"]` a funkciu, ktorá pre každú triedu vráti
> zoznam žiakov. Pomocou `retaz` dostaneš jediný plochý zoznam všetkých žiakov školy.

Napíš funkciu `retaz(zoznam, f)`, kde `f` vracia zoznam pre každý vstupný prvok.

```python
def retaz(zoznam, f):
  # TODO
  pass

# Test - rozklad na cifry
cisla = [12, 305, 7]
vysledok = retaz(cisla, lambda n: [int(c) for c in str(n)])
print("Cifry:", vysledok)  # Output: [1, 2, 3, 0, 5, 7]

# Test - identita monády
zoznam = [1, 2, 3]
assert retaz(zoznam, lambda x: [x]) == zoznam  # musí platiť

# Test - prázdne výsledky
vysledok = retaz([1, 2, 3], lambda x: [] if x % 2 == 0 else [x, x])
print("Párnez:", vysledok)  # Output: [1, 1, 3, 3]
```

## Kombinovanie

Funkcia **zipWith** (angl. *kombinovanie*) spája dva zoznamy prvok po prvku pomocou
zadanej binárnej funkcie $f : A \times B \to C$:

$$\text{zluc}([a_0, \ldots, a_{n-1}],\ [b_0, \ldots, b_{m-1}],\ f)\ =\ [f(a_0, b_0),\ \ldots,\ f(a_k, b_k)]$$

kde $k = \min(n, m) - 1$. Ide o základ tzv. **aplikatívneho funktora**.

> **Príklad:** Skalárny súčin dvoch vektorov $\mathbf{u} \cdot \mathbf{v} = \sum_i u_i v_i$
> sa dá elegantne vyjadriť ako:
> $$\mathbf{u} \cdot \mathbf{v} = \text{zloz}\!\left(\text{zluc}(\mathbf{u}, \mathbf{v},\ \times),\ +,\ 0\right)$$
> t.j. najprv skombinujeme zoznamy súčinmi, potom redukujeme súčtom.

Napíš funkciu `zluc(zoznam1, zoznam2, f)`, ktorá kombinuje dva zoznamy pomocou funkcie `f`.
Následne pomocou `zluc` a `zloz` z predchádzajúceho cvičenia vypočítaj skalárny súčin.

```python
def zluc(zoznam1, zoznam2, f):
  # TODO
  pass

# Test - súčet po prvkoch
vysledok = zluc([1, 2, 3], [10, 20, 30], lambda a, b: a + b)
print("Súčet:", vysledok)  # Output: [11, 22, 33]

# Test - skalárny súčin
u = [1, 2, 3]
v = [4, 5, 6]
sucin = zloz(zluc(u, v, lambda a, b: a * b), lambda a, b: a + b, 0)
print("Skalárny súčin:", sucin)  # Output: 32  (1*4 + 2*5 + 3*6)

# Test - rôzne dĺžky (berie sa kratší)
vysledok = zluc([1, 2, 3], [10, 20], lambda a, b: a + b)
print("Kratší:", vysledok)  # Output: [11, 22]
```

## Skenovanie

**Skenovanie** (angl. *scan*) je zovšeobecnením redukcie: namiesto jediného výsledku
vráti zoznam **všetkých priebežných akumulátorov** vrátane počiatočného $e$:

$$\text{skenuj}([a_0, a_1, \ldots, a_{n-1}],\ \oplus,\ e)\ =\ [e,\ e \oplus a_0,\ (e \oplus a_0) \oplus a_1,\ \ldots]$$

Výsledok má teda vždy $n + 1$ prvkov. Platí vzťah:

$$\text{skenuj}(z,\ \oplus,\ e)[n] = \text{zloz}(z,\ \oplus,\ e)$$

t.j. posledný prvok skenovania je zhodný s výsledkom redukcie.

> **Príklad:** Bankový účet so začiatočným stavom $0$ a pohybmi $[+200, -50, +300, -80]$:
> skenovanie dá históriu zostatku $[0, 200, 150, 450, 370]$ — každý medzivýsledok je
> viditeľný, nielen konečný stav.

Napíš funkciu `skenuj(zoznam, op, e)` podľa definície vyššie. Overte vzťah s `zloz`.

```python
def skenuj(zoznam, op, e):
  # TODO
  pass

# Test - priebežný súčet (vrátane štartu)
pohyby = [200, -50, 300, -80]
vysledok = skenuj(pohyby, lambda a, b: a + b, 0)
print("Zostatok:", vysledok)  # Output: [0, 200, 150, 450, 370]

# Test - prázdny zoznam
vysledok = skenuj([], lambda a, b: a + b, 0)
print("Prázdny:", vysledok)  # Output: [0]

# Test - vzťah so zloz (posledný prvok = redukcia)
zoznam = [1, 2, 3, 4, 5]
assert skenuj(zoznam, lambda a, b: a + b, 0)[-1] == zloz(zoznam, lambda a, b: a + b, 0)
print("Vzťah so zloz platí.")
```
