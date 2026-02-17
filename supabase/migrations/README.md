# Migracje bazy danych StatusForge

## Jak zastosować migrację w Supabase

### Opcja 1: Przez Supabase Dashboard (SQL Editor)

1. Otwórz projekt w [Supabase Dashboard](https://app.supabase.com)
2. Przejdź do **SQL Editor** (lewe menu)
3. Skopiuj zawartość pliku `001_initial_schema.sql`
4. Wklej do edytora SQL
5. Kliknij **Run** (lub Ctrl+Enter)

### Opcja 2: Przez Supabase CLI

```bash
# Zainstaluj Supabase CLI (macOS)
brew install supabase/tap/supabase

# Lub przez inne metody: https://github.com/supabase/cli#install-the-cli
# Windows: scoop bucket add supabase https://github.com/supabase/scoop-bucket.git
#         scoop install supabase
# Linux: curl -fsSL https://supabase.com/install.sh | sh

# Zaloguj się
supabase login

# Połącz z projektem (project-ref znajdziesz w URL projektu: app.supabase.com/project/[project-ref])
supabase link --project-ref twoj-project-ref

# Zastosuj migrację
supabase db push
```

**Uwaga:** Supabase CLI nie może być instalowane przez `npm install -g`. Użyj Homebrew (macOS) lub innych wspieranych metod z [oficjalnej dokumentacji](https://github.com/supabase/cli#install-the-cli).

## Co zawiera migracja

### Tabele
- **organizations** - organizacje (multi-tenant)
- **organization_members** - członkowie organizacji (role: owner/admin/member)
- **projects** - projekty należące do organizacji
- **logs** - logi aplikacji (level, message, context JSON, trace_id, source, environment)
- **monitors** - monitory uptime (HTTP/HTTPS/SSL/keyword)
- **monitor_results** - wyniki checków (region EU/US/ASIA, response_time, status)
- **status_pages** - publiczne status pages
- **webhooks** - webhooki per projekt
- **webhook_logs** - historia dostarczeń webhooków
- **alerts** - placeholder dla alertów (email/Discord/Slack/SMS)

### Funkcje
- `create_default_organization()` - automatycznie tworzy organizację "Osobista" dla nowych użytkowników
- `update_updated_at_column()` - automatycznie aktualizuje `updated_at` przy zmianach

### RLS (Row Level Security)
- Pełna izolacja multi-tenant: użytkownicy widzą tylko swoje organizacje/projekty
- Publiczny dostęp tylko do enabled status pages
- Service role może wstawiać logi i wyniki monitorów (dla cron/Edge Functions)

### Indexy
- Zoptymalizowane pod kątem zapytań: po organization_id, project_id, created_at, level, status

## Weryfikacja

Po zastosowaniu migracji sprawdź w Supabase Dashboard:
1. **Table Editor** - powinny być widoczne wszystkie tabele
2. **Authentication** - utwórz testowego użytkownika, sprawdź czy automatycznie powstała organizacja "Osobista"
3. **SQL Editor** - uruchom: `SELECT * FROM organizations;` - powinna być organizacja dla testowego użytkownika
