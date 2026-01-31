# Bier-App Uitvoeringsplan 🍻

Dit project wordt een platform voor bierclubs en evenementen. Dit plan is opgesteld zodat jij de code kunt schrijven en kunt leren hoe alles in elkaar steekt.

## Fase 1: Fundament & Navigatie
- [-] **Routing Opzetten**: Implementeer `dioxus-router` voor de hoofdnavigatie (Home, Clubs, Events, Profile). (Uitgesteld voor DB setup)
- [ ] **Tab Bar**: Maak een navigatiebalk die onderaan het scherm blijft staan (typisch voor mobiele apps).
- [ ] **Thema**: Update `main.css` met een echt "bier-thema" (amber, donkerbruin, goud, schuim-wit).

- [-] **Backend Setup**: Initialiseer een Rust backend (bijv. Axum of Actix-web) met een PostgreSQL database.
  - [x] Database installatie & setup (`setup_db.sh`)
  - [x] Database schema ontwerp (`implementation_plan.md`)
  - [x] Migraties uitvoeren (`run_migrations.sh`)
  - [x] Rust dependencies toevoegen (`sqlx`, `tokio`, etc.)
  - [x] Database connectie opzetten in Rust
- [ ] **Registratie & Login API**: Maak endpoints voor `/auth/register` en `/auth/login`.
- [ ] **Frontend Auth State**: Gebruik een Dioxus `GlobalSignal` om bij te houden of een gebruiker is ingelogd.
- [x] **Login Scherm koppelen**: Verbind je huidige login-scherm met de echte backend API. (UI Implementatie Gereed)

## Fase 3: Clubs & Leden
- [ ] **Database Schema**: Voeg de `Clubs` en `ClubMemberships` tabellen toe.
- [ ] **Clubs Overzicht**: Bouw de pagina die alle beschikbare clubs ophaalt (`GET /clubs`).
- [ ] **Club Detail Pagina**: Toon informatie over een specifieke club, inclusief de "Join" knop.
- [ ] **Club Aanmaken**: Implementeer het formulier om zelf een club te starten.

## Fase 4: Realtime Chat (WebSockets)
- [ ] **WebSocket Server**: Zet een WebSocket server op in de backend die berichten per club-room kan broadcasten.
- [ ] **Chat UI**: Bouw de chat-interface in Dioxus (messages list + input field).
- [ ] **Berichtengeschiedenis**: Haal de laatste 50 berichten op via REST voordat de WebSocket verbinding start.
- [ ] **Live Berichten**: Verbind de frontend met de WebSocket om berichten direct te ontvangen en te versturen.

## Fase 5: Evenementen & RSVP
- [ ] **Events Schema**: Voeg de `Events` en `EventAttendees` tabellen toe aan je DB.
- [ ] **Event Feed**: Maak de tijdlijn op de homepage met aankomende bierevenementen.
- [ ] **RSVP Systeem**: Zorg dat gebruikers kunnen aangeven of ze "Gaan", "Geïnteresseerd" zijn of "Niet gaan".
- [ ] **Event Creatie**: Laat club-eigenaren evenementen toevoegen voor hun club.

## Fase 6: Profiel & Beheer
- [ ] **Mijn Profiel**: Toon de clubs waarvan de gebruiker lid is en hun geplande evenementen.
- [ ] **Club Beheer**: Bouw het paneel voor club-eigenaren (leden verwijderen, rollen aanpassen).
- [ ] **Afwerking**: Voeg bier-gerelateerde graphics en animaties toe.

---

### Tips voor het leren:
1. **Begin klein**: Start met de Routing (Fase 1) zodat je app als een echte app aanvoelt.
2. **Backend**: Je kunt Dioxus `server functions` gebruiken of een aparte backend repo maken. Aangezien je PostgreSQL nodig hebt, is een aparte backend vaak overzichtelijker.
3. **Vragen**: Als je vastloopt bij een specifieke taak, vraag me dan om uitleg over de concepten (zoals "Hoe werkt een GlobalSignal?") in plaats van om de volledige code.
