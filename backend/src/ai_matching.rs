use serde::{Deserialize, Serialize};
use tracing::{info, warn};
use std::env;

/// Enhanced match analysis with AI-generated explanations
#[derive(Debug, Serialize, Deserialize)]
pub struct EnhancedMatchAnalysis {
    /// Overall match score (0-100)
    pub match_score: f64,
    /// Human-readable explanation
    pub match_explanation: String,
    /// Key strengths (why it's a good match)
    pub strengths: Vec<String>,
    /// Areas for improvement
    pub improvement_areas: Vec<String>,
    /// Experience level alignment score (0-100)
    pub experience_alignment: f64,
    /// Track alignment score (0-100)
    pub track_alignment: f64,
    /// Skill overlap score (0-100)
    pub skill_overlap: f64,
}

/// Calculate enhanced match score using heuristic algorithm
pub fn calculate_enhanced_match(
    user_skills: &[String],
    job_skills: &[String],
    user_experience: Option<&str>,
    job_experience: &str,
    user_track: Option<&str>,
    job_title: &str,
) -> EnhancedMatchAnalysis {
    // 1. Skill Overlap (60% weight) - Most important factor
    let skill_overlap = calculate_skill_overlap(user_skills, job_skills);
    
    // 2. Experience Alignment (20% weight)
    let experience_alignment = calculate_experience_alignment(user_experience, job_experience);
    
    // 3. Track Alignment (20% weight)
    let track_alignment = calculate_track_alignment(user_track, job_title);
    
    // Weighted overall score (skill overlap has highest importance)
    let match_score = (skill_overlap * 0.6) + (experience_alignment * 0.2) + (track_alignment * 0.2);
    
    // Generate explanation
    let (explanation, strengths, improvements) = generate_match_explanation(
        user_skills,
        job_skills,
        user_experience,
        job_experience,
        user_track,
        job_title,
        skill_overlap,
        experience_alignment,
        track_alignment,
        match_score,
    );
    
    EnhancedMatchAnalysis {
        match_score,
        match_explanation: explanation,
        strengths,
        improvement_areas: improvements,
        experience_alignment,
        track_alignment,
        skill_overlap,
    }
}

/// Calculate skill overlap score (0-100)
fn calculate_skill_overlap(user_skills: &[String], job_skills: &[String]) -> f64 {
    if job_skills.is_empty() {
        return 100.0; // No skills required = perfect match
    }
    
    let user_skills_set: std::collections::HashSet<_> = 
        user_skills.iter().map(|s| s.to_lowercase()).collect();
    let job_skills_set: std::collections::HashSet<_> = 
        job_skills.iter().map(|s| s.to_lowercase()).collect();
    
    let matched = user_skills_set.intersection(&job_skills_set).count();
    let overlap = (matched as f64 / job_skills.len() as f64) * 100.0;
    
    // Bonus for having more skills than required
    let bonus = if user_skills.len() > job_skills.len() {
        ((user_skills.len() - job_skills.len()) as f64 / job_skills.len() as f64) * 10.0
    } else {
        0.0
    };
    
    (overlap + bonus).min(100.0)
}

/// Calculate experience level alignment (0-100)
fn calculate_experience_alignment(user_exp: Option<&str>, job_exp: &str) -> f64 {
    let user_exp = match user_exp {
        Some(exp) => exp.to_lowercase(),
        None => return 50.0, // Unknown = neutral
    };
    
    let job_exp = job_exp.to_lowercase();
    
    // Exact match
    if user_exp == job_exp {
        return 100.0;
    }
    
    // Experience hierarchy: fresher < junior < mid
    match (user_exp.as_str(), job_exp.as_str()) {
        ("fresher", "junior") | ("junior", "mid") => 80.0, // Close match
        ("fresher", "mid") => 60.0, // Somewhat aligned
        ("junior", "fresher") | ("mid", "junior") => 70.0, // Overqualified but okay
        ("mid", "fresher") => 40.0, // Overqualified
        _ => 50.0, // Unknown
    }
}

/// Calculate track alignment (0-100)
fn calculate_track_alignment(user_track: Option<&str>, job_title: &str) -> f64 {
    let user_track = match user_track {
        Some(track) => track.to_lowercase(),
        None => return 50.0, // Unknown = neutral
    };
    
    let title_lower = job_title.to_lowercase();
    
    // Check if job title matches user's track
    match user_track.as_str() {
        "web_development" => {
            if title_lower.contains("frontend") || title_lower.contains("backend") 
                || title_lower.contains("full stack") || title_lower.contains("web") 
                || title_lower.contains("react") || title_lower.contains("node") {
                100.0
            } else {
                50.0
            }
        }
        "data" => {
            if title_lower.contains("data") || title_lower.contains("analyst") 
                || title_lower.contains("scientist") || title_lower.contains("ml") 
                || title_lower.contains("machine learning") {
                100.0
            } else {
                50.0
            }
        }
        "design" => {
            if title_lower.contains("designer") || title_lower.contains("ui") 
                || title_lower.contains("ux") || title_lower.contains("graphic") {
                100.0
            } else {
                50.0
            }
        }
        "marketing" => {
            if title_lower.contains("marketing") || title_lower.contains("seo") 
                || title_lower.contains("content") || title_lower.contains("social") {
                100.0
            } else {
                50.0
            }
        }
        _ => 50.0,
    }
}

/// Generate human-readable match explanation
fn generate_match_explanation(
    user_skills: &[String],
    job_skills: &[String],
    user_experience: Option<&str>,
    job_experience: &str,
    _user_track: Option<&str>,
    _job_title: &str,
    _skill_overlap: f64,
    experience_alignment: f64,
    track_alignment: f64,
    match_score: f64,
) -> (String, Vec<String>, Vec<String>) {
    let user_skills_set: std::collections::HashSet<_> = 
        user_skills.iter().map(|s| s.to_lowercase()).collect();
    let job_skills_set: std::collections::HashSet<_> = 
        job_skills.iter().map(|s| s.to_lowercase()).collect();
    
    let matched_skills: Vec<String> = user_skills_set
        .intersection(&job_skills_set)
        .map(|s| s.to_string())
        .collect();
    
    let missing_skills: Vec<String> = job_skills_set
        .difference(&user_skills_set)
        .map(|s| s.to_string())
        .collect();
    
    let mut explanation_parts = Vec::new();
    let mut strengths = Vec::new();
    let mut improvements = Vec::new();
    
    // Skill analysis
    if !matched_skills.is_empty() {
        let skill_list = matched_skills.iter().take(5).map(|s| s.as_str()).collect::<Vec<_>>().join(", ");
        strengths.push(format!("Strong skills match: {}", skill_list));
        explanation_parts.push(format!(
            "You have {} of {} required skills ({})",
            matched_skills.len(),
            job_skills.len(),
            skill_list
        ));
    }
    
    if !missing_skills.is_empty() {
        let missing_list = missing_skills.iter().take(3).map(|s| s.as_str()).collect::<Vec<_>>().join(", ");
        improvements.push(format!("Learn: {}", missing_list));
        explanation_parts.push(format!("Consider learning: {}", missing_list));
    }
    
    // Experience alignment
    if experience_alignment >= 80.0 {
        strengths.push(format!("Experience level ({}) aligns well with this position", 
            user_experience.unwrap_or("your level")));
        explanation_parts.push("Your experience level is a good fit for this role".to_string());
    } else if experience_alignment < 60.0 {
        improvements.push(format!("This role requires {} experience, but you have {}", 
            job_experience, user_experience.unwrap_or("different")));
    }
    
    // Track alignment
    if track_alignment >= 80.0 {
        strengths.push("This role matches your preferred career track".to_string());
        explanation_parts.push("The position aligns with your career interests".to_string());
    }
    
    // Overall assessment
    let assessment = if match_score >= 80.0 {
        "Excellent match!"
    } else if match_score >= 60.0 {
        "Good match"
    } else if match_score >= 40.0 {
        "Moderate match"
    } else {
        "Limited match"
    };
    
    let full_explanation = format!("{} {}", assessment, explanation_parts.join(". "));
    
    (full_explanation, strengths, improvements)
}

pub async fn generate_ai_explanation_hf(
    user_skills: &[String],
    job_skills: &[String],
    user_experience: Option<&str>,
    job_experience: &str,
    user_track: Option<&str>,
    job_title: &str,
    job_description: &str,
    match_score: f64,
) -> Result<String, String> {
    let api_key = env::var("HUGGINGFACE_API_KEY").map_err(|_| "HUGGINGFACE_API_KEY not set")?;
    let model = "mistralai/Mistral-7B-Instruct-v0.2";
    let prompt = format!(
        r#"<s>[INST] You are a career advisor helping candidates understand job matches. Provide clear, actionable feedback.

Analyze the job match between a candidate and a job posting.

Candidate Profile:
- Skills: {}
- Experience Level: {}
- Preferred Track: {}

Job Requirements:
- Title: {}
- Required Skills: {}
- Experience Level: {}
- Description: {}

Match Score: {:.1}%

Provide a concise, professional explanation (2-3 sentences) explaining why this is a good match or what's missing. 
Focus on specific skills, experience alignment, and career track fit.
Format: Start with overall assessment, then mention key strengths, then areas for improvement. [/INST]"#,
        user_skills.join(", "),
        user_experience.unwrap_or("Not specified"),
        user_track.unwrap_or("Not specified"),
        job_title,
        job_skills.join(", "),
        job_experience,
        job_description.chars().take(200).collect::<String>(), // Truncate description
        match_score
    );
    
    let client = reqwest::Client::builder()
        .timeout(std::time::Duration::from_secs(30))
        .build()
        .map_err(|e| format!("Failed to create HTTP client: {}", e))?;
    
    info!("Calling Hugging Face API for job match explanation");
    let response = client
        .post(&format!("https://router.huggingface.co/hf-inference/models/{}", model))
        .header("Authorization", format!("Bearer {}", api_key))
        .header("Content-Type", "application/json")
        .json(&serde_json::json!({
            "inputs": prompt,
            "parameters": {
                "max_new_tokens": 150,
                "temperature": 0.7,
                "return_full_text": false
            }
        }))
        .send()
        .await
        .map_err(|e| format!("Failed to call Hugging Face API: {}", e))?;
    
    let status = response.status();
    if !status.is_success() {
        let error_text = response.text().await.unwrap_or_default();
        return Err(format!("Hugging Face API error ({}): {}", status, error_text));
    }
    
    let json: serde_json::Value = response
        .json()
        .await
        .map_err(|e| format!("Failed to parse Hugging Face response: {}", e))?;
    
    // Handle different response formats from Hugging Face
    let explanation = if let Some(error) = json.get("error") {
        // Model is loading or other error
        return Err(format!("Hugging Face API error: {}", error));
    } else if json.is_array() {
        // Array format: [{"generated_text": "..."}]
        json[0]["generated_text"]
            .as_str()
            .ok_or("Invalid response format: missing generated_text in array")?
            .trim()
            .to_string()
    } else if let Some(generated_text) = json.get("generated_text") {
        // Object format: {"generated_text": "..."}
        generated_text
            .as_str()
            .ok_or("Invalid response format: generated_text is not a string")?
            .trim()
            .to_string()
    } else {
        return Err("Invalid response format: no generated_text found".to_string());
    };
    
    // Clean up the explanation (remove prompt artifacts if any)
    let cleaned = explanation
        .replace("[INST]", "")
        .replace("[/INST]", "")
        .trim()
        .to_string();
    
    Ok(cleaned)
}

/// Generate AI explanation with fallback to heuristic
/// Tries Hugging Face API first, falls back to heuristic if it fails
pub async fn generate_ai_explanation(
    user_skills: &[String],
    job_skills: &[String],
    user_experience: Option<&str>,
    job_experience: &str,
    user_track: Option<&str>,
    job_title: &str,
    job_description: &str,
    match_score: f64,
    skill_overlap: f64,
    experience_alignment: f64,
    track_alignment: f64,
) -> String {
    // Try Hugging Face API first
    match generate_ai_explanation_hf(
        user_skills,
        job_skills,
        user_experience,
        job_experience,
        user_track,
        job_title,
        job_description,
        match_score,
    ).await {
        Ok(explanation) => {
            info!("Successfully generated AI explanation using Hugging Face");
            explanation
        }
        Err(e) => {
            warn!("Failed to generate AI explanation: {}. Falling back to heuristic.", e);
            // Fallback to heuristic explanation
            let (explanation, _, _) = generate_match_explanation(
                user_skills,
                job_skills,
                user_experience,
                job_experience,
                user_track,
                job_title,
                skill_overlap,
                experience_alignment,
                track_alignment,
                match_score,
            );
            explanation
        }
    }
}
    