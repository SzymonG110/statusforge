# StatusForge Backend – lista zadań

## Fundament
- [x] Przenieść `ApiError` i wspólne typy do modułu `shared` (lub `error`)
- [x] Dodać konfigurację (env / config): port, Supabase URL, klucze
- [x] Połączenie z bazą (Supabase/Postgres) – moduł `shared/db.rs`
- [x] Wszystkie ID w systemie: UUID v4 (używać `uuid` crate)

## Struktura feature-first
- [ ] Wydzielić `lib.rs`, przenieść router do lib
- [ ] Dodać katalog `src/features/` i `src/shared/`
- [ ] Feature **auth**: `mod`, `route`, `service`, `model` (integracja z Supabase Auth / JWT)
- [ ] Feature **organizations**: `mod`, `route`, `service`, `model`, `repository`
- [ ] Feature **projects**: `mod`, `route`, `service`, `model`, `repository`
- [ ] Feature **monitors**: `mod`, `route`, `service`, `model`
- [ ] Feature **ingest**: `mod`, `route`, `service` (API do przyjmowania logów)
- [ ] Shared: `mod.rs`, `db.rs`, `utils.rs`

## Organizacje
- [ ] CRUD organizacji (UUID v4)
- [ ] Domyślna organizacja „osobista” przy tworzeniu użytkownika
- [ ] Zaproszenia i członkostwo – tylko członkowie widzą dane organizacji

## Projekty
- [ ] CRUD projektów w obrębie organizacji (UUID v4)
- [ ] Izolacja: projekty widoczne tylko dla członków organizacji

## Logi / ingest
- [ ] Endpoint do wysyłania logów do projektu (API key / auth)
- [ ] Model logu: level, message, context (JSON), trace_id (opcjonalnie), source, environment
- [ ] Zapis do DB, UUID v4 per log
- [ ] Endpoint do listowania/filtrowania logów (z paginacją)

## Monitory / checki
- [ ] Model monitora i wyniku checka (UUID v4)
- [ ] Typy: HTTP/HTTPS, SSL, keyword monitoring
- [ ] Pola wyniku: region, status, response_time, HTTP status, SSL validity, error message
- [ ] Multi-location (EU / US / ASIA) – zapis regionu w wynikach
- [ ] (Opcjonalnie w backendzie) Cron/scheduler co 5 min lub integracja z Supabase Edge Functions

## Status pages
- [ ] Endpoint read-only do danych status page po `project_slug` (publiczny)
- [ ] UUID v4 dla status page

## Webhooki
- [ ] CRUD webhooków per projekt (UUID v4)
- [ ] Wysyłanie przy zdarzeniach: downtime, recovery, error spike
- [ ] Tabela / model `webhook_logs` – historia dostarczeń (UUID v4)

## Multi-tenant i RLS
- [ ] Zapytania do DB z uwzględnieniem `organization_id` / `project_id`
- [ ] RLS w Supabase – backend używa klienta z odpowiednim kontekstem (JWT/service role) tak aby RLS egzekwował izolację

## Alerty (placeholder)
- [ ] Model / miejsce na triggery alertów (mail, Discord, Slack, SMS) – do implementacji później

## Jakość i DevOps
- [ ] Testy jednostkowe dla service/repository
- [ ] Health check rozszerzony (np. sprawdzenie połączenia z DB)
- [ ] Logowanie (tracing / log)
- [ ] CORS skonfigurowane pod frontend
