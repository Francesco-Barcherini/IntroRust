# IntroRust
Progetto relativo all'esame "Introduzione alla programmazione in Rust", Prof. Luca Abeni, a.a. 2022/2023 - Scuola Superiore Sant'Anna

## Funzionamento generale
Questo progetto implementa un'applicazione client-server per giocare a "Rock, Paper, Scissors" (sasso, carta, forbici) utilizzando la comunicazione TCP. L'applicazione consente agli utenti di creare o effettuare il login a un account sul server e di giocare contro il server stesso.

Il progetto è composto da due componenti principali: il server (server.rs) e il client (client.rs). Il server è responsabile della gestione degli account degli utenti e dell'esecuzione delle partite, mentre il client consente agli utenti di interagire con il server per creare account, effettuare il login, e giocare.

Il flusso di lavoro è il seguente:
* Il server viene avviato e si mette in ascolto sulla porta 8080.
* I client si connettono al server e possono scegliere di creare un nuovo account o effettuare il login con un account esistente (comando `login <username> <password>`).
* Dopo aver effettuato il login, i client possono giocare contro il server in una partita in cui si vince a `n` punti (comando `play <n>`).
  * A ogni turno il client scrive la sua giocata `<rock|paper|scissors|r|p|s>`, il server fa la propria randomicamente e comunica al client il risultato secondo le regole del sasso-carta-forbici
  * Alla fine del turno il server invia al client anche lo stato della partita (i punti di entrambi i giocatori) e l'eventuale esito finale se uno dei due ha raggiunto il numero di punti necessario per la vittoria della partita.
* Con il comando `quit` il client termina la partita rimanendo connesso, con il comando `logout` il client termina la partita e chiude la connessione.

## Implementazione in Rust
### Server (server.rs)
Il server accetta connessioni TCP e crea un thread per ogni client connesso. Le connessioni sono gestite tramite la libreria standard di Rust, in particolare utilizzando `std::net::TcpListener` e `std::net::TcpStream`. Nel main del server si esegue il binding del listener sulla porta 8080 con il metodo `bind`. Dopodiché viene inizializzato il vettore di account `accounts`; poiché il vettore sarà condiviso tra i thread, viene incapsulato tramite l'utilizzo degli smart pointer in un `Arc<Mutex<Vec<Account>>>` per garantire la condivisione e l'accesso esclusivo ai dati.
Infine il metodo `incoming` crea un iteratore di connessioni che vengono gestite in un ciclo for: per ogni connessione viene creato un clone del vettore di account e passato al thread che gestirà la connessione.

Il gioco è implementato utilizzando le seguenti strutture dati:
* Game: una struttura per tenere traccia dei punteggi di una partita.
* Account: una struttura che rappresenta un account utente, contenente nome utente, password, stato di login e informazioni sulla partita in corso.
* Vec<Account>: il vettore di account `accounts`, condiviso tra i thread e protetto da un meccanismo di locking.

In loop il thread riceve i comandi con il metodo `read` di `TcpStream` e li processa. Al login il thread salva l'id (cioè l'indice nel vettore) dell'account connesso e lo utilizza per accedere ai dati dell'account. Il thread gestisce anche la partita, ricevendo le mosse del client e calcolando il risultato, aggiornando i punteggi e inviando i risultati al client. 

I metodi della `std::net` restituiscono sempre un risultato di tipo `Result`, che può essere `Ok` o `Err`. La gestione degli errori avviene principalmente attraverso l'uso del costrutto `match`.

### Client (client.rs)
Il client consente agli utenti di interagire con il server tramite la console. La connessione al server avviene tramite il metodo `TcpStream::connect`, che restituisce un `Result` contenente un `Ok` con il socket oppure un `Err` in caso di errore. Il client legge i comandi da tastiera e li invia al server tramite il metodo `write` di `TcpStream`. Il client riceve le risposte dal server tramite il metodo `read` di `TcpStream` e le visualizza a schermo.

## Test del progetto
Per compilare aprire il terminale nella cartella del progetto ed eseguire il comando `make`. Dopodiché avviare il server con `./server` e il client in un nuovo terminale con `./client`. Il client può essere avviato più volte per simulare più utenti che giocano contemporaneamente. 
Seguendo le indicazioni nel terminale, creare un account digitando `login <user> <pass>`. Si inizia a giocare con il comando `play <n>`, dopodiché si può digitare `r`, `p`, `s`, `rock`, `paper`, `scissors` per giocare un turno, `quit` per terminare la partita e rimanere connessi, `logout` per terminare la partita e chiudere la connessione.
Nel frattempo il server stampa alcune informazioni di log sulla console.
