import { serve } from "https://deno.land/std@0.168.0/http/server.ts"

interface MonitorCheckRequest {
  monitor_id: string
  project_id: string
  name: string
  kind: "http" | "https" | "ssl" | "keyword"
  url: string
  keyword?: string | null
  region: "EU" | "US" | "ASIA"
}

interface MonitorCheckResponse {
  region: "EU" | "US" | "ASIA"
  status: "up" | "down" | "degraded"
  response_time_ms?: number
  http_status?: number
  ssl_valid?: boolean
  ssl_expires_at?: string
  error_message?: string
}

async function checkHttp(url: string, region: string): Promise<MonitorCheckResponse> {
  const startTime = Date.now()
  try {
    const response = await fetch(url, {
      method: "GET",
      headers: {
        "User-Agent": "StatusForge-Monitor/1.0",
      },
      signal: AbortSignal.timeout(10000),
    })
    const responseTime = Date.now() - startTime

    let status: "up" | "down" | "degraded" = "up"
    if (response.status >= 500) {
      status = "down"
    } else if (response.status >= 400 || responseTime > 5000) {
      status = "degraded"
    }

    return {
      region: region as "EU" | "US" | "ASIA",
      status,
      response_time_ms: responseTime,
      http_status: response.status,
    }
  } catch (error) {
    return {
      region: region as "EU" | "US" | "ASIA",
      status: "down",
      error_message: error instanceof Error ? error.message : String(error),
    }
  }
}

async function checkSsl(url: string, region: string): Promise<MonitorCheckResponse> {
  try {
    const urlObj = new URL(url)
    const hostname = urlObj.hostname
    const port = urlObj.port || (urlObj.protocol === "https:" ? 443 : 80)

    const response = await fetch(`https://api.ssllabs.com/api/v3/analyze?host=${hostname}&publish=off&fromCache=on&maxAge=1`)
    const data = await response.json()

    if (data.status === "READY" && data.endpoints && data.endpoints.length > 0) {
      const endpoint = data.endpoints[0]
      const sslValid = endpoint.grade && endpoint.grade !== "F"
      const sslExpiresAt = endpoint.details?.cert?.notAfter

      return {
        region: region as "EU" | "US" | "ASIA",
        status: sslValid ? "up" : "down",
        ssl_valid: sslValid,
        ssl_expires_at: sslExpiresAt,
        error_message: sslValid ? undefined : "SSL certificate is invalid or expired",
      }
    }

    return {
      region: region as "EU" | "US" | "ASIA",
      status: "degraded",
      error_message: "SSL Labs API did not return valid data",
    }
  } catch (error) {
    return {
      region: region as "EU" | "US" | "ASIA",
      status: "down",
      error_message: error instanceof Error ? error.message : String(error),
    }
  }
}

async function checkKeyword(url: string, keyword: string, region: string): Promise<MonitorCheckResponse> {
  const startTime = Date.now()
  try {
    const response = await fetch(url, {
      method: "GET",
      headers: {
        "User-Agent": "StatusForge-Monitor/1.0",
      },
      signal: AbortSignal.timeout(10000),
    })
    const responseTime = Date.now() - startTime
    const body = await response.text()
    const keywordFound = body.includes(keyword)

    let status: "up" | "down" | "degraded" = keywordFound ? "up" : "down"
    if (response.status >= 500) {
      status = "down"
    } else if (response.status >= 400 || responseTime > 5000) {
      status = "degraded"
    }

    return {
      region: region as "EU" | "US" | "ASIA",
      status,
      response_time_ms: responseTime,
      http_status: response.status,
      error_message: keywordFound ? undefined : `Keyword "${keyword}" not found in response`,
    }
  } catch (error) {
    return {
      region: region as "EU" | "US" | "ASIA",
      status: "down",
      error_message: error instanceof Error ? error.message : String(error),
    }
  }
}

serve(async (req) => {
  if (req.method !== "POST") {
    return new Response(JSON.stringify({ error: "Method not allowed" }), {
      status: 405,
      headers: { "Content-Type": "application/json" },
    })
  }

  try {
    const body: MonitorCheckRequest = await req.json()

    if (!body.monitor_id || !body.url || !body.kind || !body.region) {
      return new Response(
        JSON.stringify({ error: "Missing required fields: monitor_id, url, kind, region" }),
        {
          status: 400,
          headers: { "Content-Type": "application/json" },
        }
      )
    }

    let result: MonitorCheckResponse

    switch (body.kind) {
      case "http":
      case "https":
        result = await checkHttp(body.url, body.region)
        break
      case "ssl":
        result = await checkSsl(body.url, body.region)
        break
      case "keyword":
        if (!body.keyword) {
          return new Response(
            JSON.stringify({ error: "Keyword is required for keyword monitoring" }),
            {
              status: 400,
              headers: { "Content-Type": "application/json" },
            }
          )
        }
        result = await checkKeyword(body.url, body.keyword, body.region)
        break
      default:
        return new Response(
          JSON.stringify({ error: `Invalid monitor kind: ${body.kind}` }),
          {
            status: 400,
            headers: { "Content-Type": "application/json" },
          }
        )
    }

    return new Response(JSON.stringify(result), {
      status: 200,
      headers: { "Content-Type": "application/json" },
    })
  } catch (error) {
    return new Response(
      JSON.stringify({
        error: "Internal server error",
        message: error instanceof Error ? error.message : String(error),
      }),
      {
        status: 500,
        headers: { "Content-Type": "application/json" },
      }
    )
  }
})
