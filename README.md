# Create your own syntesizer with Rust

The goal of this workshop is to learn a bit about Rust, a bit about sound, and hopefully end up with a playable syntesizer

## Build and run the project
Build: `cargo build`
Run: `cargo run`

# Tasks

## 1. Create a simple oscillator
An oscillator is a component that generates a periodic signal of a given frequency. There are different kinds of oscillators, but the most common are sine, sawtooth, square and triangle. Feel free to look these up.

In this task we are creating a simple oscillator. You are free to choose wich kind of periodic signal to use, but keep in mind that some signals sounds more dull than others. (Looking at you, sine wave).

Start by cloning the repo and and familiarize yourself with the code.

The whole synthesizer will be implemented within the closure function `synth` located in `./main.rs`. This function will be called by the audioprocessing thread for each new sample to be generated.

The function takes one argument,`action`, that is an Option of `i32` which is a number from 0 to 15. This value corresponds to key currently being pressed. The `synth` function returns a value representing the oscillators output signal. _We will not need to worry about this argument until task three._

<details>
<summary>Hint</summary>

A sine oscillation wave can be expressed by the following.

y(t) = A * sin(2 &#960; &#402; t &#43; &phi;), where A, &#402;, and &phi; are constant parameters.

A = amplitude\
&#402; = ordinary frequency. Try `440Hz`\
&phi; = phase

Sinusiod function is explained in detail [here](https://en.wikipedia.org/wiki/Sine_wave).
Phase are explained in detail [here](https://en.wikipedia.org/wiki/Phase_(waves)#Formula_for_phase_of_an_oscillation_or_a_wave).

You're highly encouraged to implement another type of oscillating wave:
- [Square wave](https://en.wikipedia.org/wiki/Square_wave)
- [Triangle wave](https://en.wikipedia.org/wiki/Triangle_wave)
- [Sawtooth wave](https://en.wikipedia.org/wiki/Sawtooth_wave)
</details>

## 2. Visualizing the oscillator
Before we proceed with making a complete synthesizer, we wish to implement a graphical representation of the soundwaves we generate.

### Some theory: Communication between threads
Since real time audio comes with some demands, the processing is done in a  thread separate from the UI-thread. Therefore, we need to communicate the generated sound wave to the UI-thread where it will be rendered on the screen.

In many languages the communication between threads are primarily done by limiting resource access, often by the use of mutexes. This limits the access to reading and modifying a resource to one thread at a time, making all other threads await their turn. Applications using this strategy often have problems with data races, deadlocks and starvation, and debugging these can be a challenge.

In addition to mutexes, Rust supports communication between threads via 
channels. Sending a signal through a channel will not block the sender, and only some calls will block the reciever. 

Bellow, we see an example of how to create channel that sends data of the type `MyChannelType`.

```rust
use std::sync::mpsc::channel;
(my_sender, my_receiver) = channel::<MyChannelType>();
```

Remember that when using channels, one is still bound by the ownership rules of Rust. By sending a variable down a channel, you also give up the ownership of it.

### Task
In this task you are going to send some data of the type `GraphEvent` (as defined in `./types.rs`) to the UI-thread. You must create a channel of this type, and pass the sender and reciever to `setup_synth()` and the UI-object respectivly. You must also find out how to create an instance of `GraphEvent`


I denne oppgaven skal du sende data av typen `GraphEvent` (definert i `./types.rs`) til UI-tråden. Du må opprette en kanal som har elementer
av denne typen, og gi sender og mottaker til henholdsvis `setup_synth()` og UI-objektet. Du må selv finne ut hvordan du skal opprette
objekter av GraphEvent-typen, og hvordan å sende disse over kanalen.


<details>
<summary>Hint</summary>

The data points in `GraphEvent` are held in a queue og type `VecDeque<f64>`.
</details>

## 3. Create the keyboard

### The realation between tones and frequencies
`ISO 16` defines the musical note of `A above middle C` to have a frequency of _440.0 Hz_, and from this frequency, all other notes can be derived.     

Playing an octave higher is the same as doubling the frequency played for any given tone. `440.0 * 2.0 = 880.0 Hz` is one octave up and `880.0 * 2.0 = 1760.0 Hz` is two octaves up from the `ISO 16-A`

There are twelve notes in one octave, and due to the exponential nature of notes there is a number `x` that describes the relationship between them. Since we know that moving one octave up means doubling the frequency, we can use this to find `x`:

```
1.0 * x * x * x * x * x * x * x * x * x * x * x * x = 2.0
x^12 = 2.0
x = 12th_root(2.0)
x = 1.05946309436
```


### Task
In this task we implement a function that takes the value of the `action` parameter which correspons to the key being pressed, and returns the frequency of the note to be played. If implemented correctly, you should be able to change the sound by pressing the two top letter rows on your keyboard.



<details>
<summary>Hint</summary>

</details>

## 4. Implement an amplifier 
Til nå har vi laget en oscillator som genererer lydbølger for oss, samt et keyboard som gjør oss i stand til å styre frekvensen til oscillatoren
vha tastaturet. Det eneste som gjenstår nå for at programmet vårt skal kunne kalles en synthesizer, er at man også skal kunne skru av oscillatoren
når ingen knapper på tastaturet er trykket ned.

### Task
In this task we are creating an amplifier for our synth that will adjust the 
volume of the input signal from the oscillator in accordance with an input parameter `gain`. The formula will then be `output = input * gain`.


Deretter skal du koble denne forsterkeren på en slik måte at den justerer volumet på output fra oscillatoren, etter `gate`-verdi fra kyboard.


<details>
<summary>Hint</summary>

// Skriv hint her
</details>

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


<details>
<summary>Hint</summary>

// Skriv hint her
</details>

## 6. The rest of the f\*cking owl
Now that you have a working synthesizer, you are free to develop it even further if you wish. Some suggestions:

- Filters (low pass, band pass and high pass)
- Delay
- Reverb
- Flanging
- Portamento
