# Tabaccheria · Gestione del negozio

Una web app per gestire una piccola tabaccheria di paese. È fatta in HTML, CSS e JavaScript puro, senza installazioni: si apre `index.html` nel browser (meglio Chrome o Edge sul notebook del negozio). Una volta aperta funziona anche senza connessione.

## Cosa fa

- **Oggi · Cassa**: i reparti a pulsanti (tabacchi, gratta e vinci, lotto, ricariche, pagamenti e servizi, fotocopie e stampe, accendini e cartine, batterie, cancelleria, patatine e caramelle, sale, cartoline, altro, vincite pagate). Si compone il conto, si sceglie il pagamento (contanti con calcolo del resto, carta o libretto), si segna se la merce è stata *consegnata fuori*. Ogni vendita scala da sola le scorte. Accanto ci sono i promemoria del giorno e il riepilogo della giornata.
- **Tabacchi**: si cerca nel listino ufficiale (sigarette, stick da riscaldare come Terea, trinciati, sigari…) e si aggiungono le marche che si tengono. Per ognuna ci sono scorta al banco, scorta nel distributore 24 ore, minimo, obiettivo e multiplo d'ordine (10 = a stecche). Con «Conta il distributore» si registrano le vendite della macchinetta, con «Ricarica il distributore» si sposta merce dal banco.
- **Ordine settimanale**: propone le quantità per tornare all'obiettivo, si può modificare, stampare o copiare come testo. Si segna come inviato (il lunedì) e quando arriva la consegna (il mercoledì) si registrano i pezzi arrivati, che si aggiungono al banco.
- **Altri articoli**: batterie AA, AAA e a bottone, sale, cancelleria, patatine, caramelle, cartoline… con prezzo, costo, scorta e minimo, e una lista stampabile di ciò che va riordinato.
- **Gratta e vinci**: i pacchetti ricevuti, l'attivazione, i biglietti venduti e quelli rimasti.
- **Libretto crediti**: chi segna e paga dopo, con il saldo, i pagamenti e un promemoria via WhatsApp.
- **Registro e conti**: vendite per periodo, per reparto e per pagamento, contanti attesi in cassa, guadagno stimato con gli aggi, i tabacchi più venduti ed esportazione CSV per Excel.
- **Impostazioni**: nome, orari (9:30–13:00 e 16:30–20:00), giorni di chiusura, ferie (con lo stato «Aperto / Chiuso / In ferie» in alto), giorni di ordine e consegna, prezzi delle copie, aggi, copia di sicurezza e aggiornamento del listino.

## Listino dei tabacchi

`listino.js` contiene il listino completo in vigore dal 14 ottobre 2026 (4.343 prodotti), ricavato dal file «Completo» scaricato da [tabaccai.it](https://www.tabaccai.it/). Quando cambiano i prezzi si scarica il nuovo file `.xls` («Completo» o «Modificato») e lo si carica da *Tabacchi → Aggiorna il listino da file…*: l'app mostra i prezzi cambiati, i prodotti nuovi e quelli radiati prima di applicarli. Per leggere il file `.xls` l'app scarica una sola volta la libreria SheetJS da cdnjs, quindi in quel momento serve la connessione.

## I dati

I dati restano nel browser del computer (localStorage). Non vanno su internet, ma si perdono se si cancellano i dati di navigazione o si cambia computer. Bisogna quindi fare spesso la **copia di sicurezza** (Impostazioni → Scarica la copia): l'app la ricorda dopo 7 giorni.

## Da verificare con il titolare

Gli aggi preimpostati (tabacchi 10%, gratta e vinci 8%, lotto 8%, ricariche 3%), i prezzi di fotocopie e stampe, i giorni di chiusura e gli articoli di esempio sono solo un punto di partenza. Vanno corretti nelle Impostazioni e in Altri articoli.
