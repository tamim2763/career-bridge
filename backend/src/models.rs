use serde::{Deserialize, Serialize};
use sqlx::FromRow;

#[derive(Debug, Serialize, Deserialize, sqlx::Type)]
#[sqlx(type_name = "experience_level")]
#[sqlx(rename_all = "lowercase")]
pub enum ExperienceLevel {
    Fresher,
    Junior,
    Mid,
}

#[derive(Debug, Serialize, Deserialize, sqlx::Type)]
#[sqlx(type_name = "career_track")]
#[sqlx(rename_all = "snake_case")]
pub enum CareerTrack {
    WebDevelopment,
    Data,
    Design,
    Marketing,
}

#[derive(Debug, Serialize, Deserialize, sqlx::Type)]
#[sqlx(type_name = "job_type")]
#[sqlx(rename_all = "snake_case")]
pub enum JobType {
    Internship,
    PartTime,
    FullTime,
    Freelance,
}

#[derive(Debug, Serialize, Deserialize, sqlx::Type)]
#[sqlx(type_name = "cost_indicator")]
#[sqlx(rename_all = "lowercase")]
pub enum CostIndicator {
    Free,
    Paid,
}

#[derive(Debug, FromRow, Serialize, Deserialize)]
pub struct User {
    pub id: i32,
    pub full_name: String,
    pub email: String,

    #[sqlx(rename = "education_level")]
    pub education: Option<String>,

    #[sqlx(rename = "experience_level")]
    pub experience_level: ExperienceLevel,

    #[sqlx(rename = "preferred_track")]
    pub preferred_track: CareerTrack,

    pub skills: Vec<String>,
    pub projects: Vec<String>,

    #[sqlx(rename = "target_roles")]
    pub target_roles: Vec<String>,

    #[sqlx(rename = "raw_cv_text")]
    pub raw_cv_text: Option<String>,

    #[serde(skip_serializing)]
    pub password_hash: String,
}

#[derive(Debug, FromRow, Serialize, Deserialize)]
pub struct Job {
    pub id: i32,
    #[sqlx(rename = "job_title")]
    pub job_title: String,
    pub company: String,
    pub location: String,

    #[sqlx(rename = "required_skills")]
    pub required_skills: Vec<String>,

    #[sqlx(rename = "experience_level")]
    pub experience_level: ExperienceLevel,

    #[sqlx(rename = "job_type")]
    pub job_type: JobType,
}

#[derive(Debug, FromRow, Serialize, Deserialize)]
pub struct LearningResource {
    pub id: i32,
    pub title: String,
    pub platform: String,
    pub url: String,

    #[sqlx(rename = "related_skills")]
    pub related_skills: Vec<String>,

    #[sqlx(rename = "cost")]
    pub cost: CostIndicator,
}
