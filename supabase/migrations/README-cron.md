# Migracja Cron Jobs dla Monitorów

## Opis

Ta migracja tworzy automatyczne checki monitorów co 5 minut używając Supabase Cron Jobs (`pg_cron`).

## Co robi migracja

1. **Włącza rozszerzenia:**
   - `pg_cron` - do planowania zadań cron (instalowane w schemacie `pg_catalog`)
   - `pg_net` - do wykonywania żądań HTTP do backendu

2. **Tworzy funkcję `run_monitor_checks()`:**
   - Pobiera wszystkie aktywne monitory (`enabled = true`)
   - Dla każdego monitora wywołuje endpoint backendu: `POST /monitors/:id/check?region=EU`
   - Używa `net.http_post()` do wykonania żądania HTTP

3. **Planuje cron job:**
   - Nazwa: `monitor-checks-every-5min`
   - Harmonogram: `*/5 * * * *` (co 5 minut)
   - Wykonuje funkcję `run_monitor_checks()`

## Wymagania

### 1. Ustawienie konfiguracji

Migracja tworzy tabelę `app_settings` do przechowywania konfiguracji. Przed uruchomieniem migracji ustaw wartości w tej tabeli (możesz to zrobić po zastosowaniu migracji):

```sql
INSERT INTO app_settings (key, value) VALUES 
  ('supabase_url', 'https://your-project.supabase.co'),
  ('supabase_secret_key', 'your-secret-key')
ON CONFLICT (key) DO UPDATE SET value = EXCLUDED.value;
```

**Uwaga:** Migracja używa tabeli `app_settings` zamiast parametrów bazy danych (`ALTER DATABASE`), ponieważ ustawianie parametrów wymaga uprawnień administratora.

### 2. Włączenie rozszerzeń w Supabase

**pg_cron:**
- W Supabase Dashboard: **Database** → **Extensions** → Włącz `pg_cron`
- Lub automatycznie przez migrację SQL (instalowane w schemacie `pg_catalog`)

**pg_net:**
- `pg_net` może nie być widoczne w liście rozszerzeń w Dashboard
- Migracja próbuje zainstalować `pg_net` automatycznie przez SQL
- Jeśli instalacja się nie powiedzie, możesz:
  1. Sprawdzić czy jest dostępne w Twoim planie Supabase (może wymagać wyższego planu)
  2. Użyć alternatywnego rozwiązania (patrz sekcja "Alternatywne rozwiązanie" poniżej)

## Zastosowanie migracji

### Sprawdzenie dostępności pg_net

Najpierw sprawdź czy `pg_net` jest dostępne w Twoim projekcie:

```sql
SELECT * FROM pg_available_extensions WHERE name = 'pg_net';
```

Jeśli zwraca wynik, możesz użyć głównej migracji. Jeśli nie, użyj alternatywnego rozwiązania poniżej.

### Opcja 1: Przez Supabase Dashboard (SQL Editor) - Główna migracja

1. Otwórz projekt w [Supabase Dashboard](https://app.supabase.com)
2. Przejdź do **SQL Editor**
3. Najpierw ustaw konfigurację w tabeli `app_settings`:
   ```sql
   INSERT INTO app_settings (key, value) VALUES 
     ('supabase_url', 'https://your-project.supabase.co'),
     ('supabase_secret_key', 'your-secret-key')
   ON CONFLICT (key) DO UPDATE SET value = EXCLUDED.value;
   ```
4. Skopiuj zawartość pliku `20260217100000_monitor-cron-jobs.sql`
5. Wklej do edytora SQL
6. Kliknij **Run**

**Jeśli pojawi się błąd o brakującym `pg_net`:**
- Sprawdź czy Twój plan Supabase wspiera `pg_net` (może wymagać wyższego planu)
- Użyj alternatywnego rozwiązania poniżej

### Opcja 2: Przez Supabase CLI

```bash
cd /Users/szymon/Desktop/log-app/statusforge-backend
supabase db push
```

## Weryfikacja

Sprawdź czy cron job został utworzony:

```sql
SELECT * FROM cron.job WHERE jobname = 'monitor-checks-every-5min';
```

Sprawdź historię wykonania:

```sql
SELECT * FROM cron.job_run_details 
WHERE jobid = (SELECT jobid FROM cron.job WHERE jobname = 'monitor-checks-every-5min')
ORDER BY start_time DESC 
LIMIT 10;
```

## Zarządzanie cron job

### Zatrzymanie cron job

```sql
SELECT cron.unschedule('monitor-checks-every-5min');
```

### Zmiana harmonogramu

```sql
-- Najpierw usuń stary
SELECT cron.unschedule('monitor-checks-every-5min');

-- Dodaj nowy z innym harmonogramem (np. co 1 minutę)
SELECT cron.schedule(
  'monitor-checks-every-1min',
  '* * * * *',
  'SELECT run_monitor_checks();'
);
```

### Aktualizacja konfiguracji

```sql
UPDATE app_settings SET value = 'https://new-project.supabase.co' WHERE key = 'supabase_url';
UPDATE app_settings SET value = 'new-secret-key' WHERE key = 'supabase_secret_key';
```

## Uwagi

- Cron job wykonuje się co 5 minut dla **wszystkich** aktywnych monitorów
- Każdy monitor jest sprawdzany z regionem `EU` (domyślnie)
- Jeśli chcesz sprawdzać różne regiony, możesz zmodyfikować funkcję `run_monitor_checks()` aby iterować po regionach
- Jeśli backend nie jest dostępny, żądania HTTP będą kończyć się błędem, ale cron job będzie kontynuował działanie

## Alternatywne rozwiązanie (jeśli pg_net niedostępne)

Jeśli `pg_net` nie jest dostępne w Twoim planie Supabase, możesz użyć alternatywnej migracji która wywołuje Edge Function zamiast bezpośredniego HTTP:

**Plik:** `20260217100000_monitor-cron-jobs-alternative.sql`

Ta migracja:
- Wywołuje Supabase Edge Function `monitor-check` zamiast bezpośredniego HTTP do backendu
- Edge Function następnie wykonuje check i zapisuje wynik do bazy
- Wymaga ustawienia `app.supabase_url` i `app.supabase_secret_key` zamiast `app.backend_url`

**Ustawienia:**

Migracja tworzy tabelę `app_settings` do przechowywania konfiguracji. Ustaw wartości:

```sql
INSERT INTO app_settings (key, value) VALUES 
  ('supabase_url', 'https://your-project.supabase.co'),
  ('supabase_secret_key', 'your-secret-key')
ON CONFLICT (key) DO UPDATE SET value = EXCLUDED.value;
```

**Uwaga:** Używamy `secret_key` (service role key) zamiast `anon_key`, ponieważ Edge Function może wymagać wyższych uprawnień do zapisu wyników do bazy danych.

## Alternatywne rozwiązanie (jeśli pg_cron niedostępne)

Jeśli `pg_cron` nie jest dostępne w Twoim planie Supabase, możesz użyć:

1. **GitHub Actions** - workflow uruchamiany co 5 minut
2. **Vercel Cron** - jeśli backend jest na Vercel
3. **Zewnętrzny scheduler** - który wywołuje endpoint `/monitors/:id/check` dla każdego aktywnego monitora

Przykład z GitHub Actions:

```yaml
name: Monitor Checks
on:
  schedule:
    - cron: '*/5 * * * *'
jobs:
  check-monitors:
    runs-on: ubuntu-latest
    steps:
      - name: Check monitors
        run: |
          # Pobierz listę aktywnych monitorów z API
          # Wywołaj POST /monitors/:id/check dla każdego
```
