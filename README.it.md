# AltKeeper

**AltServer per iOS 17+ che gira sul tuo server di casa (Docker).** Rinnova e installa le app
in stile AltStore / SideStore via Wi-Fi, senza un computer acceso e senza VPN.

[English](README.md)

Tiene in vita le app installate sull'iPhone senza App Store. Le app installate con un **Apple ID
gratuito** smettono di aprirsi dopo 7 giorni, se non si rinnova il loro *profilo di provisioning*.
AltKeeper gira su una piccola macchina sempre accesa in casa e lo fa via Wi-Fi, **senza computer
collegato, senza AltServer su un computer e senza VPN sul telefono**. Può funzionare in due modi,
insieme o separati:

- **Come AltServer per AltStore Classic**: il tasto *Refresh All* di AltStore e l'installazione o
  l'aggiornamento delle app da AltStore funzionano, con questa macchina che risponde al posto di
  AltServer.
- **Da sola**: rinnova i profili ogni sera (cron), anche se AltStore non viene mai aperto.

C'è anche una piccola **pagina web** per vedere lo stato delle app e rinnovarle:

<img src="docs/screenshots/web.png" alt="Pagina web di AltKeeper: iPhone raggiungibile, tre app con la scadenza, pulsanti di rinnovo" width="320">

> **Stato: sperimentale.** Provato su un iPhone 15 Pro (iOS 27.0) con AltStore Classic (l'ultima
> versione al momento della scrittura), con il programma su un Mac e, per il rinnovo senza controllo,
> su un server Debian 13 (x86-64). Usa API private di Apple e il protocollo di AltStore, che possono
> cambiare senza preavviso. Per uso personale con il proprio Apple ID; non affiliato ad Apple né ad
> AltStore. I messaggi del programma e i commenti nel codice sono in italiano.
>
> Confermato (AltServer sul Mac): il *Refresh* da AltStore installa il profilo nuovo sull'iPhone e
> l'installazione di un'app (circa 126 MB) da AltStore funziona. Confermato prima: login, `apps`,
> `renew`, e che un'app si apre anche con i soli profili *nuovi* installati (anche dopo il riavvio
> dell'iPhone). Non ancora confermato: cosa succede il giorno in cui scade il profilo *originale*, e
> AltServer dall'immagine Docker su un server per un periodo lungo.

## Cosa fa e cosa non fa

- Rinnova i profili di provisioning delle tue app (e, in modalità AltServer, installa le app che
  AltStore gli manda). Non rifirma le app e **non crea né revoca certificati**.
- **La modalità AltServer non ha bisogno dell'Apple ID sul server.** AltStore fa il login
  sull'iPhone; questo programma gli dà solo i dati anisette e installa quello che AltStore manda. Il
  rinnovo senza controllo (`renew`, cron, il tasto "Rinnova" della pagina web) invece ha bisogno
  dell'Apple ID e della sua password sul server.
- I due non si pestano i piedi: `renew` guarda il profilo più recente sul telefono. Se AltStore ha
  rinnovato da poco, `renew` non fa niente e non accede nemmeno ad Apple.
- Dopo un rinnovo i profili vecchi restano sul telefono (iOS non li toglie), quindi per ogni app
  vedrai più profili. È normale. In modalità AltServer non toglie mai profili, nemmeno quando
  AltStore lo chiede (`activeProfiles`).
- Non supportato: l'abilitazione del JIT (`EnableUnsignedCodeExecution`); risponde con un errore.
- Dopo un `renew` senza controllo, AltStore può continuare a mostrare la vecchia scadenza: aggiorna
  il suo registro solo quando il rinnovo lo fa lui. Le app funzionano.

## Cosa serve

- Un iPhone con **iOS 17 o successivo**, con la modalità sviluppatore attiva, sullo stesso Wi-Fi
  del server.
- Una macchina sempre accesa nella rete di casa. Provato: Debian 13 (x86-64). Va bene anche un Mac.
- Per la modalità AltServer: **AltStore Classic** sull'iPhone, con il permesso *Rete locale*.
- Per il rinnovo senza controllo: un **Apple ID gratuito** il cui team ha già firmato le app.
- Solo per il **primo pairing**: l'iPhone collegato via USB a un computer (il più facile è un Mac).
- Facoltativi: `curl`, `jq` e l'app [ntfy](https://ntfy.sh) per le notifiche; Docker.

## Configurazione

### 1. Ottieni il programma

```sh
git clone https://github.com/BartolomeoRusso9/altkeeper
cd altkeeper
docker run --rm -v "$PWD":/w -w /w rust:1-bookworm cargo build --release
```

Il binario è `target/release/altkeeper`. Note:

- Su un Mac con Apple Silicon, aggiungi `--platform linux/amd64` per compilare per un server
  x86-64 (più lento: circa 5-7 minuti).
- Se compili anche per il Mac nella stessa cartella, aggiungi `-e CARGO_TARGET_DIR=/w/target-linux`
  così le due compilazioni non si sovrascrivono.
- Su un Mac puoi semplicemente lanciare `cargo build --release`.
- Oppure usa l'immagine Docker (vedi [Docker](#9-docker)).

Sul server crea una cartella (per esempio `/srv/altkeeper`) e copia lì il binario `altkeeper` e i
file di `examples/` che ti servono.

### 2. Trova l'iPhone (facoltativo)

AltKeeper trova l'iPhone da solo e lo ritrova se il suo indirizzo cambia (nelle nostre prove è
cambiato più volte in poche ore). Prova, in ordine: l'indirizzo che indichi tu, l'ultimo che ha
funzionato (`state/phone-addr`), Bonjour (`_remotepairing._tcp`) e infine una scansione della rete
locale cercando la porta del servizio del telefono (49152), che non dipende da Bonjour (su un server
dove un altro programma condivide la porta di Bonjour, le risposte possono andare perse). Puoi quindi
saltare questo passo. Se preferisci indicare l'indirizzo, usa `--phone ip:porta` o `ALTKEEPER_PHONE=ip:porta`
(la porta di solito è 49152). Da un Mac lo trovi così:

```sh
dns-sd -B _remotepairing._tcp                       # elenca le istanze, si ferma con Ctrl-C
dns-sd -L "<nome istanza>" _remotepairing._tcp local    # mostra host e porta (di solito 49152)
dns-sd -G v4 <host>.local                           # mostra l'indirizzo IP
```

### 3. Pairing (una volta sola, via USB)

`pair-usb` ha bisogno dell'`usbmuxd` del computer raggiungibile via TCP. Su un Mac usa il piccolo
ponte in `examples/`:

```sh
# Terminale 1 (lascialo aperto): espone l'usbmuxd del Mac su 127.0.0.1:27015
python3 examples/usbmux-bridge.py

# Terminale 2, con l'iPhone collegato, sbloccato e con "Autorizza" già accettato:
export USBMUXD_SOCKET_ADDRESS=127.0.0.1:27015
./target/release/altkeeper pair-usb
```

Crea `rp-pairing.plist`. Copialo accanto al binario sul server e tienilo riservato (`chmod 600`).
Copiare sul server Debian un pairing fatto con un Mac ha funzionato nelle nostre prove. Su Linux la
stessa idea con `socat` (`socat TCP-LISTEN:27015,bind=127.0.0.1,fork UNIX-CONNECT:/var/run/usbmuxd`)
dovrebbe funzionare, ma non l'abbiamo provata.

Poi controlla il collegamento, dalla cartella del server:

```sh
./altkeeper phone        # elenca i profili installati sul telefono
```

### 4. AltServer per AltStore (consigliato)

```sh
./altkeeper altserver            # oppure: ./altkeeper serve --altserver (con la pagina web)
```

Lascialo acceso (come servizio o in Docker). Sull'iPhone, in AltStore, apri **My Apps** e tocca
**Refresh All**, oppure installa un'app: AltStore trova il server da solo. Ascolta sulla porta
**49500** (`--port N` per cambiarla) e si annuncia via Bonjour come `_altserver._tcp`.

- Sulla rete deve esserci un solo AltServer: ferma quello sul computer e non far girare due copie di
  questo programma.
- Provalo prima con una sola app poco importante, non con *Refresh All* su tutto.
- AltStore mostra *"AltServer could not be found"* per diversi tipi di connessione persa quando il
  server non è quello che preferisce, quindi il messaggio può nascondere la causa vera. Guarda
  l'output del server: registra ogni connessione, richiesta e risposta.
- La prima richiesta crea l'identità anisette in `state/` se non c'è ancora. L'abbiamo usato con
  un'identità creata da `login`; una cartella pulita non è stata provata.

### 5. Accesso (solo per il rinnovo senza controllo e il "Rinnova" della pagina web)

```sh
./altkeeper login tuo.apple.id@example.com
```

Chiede la password (**si vede sullo schermo mentre scrivi**) e il codice 2FA a 6 cifre che arriva
agli altri tuoi dispositivi. Rispondi `s` alla domanda sul salvare la password: è quello che permette
al rinnovo di girare senza di te. Viene salvata **in chiaro** in `account.json` (permessi 600).
Accedi una volta sola; se Apple risponde con errori, aspetta prima di riprovare invece di ripetere.

```sh
./altkeeper apps                                # i tuoi App ID e le scadenze
./altkeeper renew --dry-run --min-days 7        # mostra cosa rinnoverebbe, non cambia niente
./altkeeper renew --min-days 7                  # rinnova ora quello che scade entro 7 giorni
```

`renew` guarda il profilo più recente di ogni app, rinnova quelle con `--min-days` giorni o meno
(default 3) e accede ad Apple **solo se c'è qualcosa da rinnovare**.

### 6. Rinnovo automatico (facoltativo)

Una rete di sicurezza che non dipende dall'iPhone che esegue AltStore.

1. Copia `renew.sh` e `notifica.sh` da `examples/` accanto al binario e fai `chmod +x renew.sh`.
   Lascia `ALTKEEPER_PHONE` vuota in `renew.sh` (il telefono si trova da solo) oppure metti il suo
   `IP:porta`.
2. Installa il cron (modifica prima il percorso): `cp altkeeper.cron /etc/cron.d/altkeeper`

Parte ogni sera alle 21:17, rinnova quello che serve e aggiunge data ed esito a `renew.log`.
L'iPhone deve essere sul Wi-Fi di casa a quell'ora. Con profili da 7 giorni e soglia a 3 hai diverse
sere di margine prima che un'app scada.

### 7. Notifiche (facoltativo)

Crea `notify.conf` accanto agli script, con un nome di topic lungo e casuale:

```sh
echo 'NTFY_TOPIC=scegli-un-nome-lungo-e-casuale' > notify.conf && chmod 600 notify.conf
```

Installa l'app ntfy sul telefono e iscriviti a quel topic. Ricevi un messaggio quando le app vengono
rinnovate e quando un giro fallisce (al massimo un messaggio di errore ogni 24 ore, perché il
telefono può essere semplicemente fuori casa). Se non c'è niente da rinnovare non arriva nulla.
Chiunque conosca il topic può leggerlo: tienilo segreto.

### 8. Pagina web (facoltativa)

```sh
ALTKEEPER_WEB_PIN=123456 ./altkeeper serve --bind 0.0.0.0:8787 --altserver
```

Apri `http://<server>:8787` (nome utente qualsiasi, il PIN come password). Mostra se l'iPhone è
raggiungibile e le app con i giorni che mancano, permette di rinnovarle ("Rinnova quelle in scadenza"
o "Rinnova tutte adesso", con il log in diretta), di accedere con l'Apple ID (il codice 2FA si scrive
nel browser) e di vedere gli ultimi rinnovi da `renew.log`. Senza `--bind`, o con un indirizzo di
loopback, ascolta solo sulla macchina stessa e non chiede il PIN; su qualsiasi altro indirizzo
**rifiuta di partire senza PIN** (almeno 4 caratteri). Usa la variabile d'ambiente e non `--pin`, che
si vede con `ps`. La pagina è in italiano e non c'è HTTPS: tienila dentro la rete di casa. L'accesso
e il rinnovo dalla pagina web non sono ancora stati provati con Apple.

### 9. Docker

```sh
docker build --platform linux/amd64 -t altkeeper:latest .
```

L'immagine non contiene niente di segreto: pairing, account e stato stanno nel volume `/data`. Copia
`examples/docker-compose.yml` in `/srv/altkeeper/docker-compose.yml`, metti nella stessa cartella il
tuo `rp-pairing.plist` (e `account.json` e `state/` se usi `renew`), scrivi
`ALTKEEPER_WEB_PIN=...` in `/srv/altkeeper/.env` (permessi 600) e lancia `docker compose up -d`.
Usa la rete dell'host, perché serve Bonjour per trovare l'iPhone e per farsi trovare da AltStore; le
porte 8787 (web) e 49500 (AltServer) devono essere libere.

Un workflow di GitHub Actions (`.github/workflows/docker.yml`) lancia i test e pubblica l'immagine su
`ghcr.io/<utente>/altkeeper` a ogni push su `main` (amd64) e sui tag `vX.Y.Z` (amd64 e arm64).
Non è ancora stato eseguito su GitHub. Se il repository è privato lo è anche il pacchetto.

## Comandi

| Comando | Cosa fa |
| --- | --- |
| `phone [--phone ip:porta]` | elenca i profili installati sull'iPhone |
| `pair-usb [file]` | crea il pairing remoto via USB (una volta) |
| `login <apple-id>` | accede con 2FA e salva l'account |
| `apps` | elenca i tuoi App ID e le scadenze |
| `renew [--dry-run] [--force] [--min-days N] [--phone ip:porta]` | rinnova i profili in scadenza |
| `altserver [--port N] [--dir cartella] [--phone ip:porta]` | fa da AltServer per AltStore |
| `serve [--bind ip:porta] [--pin PIN] [--dir cartella] [--phone ip:porta] [--altserver] [--altserver-port N]` | pagina web (e, con `--altserver`, AltServer) |
| `profile-remove <uuid>` | toglie un profilo dal telefono, salvandone una copia in `profili-salvati/` |
| `profile-restore <file>` | rimette un profilo salvato |

`ALTKEEPER_PHONE=ip:porta` può sostituire `--phone`; senza nessuno dei due il telefono si trova da
solo (vedi il passo 2). `ALTKEEPER_NO_MDNS=1` spegne Bonjour (reti che bloccano il multicast, o per
provare la scansione). `ALTKEEPER_DEBUG=1` stampa cosa risponde Apple a ogni richiesta del login (stato,
intestazioni, corpo degli errori; mai il corpo delle richieste né le risposte riuscite).
`ALTKEEPER_ANISETTE_URL=https://...` usa un server anisette tuo (per esempio `anisette-v3-server`) al
posto di quello predefinito; deve iniziare con `http://` o `https://`.

Il progetto si chiamava altrefresh: i vecchi nomi `ALTREFRESH_*` funzionano ancora (se ci sono
entrambi vince `ALTKEEPER_*`). `altkeeper --version` stampa la versione.

## Se qualcosa va storto

- **"manca il pairing remoto"**: nella cartella non c'è `rp-pairing.plist`. Rifai il pairing
  (passo 3).
- **"pairing con l'iPhone non riuscito" / `early eof`**: l'iPhone non riconosce più quel pairing.
  Rifallo, oppure copia un `rp-pairing.plist` funzionante da un altro computer.
- **"non trovo l'iPhone in rete", "No route to host", timeout**: l'iPhone non è in rete (in standby,
  fuori casa). Se avevi indicato un indirizzo che non è più giusto, il programma cerca il telefono da
  solo; se vuoi un indirizzo fisso, sull'iPhone apri Impostazioni, Wi-Fi, la (i) della tua rete e
  imposta l'indirizzo Wi-Fi privato su **Fisso** o disattivato (la dicitura dipende dalla versione di
  iOS), poi dai all'iPhone un IP fisso nel router.
- **AltStore dice "AltServer could not be found"**: vedi il passo 4. Controlla che il server sia
  acceso, sulla stessa rete, che AltStore abbia il permesso Rete locale, e leggi l'output del server.
- **Errori 429 o 503 al login**: Apple sta limitando le richieste. Non riprovare a ripetizione.
  Lancia con `ALTKEEPER_DEBUG=1` e, se persiste, apri una issue con le righe che iniziano con
  `[gs #` (non contengono segreti).
- **Un'app non si apre dopo un rinnovo**: reinstallala con AltStore o con lo strumento che usi di
  solito. Se avevi tolto un profilo con `profile-remove`, rimettilo con `profile-restore`.

## Sicurezza

Questi file contengono segreti. Sono ignorati da git e vanno tenuti con permessi `600`:

- `rp-pairing.plist`: le chiavi del pairing con il telefono.
- `account.json`: l'Apple ID e, se hai scelto di salvarla, la password **in chiaro**.
- `state/`: l'identità anisette del "dispositivo" e il `serverID` di AltServer.
- `notify.conf`: il topic di ntfy.
- `profili-salvati/`: le copie dei profili tolti dal telefono con `profile-remove`.

Per segnalare una vulnerabilità in privato vedi [SECURITY.md](SECURITY.md).

Non incollare mai la password, `account.json` o il file del pairing in una issue o in una chat.
Non usare `RUST_LOG=debug` su log condivisi: a quel livello `idevice` stampa le chiavi del pairing.
Tieni la macchina che li conserva dentro la rete di casa.

**La porta di AltServer (49500) non ha un accesso, come il vero AltServer**: chiunque sulla tua rete
parli il protocollo può chiedere i dati anisette o mandare profili e app all'iPhone accoppiato. Non
esporre quella porta fuori dalla rete di casa.

## Come funziona

1. L'iPhone (iOS 17+) annuncia in rete il servizio Bonjour `_remotepairing._tcp`.
2. Con un pairing remoto già creato, il programma fa il *pair-verify*, chiede al telefono di aprire
   un tunnel e lo raggiunge con TLS-PSK ([`idevice`](https://github.com/jkcoxson/idevice)).
3. Dentro il tunnel legge e installa i profili con `misagent`, e installa o toglie le app con
   `installation_proxy`.
4. In modalità AltServer, AltStore trova la macchina con Bonjour (`_altserver._tcp`, con un record
   `serverID`) e manda richieste JSON su TCP (la lunghezza a 32 bit little-endian, poi il JSON), una
   connessione per operazione: dati anisette, profili da installare o togliere, e l'app firmata.
5. Per il rinnovo senza controllo, con l'Apple ID ([`isideload`](https://github.com/nab138/isideload))
   scarica i profili nuovi per gli App ID esistenti.

Il vecchio Wi-Fi sync (`netmuxd`, AltServer-Linux) su iOS recenti viene rifiutato dal telefono: il
pairing remoto è il canale che funziona.

## Note sulla copia di `isideload` in `vendor/`

`isideload` 0.3.17 ha due problemi con i server di login di Apple, quindi `vendor/isideload` è
una copia modificata dell'originale (MIT, © nab138):

- **503 al login.** Si presenta come `com.apple.dt.Xcode`, che Apple rifiuta.
  `src/anisette/remote_v3/mod.rs` ora dichiara `com.apple.akd/1.0` (identità `Mac15,7`,
  macOS 27.0) e l'User-Agent AuthKit che usa AltSign.
- **429 alla prova della password.** Il bordo di Apple lascia passare poche richieste su una
  stessa connessione e rifiuta le altre. Il login ne manda tre di fila (URL bag, `init`,
  `complete`), quindi la terza prendeva 429. AltSign lo evita con una connessione nuova per
  ogni richiesta (`ALTAppleAPI+Authentication.swift`, ramo notarized); `src/auth/grandslam.rs`
  ora fa lo stesso (solo HTTP/1.1, niente riuso delle connessioni, `Connection: close`). Su
  macOS usa anche il TLS di sistema. Quale di queste modifiche sia decisiva non è stato isolato.

Espone anche alcuni accessori in sola lettura di `AnisetteData`, usati per rispondere ad AltStore.

## Crediti e licenza

Costruito su [`idevice`](https://github.com/jkcoxson/idevice) (Jackson Coxson) e
[`isideload`](https://github.com/nab138/isideload) (nab138), entrambi con licenza MIT. La
correzione del login segue quello che fa [AltSign](https://github.com/rileytestut/AltSign)
(Riley Testut), e i messaggi di AltServer seguono il protocollo di
[AltStore](https://github.com/altstoreio/AltStore). Licenza di questo progetto: [MIT](LICENSE).
