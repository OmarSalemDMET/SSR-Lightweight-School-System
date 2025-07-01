#[derive(serde::Serialize)]
pub struct FetchTutorial {
    pub id: String,
    pub code: String,
    pub grade: i32,
    pub max_capacity: i32,
    pub overall_score: i32,
}

#[derive(serde::Deserialize)]
pub struct AddTutorial {
    pub code: String,
    pub grade: i32,
    pub max_capacity: i32,
    pub overall_score: i32,
}

#[derive(serde::Deserialize)]
pub struct StudentTutorialReq {
    pub id: Vec<String>,
    pub tutorial_id: String,
}
