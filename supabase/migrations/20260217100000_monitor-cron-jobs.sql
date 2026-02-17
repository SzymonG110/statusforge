-- Alternatywne rozwiązanie: użycie Supabase Edge Function zamiast bezpośredniego HTTP
-- Jeśli pg_net nie jest dostępne, użyj tego podejścia

CREATE EXTENSION IF NOT EXISTS pg_cron WITH SCHEMA pg_catalog;

CREATE TABLE IF NOT EXISTS app_settings (
  key TEXT PRIMARY KEY,
  value TEXT NOT NULL,
  updated_at TIMESTAMPTZ NOT NULL DEFAULT NOW()
);

CREATE OR REPLACE FUNCTION run_monitor_checks_via_edge_function()
RETURNS void
LANGUAGE plpgsql
SECURITY DEFINER
AS $$
DECLARE
  monitor_record RECORD;
  supabase_url TEXT;
  supabase_secret_key TEXT;
  function_url TEXT;
  payload JSONB;
BEGIN
  SELECT value INTO supabase_url FROM app_settings WHERE key = 'supabase_url';
  SELECT value INTO supabase_secret_key FROM app_settings WHERE key = 'supabase_secret_key';
  
  IF supabase_url IS NULL OR supabase_url = '' THEN
    RAISE EXCEPTION 'supabase_url must be set in app_settings table. Use: INSERT INTO app_settings (key, value) VALUES (''supabase_url'', ''https://your-project.supabase.co'') ON CONFLICT (key) DO UPDATE SET value = EXCLUDED.value;';
  END IF;
  
  IF supabase_secret_key IS NULL OR supabase_secret_key = '' THEN
    RAISE EXCEPTION 'supabase_secret_key must be set in app_settings table. Use: INSERT INTO app_settings (key, value) VALUES (''supabase_secret_key'', ''your-secret-key'') ON CONFLICT (key) DO UPDATE SET value = EXCLUDED.value;';
  END IF;

  FOR monitor_record IN
    SELECT m.id::text, m.project_id::text, m.name, m.kind, m.url, m.keyword, m.interval_seconds
    FROM monitors m
    WHERE m.enabled = true
  LOOP
    function_url := supabase_url || '/functions/v1/monitor-check';
    
    payload := jsonb_build_object(
      'monitor_id', monitor_record.id,
      'project_id', monitor_record.project_id,
      'name', monitor_record.name,
      'kind', monitor_record.kind,
      'url', monitor_record.url,
      'keyword', monitor_record.keyword,
      'region', 'EU'
    );
    
    -- Użyj pg_net jeśli dostępne, w przeciwnym razie użyj http extension
    BEGIN
      PERFORM net.http_post(
        url := function_url,
        headers := jsonb_build_object(
          'Content-Type', 'application/json',
          'Authorization', 'Bearer ' || supabase_secret_key
        ),
        body := payload
      );
    EXCEPTION
      WHEN OTHERS THEN
        -- Fallback do http extension jeśli pg_net niedostępne
        PERFORM http_post(
          function_url,
          payload::text,
          'application/json'
        );
    END;
  END LOOP;
END;
$$;

SELECT cron.schedule(
  'monitor-checks-every-5min',
  '*/5 * * * *',
  'SELECT run_monitor_checks_via_edge_function();'
);

COMMENT ON FUNCTION run_monitor_checks_via_edge_function() IS 'Wywołuje Edge Function monitor-check dla wszystkich aktywnych monitorów';
