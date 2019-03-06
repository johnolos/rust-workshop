# Lag din egen syntesizer i Rust

Formålet med workshopen er å lære litt om Rust, litt om lyd og forhåpenligvis ende opp med en syntesizer man kan bruke.

## Bygge og kjøre prosjektet
Bygge: `cargo build`
Kjøre: `cargo run`

_ Samt støtteverktøy hvis du installerte dette fra smoketesten _:
Linting: `cargo clippy`
Formattering: `cargo fmt`
Auto-fiks: `cargo fix`


# Oppgavene

## 1. Lag en enkel oscillator
En oscillator er en komponent som genererer et periodisk signal av en gitt frekvens. Det finnes ulike typer oscillatorer, men de mest brukte
er av typene sinus, sagtann, firkant og trekant. Slå disse gjerne opp på nettet for å se hvordan disse signalene ser ut.

I denne oppgaven skal du lage en enkel oscillator. Du kan selv velge hvilket periodisk signal som skal brukes for oscillatoren, men tenk på
at ikke alle oscillatorer høres like "spennende" ut (looking at you, sine wave).

Begynn med å klone `rust-workshop`, og forsøk å gjøre deg litt kjent med kildekoden.

Hele synthesizeren din skal defineres inne i closure-funksjonen `synth` i `./main.rs`. Denne funksjonen skal kalles av audioprosesseringstråden
for hver nye sample som skal genereres, og funksjonen tar imot tre argumenter: funksjonens kjøretid, `t`; tid siden sist gang funksjonen ble kalt,
`dt`, samt `action`, som inneholder en `KeyAction` dersom en knapp på tastaturet er trykket ned. Funksjonen skal returnere et flyttall
som representerer oscillatorens utgangssignal.

## 2. Visualisering av oscillatoren
Før vi går videre med å lage en fullverdig synthesizer, ønsker vi å ha på plass en grafisk representasjon av lydbølgene vi genererer.

### Litt teori: Kommunikasjon mellom tråder
På grunn av strenge krav til behandlingstid for sanntidsaudio, kjører selve lydprosesseringsfunksjonen på en egen tråd adskilt fra
UI-tråden. Vi blir derfor nødt å sende lydbølgen vi genererte i oppgave 1 over til UI-tråden, hvor den vil bli tegnet på skjermen.

I mange språk foregår kommunikasjon mellom tråder i hovedsak vha begrensning av ressurs-tilgang, ofte implementert ved å bruke
en låsetype kalt mutex (Mutual Exclusion). Dette vil si at man kun gir en tråd tilgang til å lese og modifisere en ressurs av gangen,
og alle andre tråder må vente på tur. Applikasjoner som benytter seg av slike låser er ofte utsatt for problemer som data race,
deadlock og starvation, og det er ofte vanskelig å luke ut disse problemene.

I tillegg til mutexer, har Rust også støtte for kommunikasjon mellom tråder via kanaler. Å sende et signal via en kanal vil ikke blokkere
senderen av signalet, noe som gjør at vi kan være mer trygg på at koden vår ikke kan forårsake data race -- mottakeren er den eneste
som potensielt kan bli blokkert, men da kun ved visse kall (f.eks. vil receiver.recv() blokkere, mens receiver.try_recv() ikke).

Nedenfor kan en se et eksempel for hvordan å opprette en kanal som sender data av typen `MyChannelType` over en kanal.

```rust
use std::sync::mpsc::channel;
(my_sender, my_receiver) = channel::<MyChannelType>();
```

Husk på at ved å bruke kanaler, så må man fortsatt forholde seg til Rust sine strenge krav til eierskap: Ved å sende en variabel over en kanal,
gir man også fra seg eierskapet til variabelen.

### Oppgave
I denne oppgaven skal du sende data av typen `GraphEvent` (definert i `./types.rs`) til UI-tråden. Du må opprette en kanal som har elementer
av denne typen, og gi sender og mottaker til henholdsvis `setup_synth()` og UI-objektet. Du må selv finne ut hvordan du skal opprette
objekter av GraphEvent-typen, og hvordan å sende disse over kanalen.

Et hint: Datapunktene i `GraphEvent` er holdt i en datakø av typen `VecDeque<f64>`.


## 3. Lag et `Keyboard`

### Sammenheng mellom toner og frekvenser
`ISO 16` definerer at en `enstrøken A` skal ha en frekvens på _440.0 Hz_, og ut ifra denne frekvensen kan alle andre noter defineres.
Å spille samme tone en oktav over, betyr i praksis en dobling av frekvens, uansett hvilken grunntone. Dette betyr altså at en `tostrøken A`
har en frekvens på `440.0 * 2.0 = 880.0 Hz`, og en `trestrøken A` en frekvens på `880.0 * 2.0 = 1760.0 Hz`.

Det er tolv halvtoner i en oktav, og på grunn toner sin eksponensielle natur, betyr dette at det finnes et tall, `x`, som beskriver forholdet
mellom alle halvtoner. Siden vi vet at å flytte en tone opp en oktav betyr en dobling av frekvens, kan vi bruke dette forholdet til å finne `x`:

```
1.0 * x * x * x * x * x * x * x * x * x * x * x * x = 2.0
x^12 = 2.0
x = 12th_root(2.0)
x = 1.05946309436
```

### Oppgave
I denne oppgaven skal du lage en komponent som tar inn en heltallsverdi (hvilken knapp som er trykket ned, øker mot høyre på tastaturet),
og gir ut hvilken frekvens oscillatoren skal spille av.

`synth(...)`-funksjonen hvor du implementerte oscillatoren i oppgave 1, tar inn et argument for hvilken knapp du har trykket på. Dette argumentet
er av typen `KeyAction`, som er definert i `./audioengine/src/types.rs` -- ta en titt på denne typen for å finne ut hvordan den skal brukes.

## 4. Implementere en forsterker og fullføre en minimal synthesizer
Til nå har vi laget en oscillator som genererer lydbølger for oss, samt et keyboard som gjør oss i stand til å styre frekvensen til oscillatoren
vha tastaturet. Det eneste som gjenstår nå for at programmet vårt skal kunne kalles en synthesizer, er at man også skal kunne skru av oscillatoren
når ingen knapper på tastaturet er trykket ned.

### Oppgave
Denne oppgaven går ut på å lage en forsterker-modul til vår synthesizer, som skal justere volumet på input-signalet etter en en gitt parameter,
`gain`. Formelen som skal brukes for dette, er `output = input * gain`.

Deretter skal du koble denne forsterkeren på en slik måte at den justerer volumet på output fra oscillatoren, etter `gate`-verdi fra kyboard.


## 5. Envelope v/ADSR
Vår synth er nå i stand til å spille av lyd når man trykker på tastaturet, og å være stille når man slipper knappene igjen, men det kan
argumenteres for at den fortsatt høres litt mer ut som en justerbar summetone enn et instrument, siden den mangler punch. Dette skal du fikse
i denne oppgaven, ved å implementere en såkalt ADSR-envelope, som skal kobles mellom gate og forsterker.

### ADSR
En ADSR (Attack, Decay, Sustain, Release) transformerer en gate-input til et mer dynamisk signal. For å utføre dette inneholder den en intern
tilstandsmaskin som har tilstandende `Attack`, `Decay`, `Sustain`, `Release` og `Off`, og hvor hendelsesforløpet er som følger:
0. (ADSR er i tilstand `Off`, med output = 0.0)
1. `gate` går fra 0.0 til 1.0. ADSR går til tilstand `Attack`.
2. ADSR øker output med en konfigurerbar verdi `self.attack` hver gang `process()` blir kjørt.
3. Output når 1.0. ADSR går til tilstand `Decay`.
4. Output minker med en konfigurerbar verdi `self.decay` hver gang `process()` blir kjørt.
5. Output når konfigurerbar verdi `self.sustain`. ADSR går til tilstand Sustain.
6. Output holder verdien `self.sustain` for hver gang `process()` kalles, så lenge `gate` holdes på 1.0.
7. `gate` går fra 1.0 til 0.0. ADSR går til tilstand `Release`.
8. Output minker med en konfigurerbar verdi `self.release` hver gang `process()` blir kjørt.
9. Output når 0.0. ADSR går til tilstand `Off`.

### Vising av slidere i UI-et
UI-et vi har satt opp for denne workshopen er i stand til å vise slidere som man kan bruke for å justere på parametre. Du står fri til å
velge hvor mange slidere du ønsker, hvilken range de skal ha, default-verdi og hvilken tekst som skal stå under slideren.

For å vise slidere i UI-et, må du definere opp et array av `Slider`-e og sende dem inn som parameter i `Ui::new(...)`. Ta en titt i
`./types.rs` for å se hvordan man kan lage instanser av denne typen.

Parameteren til `Ui::new(...)` for slidere har signatur `Option<&[Slider]>`, så her må du pakke inn slider-arrayet ditt i en `Some`.

Dersom du nå prøver å kjøre programmet, vil du kunne se at dine slidere blir tegnet på skjermen, men det er foreløpig ikke koblet opp
noen logikk til disse.

På samme måte som du brukte kanaler for å sende lyddata til UI-tråden i oppgave 2, må du her sende sliderdata fra UI-tråden tilbake til
synthesizeren din. Se i parameterlisten til `Ui::new(...)` for å finne ut hvilken type kanalen din må ha.

### Oppgave
I denne oppgaven skal du implemente en ADSR-komponent som skal kobles mellom gate og forsterker. Deretter skal du knytte konfigurerbare
felter for ADSR-en din til slidere i UI-et.

## 6. The rest of the f*cking owl
Nå som du har en fungerende synthesizer, står du fritt til å utvikle synthesizeren videre slik du selv ønsker at den skal høres ut.

Her er noen forslag til videre forbedringer:
- Filtre (lavpass, båndpass, høypass)
- Delay
- Romklang
- Flanger
- Portamento
