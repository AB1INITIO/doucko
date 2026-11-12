---
title: Príklady Doučovanie INF -- Python Verzia
author: Michal Spano
header-includes: |
    \usepackage[utf8]{inputenc}
    \usepackage[slovak]{babel}
    \usepackage{fancyhdr}
    \usepackage{gensymb} 
    \usepackage{textcomp}
    \pagestyle{fancy}
    \fancyhead[CO,CE]{Python}
    \fancyfoot[CO,CE]{Materiály Doučovanie -- Michal Špano, © 2024---2026}
    \fancyfoot[LE,RO]{\thepage}
papersize: a4
fontsize: 12pt
colorlinks: true
geometry: margin=2cm
output: pdf_document
---

\newpage

# Úvod

Často dostávam dotazy, že programovanie je len pre istú *elitu* ľudí, takých, ktorí sedia za obrazovkou denno-denne, majú silné dioptrické okuliare a dlhé vlasy. Je to blud -- čistý stereotyp. Dúfam, že Ťa o tom presvedčím. Programovanie je určené pre *každého*, kto chce riešiť hravé úlohy. Na riešenie týchto cvičení **nepotrebuješ žiadne predošlé skúsenosti**. Čo teda budem potrebovať?

> *,,Odhodlanie, čas a vytrvalosť.''*

No to predsa nie je nič nezvládnuteľné! Programovanie je zručnosť ako každá iná: hra na klavíri, lyžovanie a či písanie básni. Čím viac človek trénuje, tým to je lepšie. Preto len pasívne čítať programovanie nefunguje -- dokážeme sa naučiť lyžovať len tým, že pozeráme ZOH v TV? \;)

Máš pocit, že niečo nesedí? To sa môže stať, nikto nie je predsa perfektný \=). Napíš mail na `<spano@michalspano.com>`. Taktiež budem rád za spätnú väzbu.

<!--
![](lenny_bexter.png){width=33%}
-->

## Pár praktických poznámok

Príklady sú zväčša zoradené tématicky od náročnosti a pre odhodlaných pridávam aj *bonus*. 

Na riešenie týchto úloh ti stačí hocijaký textový editor (`vim`, IDLE, VS (Code), dokonca i Notepad) a dostatočne nový (`>= 3.8`) `Python` spúšťač. Mnohé cvičenia môžeš skúsiť priamo v shell-y.

V cvičeniach zvyknem zaznačovať vstup `<<<` a výstup `>>>` programu takto. Ukážme si malý príklad

```txt
>>> Pridaj dve cisla.
<<< a = 2
<<< b = 3
>>> 5
```

Niekedy, najmä pri funkciach, je jednoduchšie ukázať očakávanú výstupnú hodnotu priamo v riadku, resp.

```python
faktorial(5)           # 120
je_prvocislo(7)        # True
trikrat_meno('Michal') # 'MichalMichalMichal'
```

takýto kód si môžeš prekopírovať do editora a priamo otestovať.

Taktiež si prečítaj poznámky malým za pätou strany, tie slúžia\footnote{na pomoc v riešení a ponúkajú doplnkové informácie.}

> *Veľa štastia!*

\newpage

# Základy Programovania

## Premenné, podmienky a slučky

**1)** Vytvor program, ktorý sa opýta na meno, priezvisko a rok narodenia. Úlohou
programu je potom vypísať reťazec v tvare: `priezvisko_meno-ROK`\footnote{Pripomeň si, že operácia `+` funguje i na reťazcoch. Bonus: tzv. `f-string`.}. Napr.:

```txt
<<< meno      = Andrej
<<< priezisko = Slovak
<<< rok       = 2003
>>> Andrej_Slovak-2003
```

**2)** Vytvor jednoduchú *BMI kalkulačku*. Program sa spýta na výšku a váhu. Potom
vypíše hodnotu *BMI* zaokrúhlenú na dve desatinné miesta\footnote{Hint: `round(n,k)`.} podľa vzorca: $\frac{\text{váha (kg)}}{\text{výška (m)}^2}$.

**3)** Ziači píšu písomku z informatiky, ktorá má 30 bodov. V triede je príliš veľa žiakov a pani učiteľka bude cez víkend zaneprázdnená a potrebuje pomoc s
hodnotením písomiek. (a) Pomôž jej s programom, ktorý na základe stupnice priradí
žiakovi známku od $1 \dots 5$.

| Počet bodov | Známka |
| :---------: | :----: |
| $\geq 25$   | `1`    |
| $20 \to 24$ | `2`    |
| $14 \to 19$ | `3`    |
| $9 \to 13$  | `4`    |
| inak        | `5`    |

```txt
<<< pocet bodov = 7
>>> znamka => 5
```

\noindent (b) Tvoj program vypíše správu `Chyba` ak náhodou napíšeme počet bodov vyšši ako max. počet bodov písomky.

**4)** Vytvor program, ktorý zistí či existuje trojuholník $\Delta$ s dĺžkami strán $a,b,c$. Pre každý takýto trojuholník $\Delta$ platí, že:

$$\underbrace{a + b > c}_{\text{Podmienka } \#1},\; \underbrace{a + c > b}_{\text{Podmienka } \#2},\; \underbrace{b + c > a}_{\text{Podmienka } \#3}.$$

Ako správne vidíš, musia platiť až 3 podmienky **zároveň**. Teda

$$
    \text{Podmienka } \#1 \textit{ a zároveň }
    \text{podmienka } \#2 \textit{ a zároveň }
    \text{podmienka } \#3.
$$

No ako docieliť toto v programovaní? Poznáme tzv. *logické spojky* z
matematickej logiky (ktoré celkom prirodzene vyplývajú aj z jazyka, ktorý
hovoríme, napr. Slovenčina)! Ak chceme vyjadriť, že 2 podmienky platia
**zároveň**, použijeme spojku `and` (teda po slovensky 'a' alebo 'a zároveň').
Noapak, existuje aj spojka `or` (teda 'alebo').

**Otázka**: ako by sa zmenil môj program, keby použijem `or` namiesto `and`
medzi podmienkami?

**5)** Vytvor program, ktorý simuluje jedno kolo hry *kameň-papier-nožnice* (skrátene `K`,`P`,`N`). Priebeh hry môže vyzerať napr. takto:

```txt
<<< hrac1   = K
<<< hrac2   = P
>>> vyherca = hrac2

<<< hrac1   = N
<<< hrac2   = N
>>> Remiza
```

**6)** Jedno kolo hry je celkom *nuda*. Vylepši program z úlohy 5) tak, že hra
*kameň-papier-nožnice* sa opakuje, pokiaľ jeden z hráčov nezíska aspoň $3$
body\footnote{Pravdaže, za remízu nedostáva nikto bod.}. Je očividné, že na toto budeme potrebovať slučku, no ktorú? Postačí aj `for`-cyklus? Zdôvodni.\footnote{Zamysli sa: môže byť takáto hra teoreticky nekonečná?}

```txt
<<< hrac1 = K
<<< hrac2 = P

<<< hrac1 = N
<<< hrac2 = K

<<< hrac1 = P
<<< hrac2 = N

>>> vyherca = hrac2
```

**7)** Vytvor program, ktorý zistí, či dáne číslo je prvočíslo\footnote{Číslo $p$ je prvočíslo jedine ak $p$ má práve dvoch delitelov: $1, p$. Inými slovami, neexistuje iný deliteľ z intervalu $(1,p) = [2,p-1]$. Najmenšie také číslo je $2$. Každé prvočíslo je taktiež nezáporné.}.

```txt
>>> 2 je prvocislo.
>>> 9 nie je prvocislo.
>>> 17 je prvocislo.
```

Ak $a$ delí $b$, tak $\frac{a}{b}$ je celé číslo, teda delenie *nemá zvyšok*. Toto v programovaní možno zapísať ako `a % b == 0`, teda zvyšok po delení $a$ s $b$ je $0$. Operácia `_%_` sa nazýva *modulo*.

**8)** Vypíš pre dané $0 \leq n \leq 10$ tabuľku s *malou násobilkou*. Napr. pre $n = 7$ máme:

```txt
<<< n = 7
>>> 7 x 1  = 7
>>> 7 x 2  = 14
>>> 7 x 3  = 21
>>> ...
>>> 7 x 10 = 70
```

**9)** Vytvor program, ktorý náhodne vyberie číslicu $n$. Tento program sa pýta
hráča na číslicu pokiaľ neuhádne toto $n$. Pre $n=2$ vyzerá hra takto

```txt
>>> Mam nahodne cislo (hra sa zacina)
>>> =================================

<<< pokus? 4
>>> Nespravne!
<<< pokus? 8
>>> Nespravne!
<<< pokus? 2
>>> Spravne.
```

Na náhodu máme v Pythone balíček `random`, ktorý pridáš do kódu cez príkaz
`import random`. Pomôž si funkciou `randint(a,b)`, ktorá vytvorí náhodné číslo
zo zatvoreného intervalu $[a,b]$, resp. aj $a,b$ vrátane. Napr.

```python
import random
r = random.randint(1,4)
# hodnota r moze byt 1, 2, 3 alebo 4.
```

**10)** Uprav program z úlohy 9) tak, že počet pokusov nie je **neobmedzený**,
teda pridaj do kódu premennú `pocet_pokusov` (s ľubovoľnou hodnotou). 
Program sa teda ukončí (i) akonáhle uhádneme náhodné číslo alebo (ii) vyčerpáme
počet pokusov. Ak vyčerpáme pokusy, program vypíše: `Vycerpal si vsetky pokusy! Koniec.`

Určite si zistil, že v úloha **9)** použijeme `while`-cyklus. Platí to aj pre
toto cvičenie alebo môžeme použiť aj `for`-cyklus\footnote{Existuje príkaz
`break`, ktorý ihneď ukončí cyklus.}?

**11)**  Hru z **9)** trochu vylepšíme; vytvor hru, ktorá náhodne vyberie číslo
z intervalu $[1;100]$. Hráč musí toto číslo nájsť. No existuje až $100$
možností, to by bolo vcelku zdĺhavé! Program preto teda vždy vypíše či zadané
číslo je väčšie alebo menšie ako to hľadané. Ak je hľadané číslo $25$, hra môže
prebiehať takto:

```
>>> Mam nahodne cislo (hra sa zacina)
>>> =================================

<<< Zadaj cislo = 60
>>> Hladane cislo je mensie.
<<< Zadaj cislo = 48
>>> Hladane cislo je mensie.
<<< Zadaj cislo = 20
>>> Hladane cislo je vacsie.
<<< Zadaj cislo = 24
>>> Hladane cislo je vacsie.
<<< Zadaj cislo = 25
>>> Nasiel si hladane cislo!
```

Existuje stratégia pomocou ktorej môžeme túto hru vyhrať pomerne ľahko: vždy
rozdelíme interval v polovici. Teda $100 \mapsto 50, 50 \mapsto 25, 25 \mapsto
\approx 13, \dots$. Koľko potrebujeme **aspoň** (minimálne) pokusov aby sme
takúto hru touto stratégiou vyhrali?

**Bonus**: dokážeme tento výpočet zovšeobecniť pre hru, kde vyberieme náhodne
číslo z $[1;n]$ pre ľubovoľné $n \geq 1$?\footnote{Pozri si niečo na tému
'Binárne vyhľadávanie' ak by ťa táto otázka nadchla.}

## Funkcie

Funkcie sú koncept, ktorý pôvodne pochádza z matematiky (v skutočnosti, infor\_matika\_, ako disciplína, existuje ani nie 100 rokov). Napr. funkcia $f$ so zápisom $f(x) := -\frac{1}{2}x + 10$ sa dá zapísať v Python-e veľmi podobne:

```python
def f(x):
    return -1/2 * x + 10
```

Teda príkaz `def` vytvorí funkciu s ľubovoľným názvom, napr. `f` a takáto funkcia príjme jeden parameter, teda `x`. Potom príkaz `return` *vráti* naspäť výslednú hodnotu. Napr. potom vyvolanie\footnote{Toto funguje takisto v matematike, no zvykneme hovoriť, že funkčná hodnota pri $x=20$ je $0$. Princíp je zhodný.} `f(20)` *vráti* hodnotu $0$. Takúto hodnotu si môžeme napr. vložiť do premennej: `vysledok = f(20)`. Ukážme si ešte jeden príklad, ktorý je viac vhodný pre informatikov: 

```python
def formatuj_vysledok(meno, vek):
    return f"{meno} ({vek})"

formatuj_vysledok("Anička", 17) # "Anička (17)"
formatuj_vysledok("Peter", 19)  # "Peter (19)"
```

**1)** Vytvor funkciu `maximum`, ktorá pre dve čísla **vyberie** číslo s vyššou
hodnotou. Funkciu vytvor sám, teda nepouži funkciu `max` zo štandardnej
knižnice.

```python
print(maximum(10,20)) # 20
print(maximum(4,2))   # 4
print(maximum(0,0))   # 0
```

**2)** Vytvor funkciu `faktorial`, ktorá pre dané $n \geq 0$ **vypočíta**
faktoriál $n$, teda $\text{faktorial}(n) = n \cdot (n-1) \cdot \dots \cdot 2
\cdot 1$. Napr. $\text{faktorial}(4) = 4 \cdot 3 \cdot 2 \cdot 1 = 24$.

```python
print(faktorial(0)) # 1 (vynimka)
print(faktorial(1)) # 1 = 1
print(faktorial(2)) # 2 = 2*1
print(faktorial(3)) # 6 = 3*2*1
# ...
print(faktorial(6)) # 720 = 6*5*4*3*2*1
```

**3)** Uvažuj, že základné aritmetické operácie $+,-,*,\div$ sú definované cez funkcie
`plus`, `minus`, `krat`, `deleno` v novom prog. jazyku *SuperJazyk*, na ktorom
práve pracuješ. Každá funkcia prijíma dve numerické hodnoty a vráti naspäť ich
výsledok. Tvojou úlohou je prepísať nasledovné výrazy do tvojho prog.
jazyka\footnote{Nezabudni, že napr. $*$ má prednosť pred $+$}:

1. $(1 + 9) * 4 * 2$ 
2. $10 \div 2 * 4 + 1$
3. $10 + (1 + 2 + 4 * 2) \div 4$

**Príklad**

Výraz $1 + 2 * 3$ možno prepísať ako: `plus(1,krat(2,3))`. Všímni si, že $*$ má
prednosť, a tak výsledok $2*3$ pripočítame k $1$, teda počítame $1 + (2 * 3)$. Teda

$$
    \text{krat}(2,3) = 2 \cdot 3 = 6 \overset{\text{vlož do } \star}{\longmapsto} \text{plus}(1,\star) \longmapsto 1 + 6 = 7.
$$

Takýto prepis je možno na prvý pohľad zvláštny, no takto cca. fungujú tieto
operácie v ozajstných prog. jazykoch, ako napr. Python.

**4)** Učiteľ ťa povolal na pomoc so školským systémom pre žiakov. Potrebuje vytvoriť
funkciu, ktorá **zistí** (resp. vypíše), či je heslo dostatočne bezpečné.
Dohodnete sa, že heslo je bezpečné, pokiaľ sa skladá z (i) aspoň 10 symbolov
(spolu) a zároveň (ii) obsahuje aspoň 2 číslice.

```python
je_ok_heslo('Michal123')        # 'Chyba, heslo nie je bezpecne!'
je_ok_heslo('MichalMichal123')  # 'OK'
je_ok_heslo('velmi_dlhe_heslo') # 'Chyba, heslo nie je bezpecne!'
```

Pre (ii) si môžeš pomôcť funkciou, ktorú ti učiteľ už predpripravil (prekopíruj
si ju do kódu).

```python
def je_cislica(c):
    try:
        return (isinstance(c, int) and 0 <= int(c) <= 9) or 48 <= ord(c) <= 57
    except Exception:
        return False

je_cislica(2)    # True
je_cislica(80)   # False
je_cislica('1')  # True
je_cislica('9')  # True
je_cislica('10') # False
je_cislica('a')  # False
```

**5) BONUS**

Fukcie sú jednoducho *super*! Sú to, podobne ako matematike, objekty s ktorými
môžeme ľubovoľne narábať. V matematike vieme napr. funkcie použiť pri rôznych
dôkazoch. Toto je taktiež (v istom slova zmysle) možné i v programovaní.

V úlohách (1)--(3) sme definovali fukcie, ktoré sú *čísté* (teda fungujú doslova
ako funkcie v matematika)\footnote{Pre každý vstup dostaneme vždy ten istý
výstup; `faktorial(2)` bude vždy jednoducho $2$. Každá takáto funkcia je čistá,
pretože nepoužívame nijaké informácie z iných častí programu.}. Určite si si
všimol, že musí platiť pre ľubovoľné $a \geq b \geq 0$ nerovnica

$$a! \geq b!$$

Inými slovami, `faktorial` **nemení nerovnosť**\footnote{Matematicky by to vyzeralo napr. takto: $a! \geq b! \iff a \geq b \geq 0$.}. Vytvor funkciu
`vlastnost_nerovnost_faktorial`, ktorá otestuje túto vlastnosť pomocou funkcií
definovaných v (1)--(3). Šablóna funkcie bude vyzerať nejako takto:

```python
def vlastnost_nerovnost_faktorial(a, b):
    # 1. Porovnaj a, b,
    # 2. aplikuj `faktorial`,
    # 3. porovnaj, či nerovnosť stále platí.
    # Pri každom kroku vypíš *niečo* do konzoly.
```

Otestuj túto funkciu napr. s náhodnými hodnotami, ktoré si vytvoríš pomocou `random.randint`. Test zopakuj aspoň $10$-krát. Platí táto vlastnosť?
