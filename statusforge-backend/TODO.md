# StatusForge Backend – lista zadań

## Fundament
- [x] Przenieść `ApiError` i wspólne typy do modułu `shared` (lub `error`)
- [x] Dodać konfigurację (env / config): port, Supabase URL, klucze
- [x] Klient Supabase – moduł `shared/supabase.rs` (supabase-lib-rs)
- [x] Wszystkie ID w systemie: UUID v4 (używać `uuid` crate)

## Struktura feature-first
- [x] Wydzielić `lib.rs`, przenieść router do lib
- [x] Katalog `src/shared/`: `mod.rs`, `config.rs`, `error.rs`, `supabase.rs`, `utils.rs`
- [x] Dodać katalog `src/features/`
- [x] Feature **auth**: `mod`, `route`, `service`, `model` (integracja z Supabase Auth / JWT – do implementacji)
- [x] Feature **organizations**: `mod`, `route`, `service`, `model`, `repository`
- [x] Feature **projects**: `mod`, `route`, `service`, `model`, `repository`
- [x] Feature **monitors**: `mod`, `route`, `service`, `model`
- [x] Feature **ingest**: `mod`, `route`, `service` (API do przyjmowania logów – do implementacji)
- [x] Shared: `utils.rs` (m.in. `uuid_v4()`)

## Organizacje
- [x] CRUD organizacji (UUID v4) - GET /organizations, POST /organizations, PUT /organizations/:id, DELETE /organizations/:id
- [x] Domyślna organizacja „osobista” przy tworzeniu użytkownika - trigger w migracji SQL
- [x] Zaproszenia i członkostwo – tylko członkowie widzą dane organizacji - RLS policies w migracji

## Projekty
- [x] CRUD projektów w obrębie organizacji (UUID v4) - GET /organizations/:org_id/projects, POST /organizations/:org_id/projects, GET /projects/:id, PUT /projects/:id, DELETE /projects/:id
- [x] Izolacja: projekty widoczne tylko dla członków organizacji - RLS policies w migracji SQL

## Logi / ingest
- [x] Endpoint do wysyłania logów do projektu - POST /projects/:project_id/logs
- [x] Model logu: level, message, context (JSON), trace_id (opcjonalnie), source, environment
- [x] Zapis do DB, UUID v4 per log - automatycznie przez gen_random_uuid()
- [x] Endpoint do listowania/filtrowania logów (z paginacją) - GET /projects/:project_id/logs?level=...&trace_id=...&limit=...&offset=...

## Monitory / checki
- [x] Model monitora i wyniku checka (UUID v4) - `Monitor`, `MonitorResult` w `features/monitors/model.rs`
- [x] Typy: HTTP/HTTPS, SSL, keyword monitoring - walidacja w service layer
- [x] Pola wyniku: region, status, response_time, HTTP status, SSL validity, error message - wszystkie pola w `MonitorResult`
- [x] Multi-location (EU / US / ASIA) – zapis regionu w wynikach - walidacja regionów w service
- [x] CRUD monitorów - GET/POST /projects/:project_id/monitors, GET/PUT/DELETE /monitors/:id
- [x] Tworzenie i listowanie wyników - POST/GET /monitors/:monitor_id/results z filtrowaniem po region/status
- [x] Testy jednostkowe - walidacja w service layer i deserializacja modeli
- [x] Integracja z Supabase Edge Functions - funkcja `monitor-check` do wykonywania checków
- [x] Endpoint `POST /monitors/:id/check` do ręcznego wywołania checka
- [x] Edge Function `monitor-check` obsługująca typy: HTTP/HTTPS, SSL, keyword monitoring
- [x] Deploy Edge Function do Supabase (funkcja wdrożona i dostępna)
- [x] Cron/scheduler co 5 min - migracja SQL z `pg_cron` i funkcją `run_monitor_checks()` wywołującą endpoint backendu

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
