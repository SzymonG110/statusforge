# Supabase Edge Functions dla StatusForge

Ten katalog zawiera Edge Functions używane przez backend do wykonywania checków monitorów.

## monitor-check

Edge Function do wykonywania checków monitorów HTTP/HTTPS/SSL/Keyword.

### Wymagane zmienne środowiskowe

Brak - funkcja używa publicznych API.

### Użycie

Funkcja jest automatycznie wywoływana przez backend Rust poprzez endpoint `POST /monitors/:id/check`.

Można też wywołać ręcznie:

```bash
curl -X POST https://YOUR_PROJECT.supabase.co/functions/v1/monitor-check \
  -H "Authorization: Bearer YOUR_ANON_KEY" \
  -H "Content-Type: application/json" \
  -d '{
    "monitor_id": "uuid",
    "project_id": "uuid",
    "name": "Example Monitor",
    "kind": "http",
    "url": "https://example.com",
    "region": "EU"
  }'
```

### Request Body

```typescript
{
  monitor_id: string
  project_id: string
  name: string
  kind: "http" | "https" | "ssl" | "keyword"
  url: string
  keyword?: string | null  // Wymagane dla kind="keyword"
  region: "EU" | "US" | "ASIA"
}
```

### Response

```typescript
{
  region: "EU" | "US" | "ASIA"
  status: "up" | "down" | "degraded"
  response_time_ms?: number
  http_status?: number
  ssl_valid?: boolean
  ssl_expires_at?: string
  error_message?: string
}
```

### Typy checków

- **http/https**: Sprawdza dostępność endpointu HTTP, zwraca status code i czas odpowiedzi
- **ssl**: Sprawdza ważność certyfikatu SSL (używa SSL Labs API)
- **keyword**: Sprawdza czy określone słowo kluczowe występuje w odpowiedzi

### Deploy

**WAŻNE:** Uruchom komendę z głównego katalogu projektu (`/Users/szymon/Desktop/log-app`), **NIE** z katalogu `supabase`:

```bash
cd /Users/szymon/Desktop/log-app
supabase functions deploy monitor-check
```

Jeśli jesteś w katalogu `supabase`, CLI będzie szukać funkcji w `supabase/supabase/functions/` (podwójne `supabase`), co spowoduje błąd.

### Deploy bezpośrednio do chmury (bez lokalnego środowiska)

Jeśli nie masz uruchomionego lokalnego środowiska Supabase, możesz zdeployować funkcję bezpośrednio:

```bash
cd /Users/szymon/Desktop/log-app
supabase functions deploy monitor-check
```

**Uwaga:** Musisz być zalogowany (`supabase login`) i mieć połączony projekt (`supabase link --project-ref YOUR_PROJECT_REF`).

### Lokalne testowanie (wymaga Supabase lokalnie)

Jeśli chcesz testować funkcję lokalnie, musisz najpierw zainicjalizować i uruchomić lokalne środowisko:

```bash
cd /Users/szymon/Desktop/log-app

# Inicjalizacja projektu Supabase (tylko raz)
supabase init

# Uruchomienie lokalnego środowiska Supabase
supabase start

# W osobnym terminalu - uruchomienie funkcji lokalnie
supabase functions serve monitor-check
```

Następnie wywołaj funkcję lokalnie:

```bash
curl -X POST http://localhost:54321/functions/v1/monitor-check \
  -H "Authorization: Bearer YOUR_ANON_KEY" \
  -H "Content-Type: application/json" \
  -d '{
    "monitor_id": "test-id",
    "project_id": "test-project-id",
    "name": "Test Monitor",
    "kind": "http",
    "url": "https://example.com",
    "region": "EU"
  }'
```

**Uwaga:** Lokalne środowisko Supabase wymaga Docker. Jeśli nie masz Docker, użyj opcji deploy bezpośrednio do chmury.

## Integracja z Cron

Aby automatycznie wykonywać checki co 5 minut, użyj Supabase Cron Jobs lub zewnętrznego schedulera (np. GitHub Actions, Vercel Cron) który wywołuje endpoint `POST /monitors/:id/check` dla wszystkich aktywnych monitorów.

Przykład cron job w Supabase (SQL):

```sql
SELECT cron.schedule(
  'monitor-checks',
  '*/5 * * * *',
  $$
  SELECT net.http_post(
    url := 'https://YOUR_BACKEND_URL/monitors/' || id || '/check?region=EU',
    headers := '{"Content-Type": "application/json"}'::jsonb
  )
  FROM monitors
  WHERE enabled = true
  $$
);
```
