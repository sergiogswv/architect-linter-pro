use crate::config::{AIConfig, AIProvider};
use serde::{Deserialize, Serialize};

// Estructuras para el mapeo de la respuesta de la IA
#[derive(Deserialize, Serialize, Debug, Clone)]
pub struct AISuggestionResponse {
    pub pattern: String,
    pub suggested_max_lines: usize,
    pub rules: Vec<SuggestedRule>,
}

#[derive(Deserialize, Serialize, Debug, Clone)]
pub struct SuggestedRule {
    pub from: String,
    pub to: String,
    pub reason: String,
}

#[derive(Deserialize, Serialize, Debug, Clone)]
pub struct ArchOption {
    pub name: String,
    pub description: String,
}

#[derive(Deserialize, Serialize, Debug)]
pub struct Top3Response {
    pub options: Vec<ArchOption>,
}

/// Obtiene la lista de modelos disponibles para el proveedor configurado
pub async fn obtener_modelos_disponibles(
    provider: &AIProvider,
    api_url: &str,
    api_key: &str,
) -> anyhow::Result<Vec<String>> {
    let client = reqwest::Client::new();
    let url = api_url.trim_end_matches('/');

    match provider {
        AIProvider::Claude => {
            let response = client
                .get(format!("{}/v1/models", url))
                .header("x-api-key", api_key)
                .header("anthropic-version", "2023-06-01")
                .send()
                .await?;

            let json: serde_json::Value = response.json().await?;
            let models = json["data"]
                .as_array()
                .ok_or_else(|| anyhow::anyhow!("Respuesta de Claude inválida"))?
                .iter()
                .filter_map(|m| m["id"].as_str().map(|s| s.to_string()))
                .collect();
            Ok(models)
        }
        AIProvider::Gemini => {
            let url = url.trim_end_matches('/');
            let response = client
                .get(format!("{}/v1beta/models?key={}", url, api_key))
                .header("x-goog-api-key", api_key)
                .send()
                .await?;

            let json: serde_json::Value = response.json().await?;
            let models = json["models"]
                .as_array()
                .ok_or_else(|| anyhow::anyhow!("Respuesta de Gemini inválida"))?
                .iter()
                .filter_map(|m| {
                    m["name"]
                        .as_str()
                        .map(|s| s.trim_start_matches("models/").to_string())
                })
                .collect();
            Ok(models)
        }
        AIProvider::Ollama => {
            let response = client
                .get(format!("{}/api/tags", url))
                .send()
                .await?;

            let json: serde_json::Value = response.json().await?;
            let models = json["models"]
                .as_array()
                .ok_or_else(|| anyhow::anyhow!("Respuesta de Ollama inválida"))?
                .iter()
                .filter_map(|m| m["name"].as_str().map(|s| s.to_string()))
                .collect();
            Ok(models)
        }
        AIProvider::OpenAI
        | AIProvider::Groq
        | AIProvider::Kimi
        | AIProvider::DeepSeek => {
            let mut request = client.get(format!("{}/models", url));
            if !api_key.is_empty() {
                request = request.header("authorization", format!("Bearer {}", api_key));
            }

            let response = request.send().await?;
            let json: serde_json::Value = response.json().await?;
            let models = json["data"]
                .as_array()
                .ok_or_else(|| anyhow::anyhow!("Respuesta de API compatible inválida"))?
                .iter()
                .filter_map(|m| m["id"].as_str().map(|s| s.to_string()))
                .collect();
            Ok(models)
        }
    }
}

/// Función para consultar la IA seleccionada de forma genérica
pub async fn consultar_ia(prompt: String, ai_config: AIConfig) -> anyhow::Result<String> {
    match ai_config.provider {
        AIProvider::Claude => consultar_claude(prompt, ai_config).await,
        AIProvider::Gemini => consultar_gemini(prompt, ai_config).await,
        AIProvider::Ollama => consultar_ollama(prompt, ai_config).await,
        AIProvider::OpenAI | AIProvider::Groq | AIProvider::Kimi => {
            consultar_openai_compatible(prompt, ai_config).await
        }
        AIProvider::DeepSeek => consultar_openai_compatible(prompt, ai_config).await,
    }
}

/// Orquestador que intenta consultar varias IAs en orden hasta que una funcione
pub async fn consultar_ia_con_fallback(
    prompt: String,
    configs: &[AIConfig],
) -> anyhow::Result<String> {
    if configs.is_empty() {
        return Err(anyhow::anyhow!("No hay configuraciones de IA disponibles. Ejecuta el linter sin architect.json para configurar una."));
    }

    let mut last_error = anyhow::anyhow!("Error desconocido");

    for (i, config) in configs.iter().enumerate() {
        if i > 0 {
            println!(
                "\n⚠️  El modelo '{}' falló. Intentando con el siguiente configurado: '{}'...",
                configs[i - 1].name,
                config.name
            );
        }

        match consultar_ia(prompt.clone(), config.clone()).await {
            Ok(res) => {
                if i > 0 {
                    println!("✅ El modelo '{}' respondió correctamente.\n", config.name);
                }
                return Ok(res);
            }
            Err(e) => {
                println!("❌ Fallo el modelo '{}': {}", config.name, e);
                last_error = e;
            }
        }
    }

    Err(anyhow::anyhow!(
        "❌ Todos los modelos configurados fallaron. Último error: {}",
        last_error
    ))
}

/// Función exclusiva para el Linter: Sugiere la arquitectura inicial
pub async fn sugerir_arquitectura_inicial(
    context: crate::discovery::ProjectContext,
    ai_configs: Vec<AIConfig>,
) -> anyhow::Result<AISuggestionResponse> {
    let prompt = format!(
        "Eres un Arquitecto de Software Senior. Analiza este proyecto {framework} con las siguientes dependencias: {deps:?}
        y esta estructura de carpetas: {files:?}.
        Además, estos son los archivos arquitectónicos clave del proyecto: {key_files:?}.

        TAREA:
        Identifica el patrón (Hexagonal, Clean o MVC) y sugiere reglas de importaciones prohibidas basándote en las mejores prácticas.
        Usa los archivos arquitectónicos clave para entender mejor la estructura del proyecto (ej: user.controller.ts, auth_service.py indican convenciones de nomenclatura).

        RESPONDE EXCLUSIVAMENTE EN FORMATO JSON con esta estructura:
        {{
          \"pattern\": \"Nombre del patrón\",
          \"suggested_max_lines\": 60,
          \"rules\": [
            {{ \"from\": \"patrón_origen\", \"to\": \"patrón_prohibido\", \"reason\": \"explicación corta\" }}
          ]
        }}",
        framework = context.framework,
        deps = context.dependencies,
        files = context.folder_structure,
        key_files = context.key_files
    );

    // Obtener respuesta con fallback
    let response_text = consultar_ia_con_fallback(prompt, &ai_configs).await?;

    let clean_json = extraer_json_flexible(&response_text)?;

    // Debug: Mostrar el JSON recibido
    eprintln!("\n🔍 DEBUG - JSON recibido de la IA:");
    eprintln!("━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━");
    eprintln!("{}", clean_json);
    eprintln!("━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━\n");

    // Validar que el JSON esté completo
    if !clean_json.ends_with('}') {
        return Err(anyhow::anyhow!(
            "JSON incompleto recibido de la IA.\n\nJSON parcial:\n{}\n\nPosible causa: La respuesta fue truncada. Intenta con un proyecto más pequeño o simplifica la estructura.",
            clean_json
        ));
    }

    // Intentar parsear con mejor manejo de errores
    let suggestion: AISuggestionResponse = serde_json::from_str(&clean_json)
        .map_err(|e| {
            anyhow::anyhow!(
                "Error parseando JSON de la IA: {}\n\nJSON recibido:\n{}\n\nSugerencia: Revisa el JSON arriba. Si está incompleto, puede ser que el límite de tokens sea insuficiente.",
                e,
                clean_json
            )
        })?;
    Ok(suggestion)
}

/// Sugiere un Top 3 de arquitecturas basadas en el framework detectado
pub async fn sugerir_top_3_arquitecturas(
    framework: &str,
    ai_configs: Vec<AIConfig>,
) -> anyhow::Result<Vec<ArchOption>> {
    let prompt = format!(
        "Eres un Arquitecto de Software Senior. El proyecto usa el framework '{framework}'.
        
        TAREA:
        Sugiere un Top 3 de patrones arquitectónicos ideales para este framework (ej: Hexagonal, Clean, MVC, Layered, Modular Monolith, etc.).
        
        RESPONDE EXCLUSIVAMENTE EN FORMATO JSON con esta estructura:
        {{
          \"options\": [
            {{ \"name\": \"Nombre del patrón\", \"description\": \"Breve explicación de por qué es ideal para {framework}\" }}
          ]
        }}
        
        Asegúrate de que sean exactamente 3 opciones.",
        framework = framework
    );

    let response_text = consultar_ia_con_fallback(prompt, &ai_configs).await?;
    let clean_json = extraer_json_flexible(&response_text)?;

    let response: Top3Response = serde_json::from_str(&clean_json)?;
    Ok(response.options)
}

/// Sugiere reglas específicas para un patrón seleccionado
pub async fn sugerir_reglas_para_patron(
    pattern_name: &str,
    context: crate::discovery::ProjectContext,
    ai_configs: Vec<AIConfig>,
) -> anyhow::Result<AISuggestionResponse> {
    let prompt = format!(
        "Eres un Arquitecto de Software Senior. Se ha seleccionado el patrón '{pattern_name}' para el proyecto {framework}.
        Dependencias: {deps:?}
        Estructura de carpetas: {files:?}
        Archivos clave: {key_files:?}

        TAREA:
        Genera reglas de importaciones prohibidas específicas para implementar el patrón '{pattern_name}' en este proyecto.

        RESPONDE EXCLUSIVAMENTE EN FORMATO JSON con esta estructura EXACTA (no uses otros campos):
        {{
          \"pattern\": \"Nombre del patrón seleccionado\",
          \"suggested_max_lines\": 60,
          \"rules\": [
            {{ \"from\": \"src/entities/**\", \"to\": \"src/features/**\", \"reason\": \"Las entidades deben ser independientes\" }},
            {{ \"from\": \"src/shared/**\", \"to\": \"src/entities/**\", \"reason\": \"Shared no debe conocer entidades específicas\" }}
          ]
        }}

        IMPORTANTE:
        - Usa SOLO los campos: \"from\", \"to\", \"reason\" en cada regla
        - NO uses campos como \"name\", \"pattern\" (dentro de rules), \"severity\"
        - \"from\" y \"to\" deben ser patrones glob o rutas relativas al proyecto
        - Genera entre 5-10 reglas relevantes para el patrón {pattern_name}",
        pattern_name = pattern_name,
        framework = context.framework,
        deps = context.dependencies,
        files = context.folder_structure,
        key_files = context.key_files
    );

    let response_text = consultar_ia_con_fallback(prompt, &ai_configs).await?;
    let clean_json = extraer_json_flexible(&response_text)?;

    let suggestion: AISuggestionResponse = serde_json::from_str(&clean_json)?;
    Ok(suggestion)
}

/// Consulta la API de Claude (Anthropic)
pub async fn consultar_claude(prompt: String, ai_config: AIConfig) -> anyhow::Result<String> {
    let url = format!("{}/v1/messages", ai_config.api_url.trim_end_matches('/'));

    let client = reqwest::Client::new();
    let body = serde_json::json!({
        "model": ai_config.model,
        "max_tokens": 4096,
        "messages": [{
            "role": "user",
            "content": prompt
        }]
    });

    let response = client
        .post(&url)
        .header("x-api-key", &ai_config.api_key)
        .header("anthropic-version", "2023-06-01")
        .header("content-type", "application/json")
        .json(&body)
        .send()
        .await?;

    procesar_respuesta(response).await
}

/// Elimina bloques <thought>...</thought> de las respuestas de modelos como Gemma
fn strip_thought_blocks(text: &str) -> String {
    let mut result = String::new();
    let mut in_thought = false;
    let mut i = 0;
    let chars: Vec<char> = text.chars().collect();

    while i < chars.len() {
        let remaining = &chars[i..];
        if !in_thought && remaining.starts_with(&['<', 't', 'h', 'o', 'u', 'g', 'h', 't', '>']) {
            in_thought = true;
            i += 9;
        } else if in_thought && remaining.starts_with(&['<', '/', 't', 'h', 'o', 'u', 'g', 'h', 't', '>']) {
            in_thought = false;
            i += 10;
        } else if !in_thought {
            result.push(chars[i]);
            i += 1;
        } else {
            i += 1;
        }
    }

    result.trim().to_string()
}

/// Consulta la API de Gemini (Google)
pub async fn consultar_gemini(prompt: String, ai_config: AIConfig) -> anyhow::Result<String> {
    let url = format!(
        "{}/v1beta/models/{}:generateContent?key={}",
        ai_config.api_url.trim_end_matches('/'),
        ai_config.model,
        ai_config.api_key
    );

    let client = reqwest::Client::builder()
        .timeout(std::time::Duration::from_secs(120))
        .build()?;
        
    let body = serde_json::json!({
        "contents": [{
            "parts": [{
                "text": prompt
            }]
        }]
    });

    let response = client
        .post(&url)
        .header("x-goog-api-key", &ai_config.api_key)
        .header("content-type", "application/json")
        .json(&body)
        .send()
        .await?;

    let status = response.status();
    let response_text = response.text().await?;

    if !status.is_success() {
        return Err(anyhow::anyhow!(
            "Error Gemini ({}): {}",
            status,
            response_text
        ));
    }

    let json: serde_json::Value = serde_json::from_str(&response_text)?;
    
    // Extraer todas las partes que no sean "thoughts" (igual que en Sentinel)
    let empty_vec = vec![];
    let candidates = json["candidates"].as_array().ok_or_else(|| anyhow::anyhow!("No 'candidates' found"))?;
    if candidates.is_empty() {
        return Err(anyhow::anyhow!("Empty candidates from Gemini"));
    }
    
    let parts = candidates[0]["content"]["parts"].as_array().unwrap_or(&empty_vec);
    
    let mut full_text = String::new();
    for part in parts {
        // Si la parte tiene "thought": true, la ignoramos
        if part["thought"].as_bool().unwrap_or(false) {
            continue;
        }
        
        if let Some(part_text) = part["text"].as_str() {
            full_text.push_str(part_text);
        }
    }

    if full_text.is_empty() {
        // Fallback al comportamiento básico si no hay partes filtradas
        full_text = candidates[0]["content"]["parts"][0]["text"]
            .as_str()
            .unwrap_or("")
            .to_string();
    }

    if full_text.is_empty() {
        return Err(anyhow::anyhow!("Estructura de Gemini inesperada o vacía."));
    }

    // Limpiar bloques de pensamiento manuales (tags <thought>)
    Ok(strip_thought_blocks(&full_text))
}

/// Consulta Ollama (API nativa de Ollama)
pub async fn consultar_ollama(
    prompt: String,
    ai_config: AIConfig,
) -> anyhow::Result<String> {
    let url = format!(
        "{}/api/generate",
        ai_config.api_url.trim_end_matches('/')
    );

    let client = reqwest::Client::new();
    let body = serde_json::json!({
        "model": ai_config.model,
        "prompt": prompt,
        "system": "Eres un Arquitecto de Software Senior.",
        "stream": false
    });

    let response = client
        .post(&url)
        .header("content-type", "application/json")
        .json(&body)
        .send()
        .await?;

    let status = response.status();
    let response_text = response.text().await?;

    if !status.is_success() {
        return Err(anyhow::anyhow!(
            "Error Ollama ({}): {}",
            status,
            response_text
        ));
    }

    let json: serde_json::Value = serde_json::from_str(&response_text)?;
    let content = json["response"]
        .as_str()
        .ok_or_else(|| anyhow::anyhow!("No se pudo extraer texto de la respuesta de Ollama"))?;

    Ok(content.to_string())
}

/// Consulta APIs compatibles con OpenAI (OpenAI, Groq, Kimi)
pub async fn consultar_openai_compatible(
    prompt: String,
    ai_config: AIConfig,
) -> anyhow::Result<String> {
    let url = format!(
        "{}/chat/completions",
        ai_config.api_url.trim_end_matches('/')
    );

    let client = reqwest::Client::new();
    let body = serde_json::json!({
        "model": ai_config.model,
        "messages": [
            {"role": "system", "content": "Eres un Arquitecto de Software Senior."},
            {"role": "user", "content": prompt}
        ],
        "temperature": 0.1
    });

    let mut request = client.post(&url).header("content-type", "application/json");

    if !ai_config.api_key.is_empty() {
        request = request.header("authorization", format!("Bearer {}", ai_config.api_key));
    }

    let response = request.json(&body).send().await?;

    let status = response.status();
    let response_text = response.text().await?;

    if !status.is_success() {
        return Err(anyhow::anyhow!("Error API ({}): {}", status, response_text));
    }

    let json: serde_json::Value = serde_json::from_str(&response_text)?;
    let content = json["choices"][0]["message"]["content"]
        .as_str()
        .ok_or_else(|| anyhow::anyhow!("No se pudo extraer texto de la respuesta"))?;

    Ok(content.to_string())
}

async fn procesar_respuesta(response: reqwest::Response) -> anyhow::Result<String> {
    let status = response.status();
    let response_text = response.text().await?;

    if !status.is_success() {
        return Err(anyhow::anyhow!("Error API ({}): {}", status, response_text));
    }

    let json: serde_json::Value = serde_json::from_str(&response_text)?;

    // Claude format
    if let Some(content) = json["content"][0]["text"].as_str() {
        return Ok(content.to_string());
    }

    Ok(response_text)
}

/// Extrae un bloque JSON de una cadena de texto, manejando bloques de markdown y texto adicional.
pub fn extraer_json_flexible(text: &str) -> anyhow::Result<String> {
    // Si la IA respondió con un bloque de código markdown, intentamos extraer su contenido
    let content = if text.contains("```json") {
        text.split("```json")
            .nth(1)
            .and_then(|s| s.split("```").next())
            .unwrap_or(text)
            .trim()
    } else if text.contains("```") {
        text.split("```")
            .nth(1)
            .and_then(|s| s.split("```").next())
            .unwrap_or(text)
            .trim()
    } else {
        text.trim()
    };

    let start = content.find('{').ok_or_else(|| {
        anyhow::anyhow!(
            "No se encontró el inicio de un objeto JSON ('{{') en la respuesta.\n\nContenido recibido:\n{}",
            content
        )
    })?;

    let end = content.rfind('}').ok_or_else(|| {
        anyhow::anyhow!(
            "No se encontró el final de un objeto JSON ('}}') en la respuesta.\n\nContenido recibido:\n{}",
            content
        )
    })?;

    let json = &content[start..=end];

    // Limpiar caracteres especiales y espacios después del JSON
    let mut json_trimmed = json.trim_end().to_string();

    // Manejar escaped JSON string: la IA a veces devuelve el JSON como string escapado
    // Reemplazar secuencias de escape comunes
    json_trimmed = json_trimmed
        .replace("\\n", "\n")
        .replace("\\\"", "\"")
        .replace("\\t", "\t")
        .replace("\\\\", "\\");

    // Validación básica de completitud
    if !json_trimmed.ends_with('}') {
        return Err(anyhow::anyhow!(
            "El JSON parece estar truncado o incompleto."
        ));
    }

    // Intenta parsear para detectar errores antes de usarlo
    serde_json::from_str::<serde_json::Value>(&json_trimmed)
        .map_err(|e| {
            anyhow::anyhow!(
                "JSON inválido: {}\n\nJSON extraído:\n{}",
                e,
                json_trimmed
            )
        })?;

    Ok(json_trimmed)
}

pub mod suggestions;

pub mod pattern_detection;

pub mod recommendations;
